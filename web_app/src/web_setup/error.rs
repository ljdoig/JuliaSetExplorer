use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub enum EngineError {
    IO(std::io::Error),
    Js(JsValue),
}

impl From<JsValue> for EngineError {
    fn from(e: JsValue) -> Self {
        EngineError::Js(e)
    }
}

impl From<EngineError> for JsValue {
    fn from(e: EngineError) -> Self {
        match e {
            EngineError::Js(e) => e,
            EngineError::IO(e) => JsValue::from_str(&e.to_string()),
        }
    }
}
