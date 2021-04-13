use crate::*;

use crc::crc32;

use fixed_hash::construct_fixed_hash;

construct_fixed_hash! {
    pub struct Address(8);
}

impl From<Address> for Value {
    fn from(arg: Address) -> Self {
        Value::Binary(arg.as_bytes().to_vec())
    }
}

impl From<&Address> for Value {
    fn from(arg: &Address) -> Self {
        Value::Binary(arg.as_bytes().to_vec())
    }
}

impl FromValue for Address {
    fn from_value(arg: &Value) -> Option<Self> {
        if let Value::Binary(vec) = arg {
            return Some(Address::from_slice(&vec[..]));
        }
        if let Value::Array(vec) = arg {
            if vec.len() != 8 {
                return None;
            }
            let mut v: Vec<u8> = Vec::with_capacity(vec.len());
            for b in vec {
                v.push(b.as_u64().unwrap() as u8);
            }
            return Some(Address::from_slice(&v[..]));
        }
        None
    }
}

pub fn string_to_address(address: &str) -> Option<Address> {
    if address.len() == 20 {
        let g1 = ((address.as_bytes()[0] as u8) - ('A' as u8)) as u64;
        let g2 = ((address.as_bytes()[1] as u8) - ('A' as u8)) as u64;
        let gr = u64::from_str_radix(&address[2..4], 10).unwrap();
        let i = u64::from_str_radix(&address[4..18], 10).unwrap();
        let c = u8::from_str_radix(&address[18..20], 10).unwrap();

        let group = ((((g1 * 26 + g2) * 100) as u64) + gr) << 5;
        let block = (i >> 24) as u64;
        let wallet = (i & 0xffffff) as u64;

        let v = vec![
            ((group >> 16) & 0x1f | 0b10000000) as u8,
            ((group >> 8) & 0xff) as u8,
            ((group >> 0) & 0xff | ((block >> 16) & 0x1f)) as u8,
            ((block >> 8) & 0xff) as u8,
            ((block >> 0) & 0xff) as u8,
            ((wallet >> 16) & 0xff) as u8,
            ((wallet >> 8) & 0xff) as u8,
            ((wallet >> 0) & 0xff) as u8,
        ];

        let cc = (crc32::checksum_ieee(&v[..]) % 100) as u8;
        if c == cc {
            Some(Address::from_slice(&v[..]))
        } else {
            None
        }
    } else if address.len() == 18 {
        let a = u64::from_str_radix(&address[0..16], 16).unwrap();
        let c = u8::from_str_radix(&address[16..18], 16).unwrap();
        let v = vec![
            ((a >> 8 * 7) & 0xff | 0b10100000) as u8,
            ((a >> 8 * 6) & 0xff) as u8,
            ((a >> 8 * 5) & 0xff) as u8,
            ((a >> 8 * 4) & 0xff) as u8,
            ((a >> 8 * 3) & 0xff) as u8,
            ((a >> 8 * 2) & 0xff) as u8,
            ((a >> 8 * 1) & 0xff) as u8,
            ((a >> 8 * 0) & 0xff) as u8,
        ];

        let cc = (crc32::checksum_ieee(&v[..]) & 0xff) as u8;
        if c == cc {
            Some(Address::from_slice(&v[..]))
        } else {
            None
        }
    } else {
        None
    }
}

fn pad(base: u64, num: u64, size: u64) -> String {
    let alnum: &[u8] = &"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
    let b26: &[u8] = &"ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();

    let alph = if base == 26 { b26 } else { alnum };

    let mut s: Vec<u8> = Vec::with_capacity(32);
    let mut n = num;
    while n >= base {
        s.insert(0, alph[(n % base) as usize]);
        n /= base;
    }
    s.insert(0, alph[n as usize]);
    while (s.len() as u64) < size {
        s.insert(0, alph[0]);
    }
    String::from_utf8_lossy(&s[..]).to_string()
}

pub fn address_to_string(address: &Address) -> String {
    let wallet =
        ((address[7] as u64) | ((address[6] as u64) << 8) | ((address[5] as u64) << 16)) as u64;
    let checksum = crc32::checksum_ieee(address.as_bytes()) as u64;
    if address[0] & 0x20 > 0 {
        let block = ((((address[4] as u64)
            | (address[3] as u64) << 8
            | (address[2] as u64) << 16
            | (address[1] as u64) << 24)
            >> 0)
            + ((0x1f as u64 & (address[0] as u64)) * (4294967296 as u64)))
            as u64;
        return pad(16, block, 10) + &pad(16, wallet, 6) + &pad(16, checksum % 256, 2);
    } else {
        let block = ((address[4] as u64)
            | ((address[3] as u64) << 8)
            | ((0x1f & (address[2] as u64)) << 16)) as u64;
        let group = (((address[2] as u64)
            | ((address[1] as u64) << 8)
            | ((0x1f & (address[0] as u64)) << 16))
            >> 5) as u64;
        let int_part = (wallet + (block * 16777216)) as u64;
        return pad(26, group / 100, 2)
            + &pad(10, group % 100, 2)
            + &pad(10, int_part, 14)
            + &pad(10, checksum % 100, 2);
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TxKind {
    Generic = 16,
    Register,
    Deploy,
    Patch,
    Block,
    TStore,
    LStore,
    Notify
}

impl From<TxKind> for Value {
    fn from(arg: TxKind) -> Self {
        Value::Integer((arg as u64).into())
    }
}
impl FromValue for TxKind {
    fn from_value(arg: &Value) -> Option<Self> {
        if arg.is_u64() {
            match arg.as_u64().unwrap() {
                16 => Some(TxKind::Generic),
                17 => Some(TxKind::Register),
                18 => Some(TxKind::Deploy),
                19 => Some(TxKind::Patch),
                20 => Some(TxKind::Block),
                21 => Some(TxKind::TStore),
                22 => Some(TxKind::LStore),
                23 => Some(TxKind::Notify),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Purpose {
    Transfer = 0,
    SrcFee,
    DstFee,
    Gas,
}

impl From<Purpose> for Value {
    fn from(arg: Purpose) -> Self {
        Value::Integer((arg as u64).into())
    }
}

impl FromValue for Purpose {
    fn from_value(arg: &Value) -> Option<Self> {
        if arg.is_u64() {
            match arg.as_u64().unwrap() {
                0 => Some(Purpose::Transfer),
                1 => Some(Purpose::SrcFee),
                2 => Some(Purpose::DstFee),
                3 => Some(Purpose::Gas),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Amount {
    pub currency: Vec<u8>,
    pub amount: u64,
}

#[derive(Debug)]
pub struct PayloadItem {
    pub purpose: Purpose,
    pub amount: Amount,
}

impl From<PayloadItem> for Value {
    fn from(arg: PayloadItem) -> Self {
        let mut a = Vec::new();
        a.push(arg.purpose.into());
        a.push(Value::Binary(arg.amount.currency));
        a.push(arg.amount.amount.into());
        Value::Array(a)
    }
}
impl From<&PayloadItem> for Value {
    fn from(arg: &PayloadItem) -> Self {
        let mut a = Vec::new();
        a.push(arg.purpose.into());
        a.push(Value::Binary(arg.amount.currency.clone()));
        a.push(arg.amount.amount.into());
        Value::Array(a)
    }
}

impl FromValue for PayloadItem {
    fn from_value(arg: &Value) -> Option<Self> {
        let p: Purpose = from_value(&arg[0]).unwrap();
        let c1: Option<Vec<u8>> = from_value(&arg[1]);
        let c2: Option<String> = from_value(&arg[1]);
        let c: Vec<u8>;
        if c1.is_some() {
            c = c1.unwrap();
        } else if c2.is_some() {
            c = c2.unwrap().as_bytes().to_vec();
        } else {
            return None;
        }
        let a: u64 = from_value(&arg[2]).unwrap();
        Some(PayloadItem {
            purpose: p,
            amount: Amount {
                currency: c,
                amount: a,
            },
        })
    }
}

#[derive(Debug)]
pub struct Tx {
    pub kind: TxKind,
    pub from: Address,
    pub to: Option<Address>,
    pub payload: Vec<PayloadItem>,
    pub timestamp: u64,
    //pub not_before: u64,
    pub extradata: BTreeMap<String, Value>,
}

impl FromValue for Tx {
    fn from_value(arg: &Value) -> Option<Self> {
        let om: Option<BTreeMap<String, Value>> = from_value(arg);
        if om.is_some() {
            let m = om.unwrap();
            let kind: TxKind = from_value(m.get("k").unwrap()).unwrap();
            let from: Address = from_value(m.get("f").unwrap()).unwrap();

            let to: Option<Address> = if m.get("to").is_some() {
                from_value(m.get("to").unwrap())
            } else {
                None
            };

            let payload: Vec<PayloadItem> = if m.get("p").is_some() {
                from_value(m.get("p").unwrap()).unwrap()
            } else {
                vec![]
            };

            let timestamp: u64 = from_value(m.get("t").unwrap()).unwrap_or(0);
            //let not_before: u64 = from_value(m.get("nb").unwrap_or(&Value::Integer(From::from(0)))).unwrap_or(0);
            let extradata: BTreeMap<String, Value> = if m.get("e").is_some() {
                from_value(m.get("e").unwrap()).unwrap()
            } else {
                BTreeMap::new()
            };
            Some(Tx {
                kind: kind,
                from: from,
                to: to,
                payload: payload,
                timestamp: timestamp,
                //not_before: not_before,
                extradata: extradata,
            })
        } else {
            None
        }
    }
}
