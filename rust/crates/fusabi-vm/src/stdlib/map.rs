// Fusabi Map Standard Library
use crate::value::Value;
use crate::vm::VmError;
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

pub fn map_empty(_unit: &Value) -> Result<Value, VmError> {
    Ok(Value::Map(Arc::new(Mutex::new(HashMap::new()))))
}

pub fn map_add(key: &Value, value: &Value, map: &Value) -> Result<Value, VmError> {
    let key_str = key.as_str().ok_or_else(|| VmError::TypeMismatch {
        expected: "string",
        got: key.type_name(),
    })?;
    match map {
        Value::Map(m) => {
            let mut new_map = m.lock().unwrap().clone();
            new_map.insert(key_str.to_string(), value.clone());
            Ok(Value::Map(Arc::new(Mutex::new(new_map))))
        }
        _ => Err(VmError::TypeMismatch {
            expected: "map",
            got: map.type_name(),
        }),
    }
}

pub fn map_remove(key: &Value, map: &Value) -> Result<Value, VmError> {
    let key_str = key.as_str().ok_or_else(|| VmError::TypeMismatch {
        expected: "string",
        got: key.type_name(),
    })?;
    match map {
        Value::Map(m) => {
            let mut new_map = m.lock().unwrap().clone();
            new_map.remove(key_str);
            Ok(Value::Map(Arc::new(Mutex::new(new_map))))
        }
        _ => Err(VmError::TypeMismatch {
            expected: "map",
            got: map.type_name(),
        }),
    }
}

pub fn map_find(key: &Value, map: &Value) -> Result<Value, VmError> {
    let key_str = key.as_str().ok_or_else(|| VmError::TypeMismatch {
        expected: "string",
        got: key.type_name(),
    })?;
    match map {
        Value::Map(m) => {
            let m = m.lock().unwrap();
            m.get(key_str).cloned().ok_or_else(|| {
                VmError::Runtime(format!("Map key not found: {}", key_str))
            })
        }
        _ => Err(VmError::TypeMismatch {
            expected: "map",
            got: map.type_name(),
        }),
    }
}

pub fn map_try_find(key: &Value, map: &Value) -> Result<Value, VmError> {
    let key_str = key.as_str().ok_or_else(|| VmError::TypeMismatch {
        expected: "string",
        got: key.type_name(),
    })?;
    match map {
        Value::Map(m) => {
            let m = m.lock().unwrap();
            match m.get(key_str) {
                Some(value) => Ok(Value::Variant {
                    type_name: "Option".to_string(),
                    variant_name: "Some".to_string(),
                    fields: vec![value.clone()],
                }),
                None => Ok(Value::Variant {
                    type_name: "Option".to_string(),
                    variant_name: "None".to_string(),
                    fields: vec![],
                }),
            }
        }
        _ => Err(VmError::TypeMismatch {
            expected: "map",
            got: map.type_name(),
        }),
    }
}

pub fn map_contains_key(key: &Value, map: &Value) -> Result<Value, VmError> {
    let key_str = key.as_str().ok_or_else(|| VmError::TypeMismatch {
        expected: "string",
        got: key.type_name(),
    })?;
    match map {
        Value::Map(m) => {
            let m = m.lock().unwrap();
            Ok(Value::Bool(m.contains_key(key_str)))
        }
        _ => Err(VmError::TypeMismatch {
            expected: "map",
            got: map.type_name(),
        }),
    }
}

pub fn map_is_empty(map: &Value) -> Result<Value, VmError> {
    match map {
        Value::Map(m) => Ok(Value::Bool(m.lock().unwrap().is_empty())),
        _ => Err(VmError::TypeMismatch {
            expected: "map",
            got: map.type_name(),
        }),
    }
}

pub fn map_count(map: &Value) -> Result<Value, VmError> {
    match map {
        Value::Map(m) => Ok(Value::Int(m.lock().unwrap().len() as i64)),
        _ => Err(VmError::TypeMismatch {
            expected: "map",
            got: map.type_name(),
        }),
    }
}

pub fn map_of_list(list: &Value) -> Result<Value, VmError> {
    let mut map = HashMap::new();
    let mut current = list.clone();
    loop {
        match current {
            Value::Nil => break,
            Value::Cons { head, tail } => {
                if let Value::Tuple(elements) = &*head {
                    if elements.len() != 2 {
                        return Err(VmError::Runtime(
                            "Map.ofList expects list of 2-tuples".to_string(),
                        ));
                    }
                    let key_str = elements[0].as_str().ok_or_else(|| VmError::TypeMismatch {
                        expected: "string",
                        got: elements[0].type_name(),
                    })?;
                    map.insert(key_str.to_string(), elements[1].clone());
                } else {
                    return Err(VmError::Runtime(
                        "Map.ofList expects list of tuples".to_string(),
                    ));
                }
                current = (*tail).clone();
            }
            _ => {
                return Err(VmError::TypeMismatch {
                    expected: "list",
                    got: current.type_name(),
                })
            }
        }
    }
    Ok(Value::Map(Arc::new(Mutex::new(map))))
}

pub fn map_to_list(map: &Value) -> Result<Value, VmError> {
    match map {
        Value::Map(m) => {
            let m = m.lock().unwrap();
            let mut entries: Vec<_> = m
                .iter()
                .map(|(k, v)| Value::Tuple(vec![Value::Str(k.clone()), v.clone()]))
                .collect();
            entries.sort_by(|a, b| {
                if let (Value::Tuple(a_tuple), Value::Tuple(b_tuple)) = (a, b) {
                    if let (Some(Value::Str(a_key)), Some(Value::Str(b_key))) =
                        (a_tuple.get(0), b_tuple.get(0))
                    {
                        return a_key.cmp(b_key);
                    }
                }
                std::cmp::Ordering::Equal
            });
            Ok(Value::vec_to_cons(entries))
        }
        _ => Err(VmError::TypeMismatch {
            expected: "map",
            got: map.type_name(),
        }),
    }
}
