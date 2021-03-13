use crate::*;

use corepack::to_bytes;
use serde::ser::{SerializeMap, SerializeSeq, Serializer};
pub use serde::Serialize;

pub fn serialize<T: Serialize>(value: T) -> Vec<u8> {
    to_bytes(value).unwrap()
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(self.as_bytes())
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Value::Nil => return serializer.serialize_none(),
            Value::Boolean(b) => return serializer.serialize_bool(b),
            Value::Integer(i) => {
                if i.is_u64() {
                    return serializer.serialize_u64(i.as_u64().unwrap());
                } else {
                    return serializer.serialize_i64(i.as_i64().unwrap());
                }
            }
            Value::String(ref val) => return serializer.serialize_str(val.as_str().unwrap()),
            Value::Array(ref vec) => {
                let mut seq = serializer.serialize_seq(Some(vec.len())).unwrap();
                for e in vec.iter() {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
            Value::Map(ref vec) => {
                let mut map = serializer.serialize_map(Some(vec.len()))?;
                for (k, v) in vec {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
            Value::Binary(ref vec) => serializer.serialize_bytes(vec.as_slice()),
            _ => serializer.serialize_none(),
        }
    }
}
