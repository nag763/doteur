mod codemirror;
mod graphviz;

use std::str::FromStr;

use codemirror::{CodeMirror, CodeMirrorOptions, Position};
use graphviz::Graphviz;
use leptos::{
    component, create_effect, create_local_resource, create_signal, document, event_target_checked,
    view, window_event_listener, IntoView, SignalGet, SignalSet, Suspense,
};
use leptos_use::{use_timeout_fn, UseTimeoutFnReturn};
use wasm_bindgen::{closure::Closure, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    js_sys::{Array, Function, JsString},
    Blob,
};

const DEFAULT: &str = "CREATE TABLE HELLO (world INT PRIMARY KEY);";

#[component]
pub fn app() -> impl IntoView {
    let graphviz = create_local_resource(
        || (),
        move |_| async move {
            let promise = Graphviz::load();
            let future = JsFuture::from(promise);
            let result = future.await.unwrap();
            result.dyn_into::<Graphviz>().unwrap()
        },
    );

    let is_dark_mode = leptos_use::use_preferred_dark();
    let is_light_mode = move || !is_dark_mode.get();

    let (render_in_dark_mode_val, render_in_dark_mode_set) = create_signal(false);
    let (show_legend_val, show_legend_set) = create_signal(false);

    let (output_val, output_set) = create_signal(None);
    let (options_open_val, options_open_set) = create_signal(false);

    let cm = create_local_resource(
        || (),
        move |_| async move {
            let el = document().get_element_by_id("sql_source");
            let options = CodeMirrorOptions::default();
            options.set_line_numbers(true);
            options.set_mode("sql");
            let Some(cm) = CodeMirror::from_text_area(el, options) else {
                panic!("Code mirror can't be initialized");
            };
            cm.focus();
            cm.set_size("100%", "100%");
            let position = Position::default();
            position.set_character(DEFAULT.len());
            position.set_line(0);
            cm.set_value(DEFAULT);
            cm.set_cursor(&position);
            cm
        },
    );

    let cm_on_val_change = move || {
        if let (Some(cm), Some(graphviz)) = (cm.get(), graphviz.get()) {
            let cm_val = cm.get_value();
            if !doteur_core::contains_sql_tables(&cm_val) {
                output_set.set(None);
            };
            let dot = doteur_core::process_data(
                &cm_val,
                None,
                show_legend_val.get(),
                render_in_dark_mode_val.get(),
            );
            output_set.set(Some(graphviz.dot(&dot)));
        }
    };

    let UseTimeoutFnReturn {
        start, is_pending, ..
    } = use_timeout_fn(|()| {}, 5000.0);

    let gen_download = move || {
        if let Some(output) = output_val.get() {
            let js_str = JsString::from_str(&output).unwrap();
            let array = Array::from(&js_str);
            let b = Blob::new_with_str_sequence(&array).unwrap();
            let url = web_sys::Url::create_object_url_with_blob(&b).unwrap();
            Some(url)
        } else {
            None
        }
    };

    let compute_cm_theme = move |e| {
        let theme = if event_target_checked(&e) == is_dark_mode.get() {
            "normal"
        } else {
            "material"
        };
        if let Some(cm) = cm.get() {
            cm.set_option("theme", theme.into());
        }
    };

    let _handle = window_event_listener(leptos::ev::keydown, move |ev| {
        if ev.key_code() == 27 {
            options_open_set.set(false);
        }
    });

    // Once loaded
    create_effect(move |_| {
        if let Some(cm) = cm.get() {
            cm_on_val_change();
            if is_dark_mode.get() {
                cm.set_option("theme", "material".into());
                render_in_dark_mode_set.set(true);
            }
            let closure: Box<dyn FnMut()> = Box::new(cm_on_val_change);
            let closure = Closure::wrap(closure);
            cm.on(
                "change",
                &closure.into_js_value().unchecked_into::<Function>(),
            );
        }
    });

    view! {
        <nav class="navbar bg-base-100">
            <div class="flex-1">
                <p class="text-xl">doteur</p>
            </div>
            <div class="flex-none">
                <ul class="menu menu-horizontal px-1">
                <li >
                    <a title="Documentation" href="https://nag763.github.io/doteur"  class="btn btn-ghost btn-circle">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="h-5 w-5">
                        <path fill-rule="evenodd" d="M18 10a8 8 0 1 1-16 0 8 8 0 0 1 16 0ZM8.94 6.94a.75.75 0 1 1-1.061-1.061 3 3 0 1 1 2.871 5.026v.345a.75.75 0 0 1-1.5 0v-.5c0-.72.57-1.172 1.081-1.287A1.5 1.5 0 1 0 8.94 6.94ZM10 15a1 1 0 1 0 0-2 1 1 0 0 0 0 2Z" clip-rule="evenodd" />
                        </svg>
                    </a>
                </li>
                <li>
                    <a title="Download" href=gen_download download="export.svg"  class="btn btn-ghost btn-circle" on:click=move |_| if output_val.get().is_some() { start(()) } >
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="size-5 h-5">
                    <path d="M10.75 2.75a.75.75 0 0 0-1.5 0v8.614L6.295 8.235a.75.75 0 1 0-1.09 1.03l4.25 4.5a.75.75 0 0 0 1.09 0l4.25-4.5a.75.75 0 0 0-1.09-1.03l-2.955 3.129V2.75Z" />
                    <path d="M3.5 12.75a.75.75 0 0 0-1.5 0v2.5A2.75 2.75 0 0 0 4.75 18h10.5A2.75 2.75 0 0 0 18 15.25v-2.5a.75.75 0 0 0-1.5 0v2.5c0 .69-.56 1.25-1.25 1.25H4.75c-.69 0-1.25-.56-1.25-1.25v-2.5Z" />
                  </svg>

                    </a>
                </li>
                <li>
                    <button title="Options" on:click=move|_|options_open_set.set(true) class="btn btn-ghost btn-circle">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="size-5 h-5">
                        <path d="M10 3.75a2 2 0 1 0-4 0 2 2 0 0 0 4 0ZM17.25 4.5a.75.75 0 0 0 0-1.5h-5.5a.75.75 0 0 0 0 1.5h5.5ZM5 3.75a.75.75 0 0 1-.75.75h-1.5a.75.75 0 0 1 0-1.5h1.5a.75.75 0 0 1 .75.75ZM4.25 17a.75.75 0 0 0 0-1.5h-1.5a.75.75 0 0 0 0 1.5h1.5ZM17.25 17a.75.75 0 0 0 0-1.5h-5.5a.75.75 0 0 0 0 1.5h5.5ZM9 10a.75.75 0 0 1-.75.75h-5.5a.75.75 0 0 1 0-1.5h5.5A.75.75 0 0 1 9 10ZM17.25 10.75a.75.75 0 0 0 0-1.5h-1.5a.75.75 0 0 0 0 1.5h1.5ZM14 10a2 2 0 1 0-4 0 2 2 0 0 0 4 0ZM10 16.25a2 2 0 1 0-4 0 2 2 0 0 0 4 0Z" />
                    </svg>

                    </button>
                </li>
                </ul>
            </div>
        </nav>
        <main class="flex flex-row max-h-full grow overflow-x-auto">
            <div class="h-full w-1/2 ">
                <textarea placeholder="Type here ..." id="sql_source" class="textarea  w-full h-full resize-none focus:ring-0 focus:border-transparent focus:outline-none rounded-none"  ></textarea>
            </div>
            <Suspense
                fallback=move || view! { <p>"Loading..."</p> }
            >
            <div inner_html=output_val class="w-1/2 h-full max-w-1/2 glass overflow-x-auto"></div>
            </Suspense>
        </main>
        <footer class="footer footer-center p-4 bg-base-200 text-base-content">
         <aside>
            <p>"Copyright © 2021-2024 LABEYE Loïc"</p>
          </aside>
        </footer>
        <dialog class="modal" class:modal-open=options_open_val>
            <div class="modal-box">
            <h3 class="font-bold text-lg mb-2">Options panel</h3>
            <div class="form-control">
            <label class="label cursor-pointer">
                <span class="label-text">Dark mode</span>
                <label class="swap">
                <input type="checkbox" class="theme-controller" on:change=compute_cm_theme value=move|| if is_dark_mode.get() {  "light" } else { "dark"} />
                    <div class:swap-off=is_light_mode class:swap-on=is_dark_mode >OFF</div>
                    <div class:swap-off=is_dark_mode class:swap-on=is_light_mode >ON</div>
              </label>
            </label>
            <label class="label cursor-pointer">
              <span class="label-text">Dark mode rendering</span>
              <input type="checkbox" checked=render_in_dark_mode_val class="checkbox" on:click=move|_|render_in_dark_mode_set.set(!render_in_dark_mode_val.get()) />
            </label>
            <label class="label cursor-pointer">
                <span class="label-text">Show legend</span>
                <input type="checkbox" checked=show_legend_val class="checkbox" on:click=move|_|show_legend_set.set(!show_legend_val.get())  />
            </label>
          </div>
            <div class="modal-action">
                <form method="dialog ">
                <a class="btn"  on:click=move|_|options_open_set.set(false)>Close</a>
                </form>
            </div>
            </div>
        </dialog>
        {move || is_pending.get().then(|| view! {
                <div class="toast animate-fade-up animate-twice animate-duration-[2500ms] animate-ease-out animate-alternate">
                    <div role="alert" class="alert alert-success">
                        <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                        <span>Your download has started</span>
                    </div>
                </div>
            })
        }

    }
}
