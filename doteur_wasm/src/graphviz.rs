use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::js_sys::Promise;

#[wasm_bindgen(module = "https://cdn.jsdelivr.net/npm/@hpcc-js/wasm@2.16.2/dist/graphviz.js/+esm")]
extern "C" {

    #[wasm_bindgen(js_name = "Graphviz")]
    #[derive(Clone)]
    pub type Graphviz;

    #[wasm_bindgen(static_method_of = Graphviz)]
    pub fn load() -> Promise;

    #[wasm_bindgen(method)]
    pub fn dot(this: &Graphviz, param: &str) -> String;
}
