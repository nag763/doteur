// Copyright ⓒ 2021-2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/doteur/blob/main/LICENCE.MD).
#![allow(clippy::unused_unit)]
use doteur_core::process_data;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert_sql_to_dot(data: &str, legend: bool, dark_mode: bool) -> String {
    wasm_logger::init(wasm_logger::Config::default());
    process_data(data, None, legend, dark_mode)
}
