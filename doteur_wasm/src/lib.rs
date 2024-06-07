mod graphviz;

use graphviz::Graphviz;
use leptos::{
    component, create_local_resource, create_signal, event_target_value, view, IntoView, SignalGet,
    SignalGetUntracked, SignalSet, Suspense,
};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

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

    view! {
        <nav class="navbar bg-base-100">
            <div class="flex-1">
                <p class="text-xl">doteur</p>
            </div>
            <div class="flex-none">
                <ul class="menu menu-horizontal px-1">
                <li><a on:click=move|_|options_open_set.set(true)>"Show options"</a></li>
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
    }
}
