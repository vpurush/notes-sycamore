use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/js_service/notebook_service.js")]
extern "C" {
    pub async fn getAllNotebooks() -> JsValue;
    pub fn getTemp() -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
