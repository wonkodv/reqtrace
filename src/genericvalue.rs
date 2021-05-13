use std::{collections::HashMap, convert::TryFrom};

use crate::errors::Error;
use Error::*;


#[derive(Debug)]
pub enum Value {
    Null,
    Bool(bool),
    Number(i64),
    String(String),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
}

impl TryFrom<serde_json::Value> for Value {
    type Error = Error;

    fn try_from(v: serde_json::Value) -> Result<Self, Self::Error> {
        let r = match v {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Bool(b),
            serde_json::Value::Number(n) => {
                if let Some(n) = n.as_i64() {
                    Value::Number(n)
                } else {
                    return Err(ConfigError(format!("Not a i64: {}", n)));
                }
            }
            serde_json::Value::String(s) => Value::String(s),
            serde_json::Value::Array(vec) => {
                let mut r = Vec::with_capacity(vec.len());
                for v in vec {
                    let v = Value::try_from(v)?;
                    r.push(v);
                }
                Value::List(r)
            }
            serde_json::Value::Object(map) => {
                let mut r = HashMap::new();
                for (k, v) in map {
                    let v = Value::try_from(v)?;
                    let old = r.insert(k, v);
                    if old.is_some() {
                        unreachable!();
                    }
                }
                Value::Map(r)
            }
        };
        Ok(r)
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryInto;

    use super::*;
    use Value::*;

    #[test]
    fn test_fron_json_value() {
        let o = serde_json::json! {
            {
                "key" : ["list", 1, true, [] ]
            }
        };
        let v: Value = o.try_into().unwrap();

        if let Map(m) = v {
            if let List(l) = &m["key"] {
                if let String(s) = &l[0] {
                    assert!(s == "list");
                    return;
                }
            }
        }
        panic!();
    }
}
