mod graphviz;

use std::str::FromStr;

use graphviz::Graphviz;
use leptos::{
    component,  create_local_resource, create_signal, event_target_value, view, IntoView, SignalGet, SignalGetUntracked, SignalSet, Suspense
};
use leptos_use::{use_timeout_fn, UseTimeoutFnReturn};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys::{Array, JsString}, Blob};

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

    let (typed_val, typed_set) = create_signal(String::from(DEFAULT));
    let (options_open_val, options_open_set) = create_signal(false);

    let output = move || {
        let Some(graphviz) = graphviz.get() else {
            return None;
        };
        let true = doteur_core::contains_sql_tables(&typed_val.get()) else {
            return None;
        };
        let dot = doteur_core::process_data(
            &typed_val.get(),
            None,
            show_legend_val.get(),
            render_in_dark_mode_val.get(),
        );
        Some(graphviz.dot(&dot))
    };

    let UseTimeoutFnReturn { start, is_pending, .. } = use_timeout_fn(
        |()| {},
        5000.0
    );
    
    let gen_download = move || {
        if let Some(output) = output() {
            let js_str = JsString::from_str(&output).unwrap();
            let array = Array::from(&js_str);
            let b = Blob::new_with_str_sequence(&array).unwrap();
            let url = web_sys::Url::create_object_url_with_blob(&b).unwrap();
            Some(url)
        } else {
            None
        }
    };

    view! {
        <nav class="navbar bg-base-100">
            <div class="flex-1">
                <p class="text-xl">doteur</p>
            </div>
            <div class="flex-none">
                <ul class="menu menu-horizontal px-1">
                <li>
                    <a title="Download" href=gen_download download="export.svg"  class="btn btn-ghost btn-circle" on:click=move |_| start(())>
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 20 20" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" />
                        <path d="M10.75 2.75a.75.75 0 0 0-1.5 0v8.614L6.295 8.235a.75.75 0 1 0-1.09 1.03l4.25 4.5a.75.75 0 0 0 1.09 0l4.25-4.5a.75.75 0 0 0-1.09-1.03l-2.955 3.129V2.75Z" />
                        <path d="M3.5 12.75a.75.75 0 0 0-1.5 0v2.5A2.75 2.75 0 0 0 4.75 18h10.5A2.75 2.75 0 0 0 18 15.25v-2.5a.75.75 0 0 0-1.5 0v2.5c0 .69-.56 1.25-1.25 1.25H4.75c-.69 0-1.25-.56-1.25-1.25v-2.5Z" />
                        </svg>
                    </a>
                </li>
                <li>
                    <button title="Options" on:click=move|_|options_open_set.set(true) class="btn btn-ghost btn-circle">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 20 20" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" />
                        <path fill-rule="evenodd" d="M8.34 1.804A1 1 0 0 1 9.32 1h1.36a1 1 0 0 1 .98.804l.295 1.473c.497.144.971.342 1.416.587l1.25-.834a1 1 0 0 1 1.262.125l.962.962a1 1 0 0 1 .125 1.262l-.834 1.25c.245.445.443.919.587 1.416l1.473.294a1 1 0 0 1 .804.98v1.361a1 1 0 0 1-.804.98l-1.473.295a6.95 6.95 0 0 1-.587 1.416l.834 1.25a1 1 0 0 1-.125 1.262l-.962.962a1 1 0 0 1-1.262.125l-1.25-.834a6.953 6.953 0 0 1-1.416.587l-.294 1.473a1 1 0 0 1-.98.804H9.32a1 1 0 0 1-.98-.804l-.295-1.473a6.957 6.957 0 0 1-1.416-.587l-1.25.834a1 1 0 0 1-1.262-.125l-.962-.962a1 1 0 0 1-.125-1.262l.834-1.25a6.957 6.957 0 0 1-.587-1.416l-1.473-.294A1 1 0 0 1 1 10.68V9.32a1 1 0 0 1 .804-.98l1.473-.295c.144-.497.342-.971.587-1.416l-.834-1.25a1 1 0 0 1 .125-1.262l.962-.962A1 1 0 0 1 5.38 3.03l1.25.834a6.957 6.957 0 0 1 1.416-.587l.294-1.473ZM13 10a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" clip-rule="evenodd" />
                        </svg>
                    </button>
                </li>              
                </ul>
            </div>
        </nav>
        <main class="flex flex-row max-h-full grow overflow-x-auto">
            <div class="h-full w-1/2 ">
                <textarea placeholder="Type here ..." class="textarea  w-full h-full resize-none focus:ring-0 focus:border-transparent focus:outline-none rounded-none"  prop:value=move || typed_val.get() on:input=move |ev| typed_set.set(event_target_value(&ev))>{typed_val.get_untracked()}</textarea>
            </div>
            <Suspense
                fallback=move || view! { <p>"Loading..."</p> }
            >
            <div inner_html=output class="w-1/2 h-full max-w-1/2 glass overflow-x-auto"></div>
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
                <input type="checkbox" class="theme-controller" value=move|| if is_dark_mode.get() { "light" } else { "dark"} checked:is_dark_mode />
                    <div class="swap-off" class:swap-off=is_light_mode class:swap-on=is_dark_mode >OFF</div>
                    <div class="swap-on" class:swap-off=is_dark_mode class:swap-on=is_light_mode >ON</div>
              </label>
            </label>
            <label class="label cursor-pointer">
              <span class="label-text">Dark mode rendering</span>
              <input type="checkbox" checked:render_in_dark_mode_val class="checkbox" on:click=move|_|render_in_dark_mode_set.set(!render_in_dark_mode_val.get()) />
            </label>
            <label class="label cursor-pointer">
                <span class="label-text">Show legend</span>
                <input type="checkbox" checked:show_legend_val class="checkbox" on:click=move|_|show_legend_set.set(!show_legend_val.get())  />
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
