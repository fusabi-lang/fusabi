//! Async operations for Fusabi VM (Tokio-backed)
//!
//! This module provides standard library functions for real async computation expressions
//! backed by Tokio runtime.

#[cfg(feature = "async")]
use crate::async_types::{AsyncState, AsyncValue, TaskId};
use crate::{Value, Vm, VmError};

// Synchronous free-monad based async implementations
// These are used for computation expression desugaring

/// Async.Return : 'a -> Async<'a>
/// Creates an async computation that returns a value
pub fn async_return(_vm: &mut Vm, args: &[Value]) -> Result<Value, VmError> {
    if args.len() != 1 {
        return Err(VmError::Runtime(format!(
            "Async.Return expects 1 argument, got {}",
            args.len()
        )));
    }

    let value = args[0].clone();

    Ok(Value::Variant {
        type_name: "Async".to_string(),
        variant_name: "Pure".to_string(),
        fields: vec![value],
    })
}

/// Async.Bind : Async<'a> -> ('a -> Async<'b>) -> Async<'b>
pub fn async_bind(_vm: &mut Vm, args: &[Value]) -> Result<Value, VmError> {
    if args.len() != 2 {
        return Err(VmError::Runtime(
            "Async.Bind expects 2 arguments".to_string(),
        ));
    }

    let computation = args[0].clone();
    let binder = args[1].clone();

    Ok(Value::Variant {
        type_name: "Async".to_string(),
        variant_name: "Bind".to_string(),
        fields: vec![computation, binder],
    })
}

/// Async.Delay : (unit -> Async<'a>) -> Async<'a>
pub fn async_delay(_vm: &mut Vm, args: &[Value]) -> Result<Value, VmError> {
    if args.len() != 1 {
        return Err(VmError::Runtime(
            "Async.Delay expects 1 argument".to_string(),
        ));
    }

    let generator = args[0].clone();

    Ok(Value::Variant {
        type_name: "Async".to_string(),
        variant_name: "Delay".to_string(),
        fields: vec![generator],
    })
}

/// Async.ReturnFrom : Async<'a> -> Async<'a>
pub fn async_return_from(_vm: &mut Vm, args: &[Value]) -> Result<Value, VmError> {
    if args.len() != 1 {
        return Err(VmError::Runtime(
            "Async.ReturnFrom expects 1 argument".to_string(),
        ));
    }
    Ok(args[0].clone())
}

/// Async.Zero : unit -> Async<unit>
pub fn async_zero(_vm: &mut Vm, _args: &[Value]) -> Result<Value, VmError> {
    Ok(Value::Variant {
        type_name: "Async".to_string(),
        variant_name: "Pure".to_string(),
        fields: vec![Value::Unit],
    })
}

/// Async.Combine : Async<unit> -> Async<'a> -> Async<'a>
pub fn async_combine(_vm: &mut Vm, args: &[Value]) -> Result<Value, VmError> {
    if args.len() != 2 {
        return Err(VmError::Runtime(
            "Async.Combine expects 2 arguments".to_string(),
        ));
    }

    let first = args[0].clone();
    let second = args[1].clone();

    Ok(Value::Variant {
        type_name: "Async".to_string(),
        variant_name: "Combine".to_string(),
        fields: vec![first, second],
    })
}

/// Async.RunSynchronously : Async<'a> -> 'a
pub fn async_run_synchronously(vm: &mut Vm, args: &[Value]) -> Result<Value, VmError> {
    if args.len() != 1 {
        return Err(VmError::Runtime(
            "Async.RunSynchronously expects 1 argument".to_string(),
        ));
    }

    let mut current = args[0].clone();
    let mut continuations: Vec<Value> = Vec::new();

    loop {
        match current {
            Value::Variant {
                variant_name,
                fields,
                ..
            } => match variant_name.as_str() {
                "Pure" => {
                    let result = fields[0].clone();
                    if let Some(continuation) = continuations.pop() {
                        current = vm.call_value(continuation, &[result])?;
                    } else {
                        return Ok(result);
                    }
                }
                "Bind" => {
                    let computation = fields[0].clone();
                    let binder = fields[1].clone();
                    continuations.push(binder);
                    current = computation;
                }
                "Combine" => {
                    let first = fields[0].clone();
                    let second = fields[1].clone();

                    let helper = Value::NativeFn {
                        name: "Async.Internal.CombineHelper".to_string(),
                        arity: 2,
                        args: vec![second],
                    };

                    current = Value::Variant {
                        type_name: "Async".to_string(),
                        variant_name: "Bind".to_string(),
                        fields: vec![first, helper],
                    };
                }
                "Delay" => {
                    let generator = fields[0].clone();
                    current = vm.call_value(generator, &[Value::Unit])?;
                }
                _ => {
                    return Err(VmError::Runtime(format!(
                        "Unknown Async variant: {}",
                        variant_name
                    )))
                }
            },
            _ => {
                return Err(VmError::TypeMismatch {
                    expected: "Async variant",
                    got: current.type_name(),
                })
            }
        }
    }
}

/// Helper function for Combine
pub fn async_combine_helper(_vm: &mut Vm, args: &[Value]) -> Result<Value, VmError> {
    if args.len() != 2 {
        return Err(VmError::Runtime(
            "CombineHelper expects 2 arguments".to_string(),
        ));
    }
    Ok(args[0].clone())
}

// Tokio-backed async implementations (when async feature is enabled)
#[cfg(feature = "async")]
pub mod tokio_async {
    use super::*;

    /// Register all Tokio-backed async operations with the VM
    pub fn register_tokio_async_ops(vm: &mut Vm) -> Result<(), VmError> {
        // Enable async runtime
        vm.enable_async()?;

        let registry = vm.host_registry.clone();
        let mut reg = registry.lock().unwrap();

        // Async.sleep: int -> Async<unit>
        reg.register("Async.sleep", |vm: &mut Vm, args: &[Value]| {
            if args.len() != 1 {
                return Err(VmError::Runtime(format!(
                    "Async.sleep expects 1 argument, got {}",
                    args.len()
                )));
            }

            let millis = args[0].as_int().ok_or_else(|| {
                VmError::Runtime("Async.sleep expects int argument".to_string())
            })?;

            let task_id = vm.exec_async(move || {
                std::thread::sleep(std::time::Duration::from_millis(millis as u64));
                Ok(Value::Unit)
            })?;

            Ok(Value::Async(AsyncValue::Task(task_id)))
        });

        // Async.parallel: List<Async<'a>> -> Async<List<'a>>
        reg.register("Async.parallel", |vm: &mut Vm, args: &[Value]| {
            if args.len() != 1 {
                return Err(VmError::Runtime(format!(
                    "Async.parallel expects 1 argument, got {}",
                    args.len()
                )));
            }

            let tasks = args[0].list_to_vec().ok_or_else(|| {
                VmError::Runtime("Async.parallel expects list argument".to_string())
            })?;

            let task_ids: Result<Vec<TaskId>, VmError> = tasks
                .iter()
                .map(|v| match v {
                    Value::Async(AsyncValue::Task(id)) => Ok(*id),
                    _ => Err(VmError::Runtime(
                        "Async.parallel expects list of Async values".to_string(),
                    )),
                })
                .collect();

            let task_ids = task_ids?;
            let runtime_clone = vm.async_runtime.as_ref().unwrap().clone();

            let task_id = vm.exec_async(move || {
                let mut results = Vec::new();
                for id in task_ids {
                    let result = runtime_clone.block_on(id)?;
                    results.push(result);
                }
                Ok(Value::vec_to_cons(results))
            })?;

            Ok(Value::Async(AsyncValue::Task(task_id)))
        });

        // Async.withTimeout: int -> Async<'a> -> Async<Option<'a>>
        reg.register("Async.withTimeout", |vm: &mut Vm, args: &[Value]| {
            if args.len() != 2 {
                return Err(VmError::Runtime(format!(
                    "Async.withTimeout expects 2 arguments, got {}",
                    args.len()
                )));
            }

            let timeout_ms = args[0].as_int().ok_or_else(|| {
                VmError::Runtime("Async.withTimeout expects int as first argument".to_string())
            })?;

            let task_id = match &args[1] {
                Value::Async(AsyncValue::Task(id)) => *id,
                _ => {
                    return Err(VmError::Runtime(
                        "Async.withTimeout expects Async value as second argument".to_string(),
                    ))
                }
            };

            let runtime_clone = vm.async_runtime.as_ref().unwrap().clone();
            let timeout_task_id = vm.exec_async(move || {
                let start = std::time::Instant::now();
                loop {
                    let state = runtime_clone.poll(task_id);
                    match state {
                        AsyncState::Pending => {
                            if start.elapsed().as_millis() > timeout_ms as u128 {
                                let _ = runtime_clone.cancel(task_id);
                                return Ok(Value::Variant {
                                    type_name: "Option".to_string(),
                                    variant_name: "None".to_string(),
                                    fields: vec![],
                                });
                            }
                            std::thread::sleep(std::time::Duration::from_millis(1));
                        }
                        AsyncState::Ready(v) => {
                            return Ok(Value::Variant {
                                type_name: "Option".to_string(),
                                variant_name: "Some".to_string(),
                                fields: vec![v],
                            });
                        }
                        AsyncState::Failed(e) => {
                            return Err(VmError::Runtime(e));
                        }
                        AsyncState::Cancelled => {
                            return Ok(Value::Variant {
                                type_name: "Option".to_string(),
                                variant_name: "None".to_string(),
                                fields: vec![],
                            });
                        }
                    }
                }
            })?;

            Ok(Value::Async(AsyncValue::Task(timeout_task_id)))
        });

        // Async.catch: Async<'a> -> Async<Result<'a, string>>
        reg.register("Async.catch", |vm: &mut Vm, args: &[Value]| {
            if args.len() != 1 {
                return Err(VmError::Runtime(format!(
                    "Async.catch expects 1 argument, got {}",
                    args.len()
                )));
            }

            let task_id = match &args[0] {
                Value::Async(AsyncValue::Task(id)) => *id,
                _ => {
                    return Err(VmError::Runtime(
                        "Async.catch expects Async value".to_string(),
                    ))
                }
            };

            let runtime_clone = vm.async_runtime.as_ref().unwrap().clone();
            let catch_task_id = vm.exec_async(move || {
                match runtime_clone.block_on(task_id) {
                    Ok(v) => Ok(Value::Variant {
                        type_name: "Result".to_string(),
                        variant_name: "Ok".to_string(),
                        fields: vec![v],
                    }),
                    Err(e) => Ok(Value::Variant {
                        type_name: "Result".to_string(),
                        variant_name: "Error".to_string(),
                        fields: vec![Value::Str(format!("{}", e))],
                    }),
                }
            })?;

            Ok(Value::Async(AsyncValue::Task(catch_task_id)))
        });

        // Async.cancel: Async<'a> -> unit
        reg.register("Async.cancel", |vm: &mut Vm, args: &[Value]| {
            if args.len() != 1 {
                return Err(VmError::Runtime(format!(
                    "Async.cancel expects 1 argument, got {}",
                    args.len()
                )));
            }

            match &args[0] {
                Value::Async(AsyncValue::Task(task_id)) => {
                    vm.cancel_async(*task_id)?;
                    Ok(Value::Unit)
                }
                _ => Err(VmError::Runtime(
                    "Async.cancel expects Async value".to_string(),
                )),
            }
        });

        Ok(())
    }
}
