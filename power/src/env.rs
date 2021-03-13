use crate::*;

use power_env;

pub fn debug(string: &str) {
    unsafe {
        power_env::debug(string.len(), string.as_ptr());
    }
}

pub fn flush() {
    unsafe {
        power_env::flush();
    }
}

pub fn write<T1, T2>(key: &T1, value: &T2)
where
    T1: ?Sized + Serialize,
    T2: ?Sized + Serialize,
{
    let k = serialize(key);
    let v = serialize(value);

    unsafe {
        power_env::storage_write(k.len(), k.as_ptr(), v.len(), v.as_ptr());
    }
}

pub fn read<T1, T2>(key: &T1) -> Option<T2>
where
    T1: ?Sized + Serialize,
    T2: DeserializeOwned,
{
    let k = serialize(key);

    unsafe {
        let value_size = power_env::storage_value_size(k.len(), k.as_ptr());
        if value_size == 0 {
            return None;
        }
        let mut v: Vec<u8> = Vec::with_capacity(value_size);
        power_env::storage_read(k.len(), k.as_ptr(), value_size, v.as_mut_ptr());
        v.set_len(value_size);

        return deserialize(&v[..]);
    }
}

pub fn delete<T>(key: &T)
where
    T: ?Sized + Serialize,
{
    let k = serialize(key);

    unsafe {
        power_env::storage_write(k.len(), k.as_ptr(), 0, 0 as *const u8);
    }
}

pub fn has_key<T>(key: &T) -> bool
where
    T: ?Sized + Serialize,
{
    let k = serialize(key);

    unsafe {
        let value_size = power_env::storage_value_size(k.len(), k.as_ptr());
        return value_size > 0;
    }
}

pub fn reset() {
    unsafe {
        power_env::storage_reset();
    }
}

fn get_tx_raw() -> Vec<u8> {
    unsafe {
        let size = power_env::get_tx_raw_size();
        let mut dst: Vec<u8> = Vec::with_capacity(size);
        power_env::get_tx_raw(dst.as_mut_ptr());
        dst.set_len(size);
        return dst;
    }
}

pub fn get_tx() -> Option<Tx> {
    let raw = get_tx_raw();
    if raw.len() == 0 {
        return None;
    }
    let v: Value = deserialize(&raw[..]).unwrap();
    from_value(&v)
}

fn get_args_raw() -> Vec<u8> {
    unsafe {
        let size = power_env::get_args_raw_size();
        let mut dst: Vec<u8> = Vec::with_capacity(size);
        power_env::get_args_raw(dst.as_mut_ptr());
        dst.set_len(size);
        return dst;
    }
}

pub fn get_args<T>() -> T
where
    T: DeserializeOwned,
{
    let raw = get_args_raw();
    deserialize(&raw[..]).unwrap()
}

fn get_balance_raw() -> Vec<u8> {
    unsafe {
        let size = power_env::get_balance_raw_size();
        let mut dst: Vec<u8> = Vec::with_capacity(size);
        power_env::get_balance_raw(dst.as_mut_ptr());
        dst.set_len(size);
        return dst;
    }
}

pub fn get_balance() -> Vec<Amount> {
    let raw = get_balance_raw();
    if raw.len() == 0 {
        return vec![];
    }
    let v: Value = deserialize(&raw[..]).unwrap();

    if let Value::Map(balance) = v {
        let mut res = Vec::with_capacity(balance.len());
        for (k, v) in balance {
            res.push(Amount {
                currency: k.as_slice().unwrap().to_vec(),
                amount: v.as_u64().unwrap(),
            });
        }
        return res;
    }
    vec![]
}

pub fn set_return<T>(ret: T)
where
    T: Serialize,
{
    let r = serialize(ret);
    unsafe {
        power_env::set_return(r.len(), r.as_ptr());
    }
}

pub fn emit_tx(
    kind: TxKind,
    to: Option<Address>,
    payload: Option<Vec<PayloadItem>>,
    call: Option<(&str, Vec<Value>)>,
    extradata: Option<BTreeMap<&str, Value>>,
) {
    let mut v: Vec<(Value, Value)> = Vec::with_capacity(5);
    v.push(("k".into(), (kind as u64).into()));

    if to.is_some() {
        v.push(("to".into(), to.unwrap().into()));
    }
    if payload.is_some() {
        v.push(("p".into(), payload.unwrap().into()));
    }
    if call.is_some() {
        v.push(("c".into(), call.unwrap().into()));
    }
    if extradata.is_some() {
        v.push(("e".into(), extradata.unwrap().into()));
    }

    let enc = serialize(Value::Map(v));
    unsafe {
        power_env::emit_tx(enc.len(), enc.as_ptr());
    }
}

#[derive(Serialize)]
struct TxCall<'a, T> {
    k: u64,
    to: &'a Address,
    c: (&'a str, T),
}

pub fn emit_call<T>(to: &Address, method: &str, args: &T)
where
    T: ?Sized + Serialize,
{
    let tx = TxCall {
        k: TxKind::Generic as u64,
        to: to,
        c: (method, args),
    };
    let enc = serialize(tx);
    unsafe {
        power_env::emit_tx(enc.len(), enc.as_ptr());
    }
}
