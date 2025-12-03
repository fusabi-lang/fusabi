//! Script module for dynamic code evaluation
//!
//! Provides functions for evaluating Fusabi code at runtime.

use crate::value::Value;
use crate::vm::{Vm, VmError};

/// Script.eval : string -> Result<Value, string>
/// Evaluates Fusabi code and returns the result
/// Returns Ok(Value) on success, Error(message) on failure
///
/// Note: This function requires the fusabi-frontend crate for compilation.
/// Since we're in the VM crate, we can't directly compile code here.
/// The actual implementation will need to be provided by the host environment.
pub fn script_eval(_vm: &mut Vm, args: &[Value]) -> Result<Value, VmError> {
    if args.is_empty() {
        return Err(VmError::Runtime(
            "Script.eval requires 1 argument".to_string(),
        ));
    }

    let _code = match &args[0] {
        Value::Str(s) => s.clone(),
        other => {
            return Err(VmError::TypeMismatch {
                expected: "string",
                got: other.type_name(),
            })
        }
    };

    // Since we're in the VM crate and don't have access to the compiler,
    // we return an error indicating that dynamic evaluation is not available
    // in this context. This would need to be overridden by a higher-level
    // implementation that has access to the fusabi-frontend crate.
    Err(VmError::Runtime(
        "Script.eval is not available in this context. \
         Dynamic code evaluation requires the fusabi-frontend crate. \
         Use the Engine API from the fusabi crate instead."
            .to_string(),
    ))
}

/// Script.evalToString : string -> string
/// Evaluates code and returns result as string, or error message prefixed with "Error: "
pub fn script_eval_to_string(vm: &mut Vm, args: &[Value]) -> Result<Value, VmError> {
    if args.is_empty() {
        return Err(VmError::Runtime(
            "Script.evalToString requires 1 argument".to_string(),
        ));
    }

    // Try to evaluate using script_eval
    match script_eval(vm, args) {
        Ok(result) => {
            // Convert result to string representation
            Ok(Value::Str(format!("{}", result)))
        }
        Err(e) => {
            // Return error message as string
            Ok(Value::Str(format!("Error: {}", e)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_eval_requires_argument() {
        let mut vm = Vm::new();
        let result = script_eval(&mut vm, &[]);
        assert!(result.is_err());
        if let Err(VmError::Runtime(msg)) = result {
            assert!(msg.contains("requires 1 argument"));
        }
    }

    #[test]
    fn test_script_eval_requires_string() {
        let mut vm = Vm::new();
        let result = script_eval(&mut vm, &[Value::Int(42)]);
        assert!(result.is_err());
        if let Err(VmError::TypeMismatch { expected, got }) = result {
            assert_eq!(expected, "string");
            assert_eq!(got, "int");
        }
    }

    #[test]
    fn test_script_eval_returns_not_available_error() {
        let mut vm = Vm::new();
        let result = script_eval(&mut vm, &[Value::Str("1 + 1".to_string())]);
        assert!(result.is_err());
        if let Err(VmError::Runtime(msg)) = result {
            assert!(msg.contains("not available in this context"));
        }
    }

    #[test]
    fn test_script_eval_to_string_error_handling() {
        let mut vm = Vm::new();
        let result = script_eval_to_string(&mut vm, &[Value::Str("1 + 1".to_string())]);
        assert!(result.is_ok());
        if let Ok(Value::Str(msg)) = result {
            assert!(msg.starts_with("Error:"));
        }
    }
}
