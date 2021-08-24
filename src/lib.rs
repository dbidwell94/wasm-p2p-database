use chrono::{DateTime, Utc};
use js_sys::Date;
use wasm_bindgen::prelude::*;
use web_sys::{window};

#[wasm_bindgen]
pub struct Database {
    database_name: String,
    chrono_date: DateTime<Utc>,
}

#[wasm_bindgen]
impl Database {
    pub fn new(name: &str) -> Database {
        
        Database {
            database_name: name.to_owned(),
            chrono_date: Utc::now(),
        }
    }

    #[wasm_bindgen(js_name = getName)]
    pub fn get_name(&self) -> String {
        self.database_name.to_owned()
    }

    #[wasm_bindgen(js_name = getCreatedDate)]
    pub fn get_created_date(&self) -> Date {
        Date::new(&JsValue::from_str(
            &self.chrono_date.naive_utc().to_string(),
        ))
    }
}
