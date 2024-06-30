use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::{
    js_sys::{Array, Function, Object},
    Element,
};

#[wasm_bindgen(module = "https://cdn.jsdelivr.net/npm/@codemirror/lang-sql@6.7.0/+esm")]
extern "C" {




    #[wasm_bindgen]
    pub fn sql() -> LanguageSupport;

}

#[wasm_bindgen(module = "https://cdn.jsdelivr.net/npm/codemirror@6.65.7/+esm")]
extern "C" {


    #[wasm_bindgen(js_name = "EditorView")]
    pub type EditorView;

    #[wasm_bindgen(constructor ,js_class = "EditorView")]
    pub fn new() -> EditorView;


    #[wasm_bindgen(js_name = "LanguageSupport")]
    pub type LanguageSupport;

    #[derive(Clone)]
    #[wasm_bindgen(js_name = "default")]
    pub type CodeMirror;

    #[wasm_bindgen(extends = Object)]
    #[derive(Default)]
    pub type CodeMirrorOptions;

    #[wasm_bindgen(extends = Object)]
    #[derive(Default)]
    pub type Position;

    #[wasm_bindgen(method, setter = lineNumbers)]
    pub fn set_line_numbers(this: &CodeMirrorOptions, line_numbers: bool);

    #[wasm_bindgen(method, setter = mode)]
    pub fn set_mode(this: &CodeMirrorOptions, mode: &str);

    #[wasm_bindgen(method, setter = theme)]
    pub fn set_theme(this: &CodeMirrorOptions, theme: &str);

    #[wasm_bindgen(method, setter = extensions)]
    pub fn set_extensions(this: &CodeMirrorOptions, extensions: Array);

    #[wasm_bindgen(method, setter = line)]
    pub fn set_line(this: &Position, line: u32);

    #[wasm_bindgen(method, setter = character)]
    pub fn set_character(this: &Position, character: usize);

    #[wasm_bindgen(static_method_of=CodeMirror, js_name =fromTextArea ,js_class = "default")]
    pub fn from_text_area(el: Option<Element>, options: CodeMirrorOptions) -> Option<CodeMirror>;

    #[wasm_bindgen(method)]
    pub fn focus(this: &CodeMirror);

    #[wasm_bindgen(method, js_name=setOption)]
    pub fn set_option(this: &CodeMirror, option_name: &str, val: JsValue);

    #[wasm_bindgen(method, js_name=setCursor)]
    pub fn set_cursor(this: &CodeMirror, cursor: &Position);

    #[wasm_bindgen(method, js_name=setSize)]
    pub fn set_size(this: &CodeMirror, height: &str, width: &str);

    #[wasm_bindgen(method, js_name=on)]
    pub fn on(this: &CodeMirror, event: &str, event_triggered: &Function);

    #[wasm_bindgen(method, js_name=getValue)]
    pub fn get_value(this: &CodeMirror) -> String;
}
