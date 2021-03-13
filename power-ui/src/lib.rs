#![no_std]
#![feature(alloc)]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{format, vec};

use serde::ser::{SerializeMap, SerializeTuple, Serializer};
use serde::Serialize;

extern "C" {
    #[link_name = "time"]
    fn unsafe_time() -> i32;
}

pub fn time() -> i32 {
    unsafe {
        return unsafe_time();
    }
}

pub struct Form {
    body: Option<String>,
    defaults: Option<String>,
    actions: Vec<FormAction>,
}

struct XForm<'a>(&'a Form);

#[derive(Serialize)]
struct FormAction {
    action: String,
    caption: String,
    #[serde(rename = "noValidate")]
    no_validate: bool,
}

impl Form {
    pub fn new() -> Form {
        Form {
            body: None,
            defaults: None,
            actions: vec![],
        }
    }

    pub fn with_body(body: &str) -> Form {
        let mut form = Form::new();
        form.body(body);
        form
    }

    pub fn with_rho(rho: &str) -> Form {
        let mut form = Form::new();
        form.rho(rho);
        form
    }

    pub fn body(&mut self, body: &str) -> &mut Self {
        self.body = Some(body.to_string());
        self
    }

    pub fn rho(&mut self, rho: &str) -> &mut Self {
        self.body = Some(format!(
            r#"{{ "type": "roTextNode", "text": "{rho}" }}"#,
            rho = rho.to_string().replace("\n", "\\n")
        ));
        self
    }

    pub fn defaults(&mut self, defaults: &str) -> &mut Self {
        self.defaults = Some(defaults.to_string());
        self
    }

    pub fn action(&mut self, caption: &str, action: &str, validate: bool) -> &mut Self {
        self.actions.push(FormAction {
            action: action.to_string(),
            caption: caption.to_string(),
            no_validate: !validate,
        });
        self
    }
}

impl Serialize for Form {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(2)?;
        seq.serialize_element("form")?;
        seq.serialize_element(&XForm(self))?;
        seq.end()
    }
}

impl<'a> Serialize for XForm<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut c = 0;
        if self.0.body.is_some() {
            c += 1;
        }
        if self.0.defaults.is_some() {
            c += 1;
        }
        if self.0.actions.len() > 0 {
            c += 1;
        }
        let mut map = serializer.serialize_map(Some(c))?;
        if self.0.body.is_some() {
            map.serialize_entry("form", self.0.body.as_ref().unwrap())?;
        }
        if self.0.defaults.is_some() {
            map.serialize_entry("default_values", self.0.defaults.as_ref().unwrap())?;
        }
        if self.0.actions.len() > 0 {
            map.serialize_entry("actions", &self.0.actions)?;
        }
        map.end()
    }
}
