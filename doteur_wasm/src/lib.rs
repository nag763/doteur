mod graphviz;

use graphviz::Graphviz;
use leptos::{component, create_local_resource, create_signal, event_target_value, view, IntoView, SignalGet, SignalGetUntracked, SignalSet, Suspense};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

const DEFAULT : &str = "CREATE TABLE HELLO (world INT PRIMARY KEY);";

#[component]
pub fn app() -> impl IntoView {
    let graphviz = create_local_resource(|| (), move |_| async move   {
        let promise = Graphviz::load();
        let future = JsFuture::from(promise);
        let result = future.await.unwrap();
        result.dyn_into::<Graphviz>().unwrap()
    });

    let (typed_val, typed_set) = create_signal(String::from(DEFAULT));
    
    let output = move || {
        let Some(graphviz) = graphviz.get() else {
            return None;
        };
        let true = doteur_core::contains_sql_tables(&typed_val.get()) else {
            return None;
        };
        let dot = doteur_core::process_data(&typed_val.get(), None, false, true);
        Some(graphviz.dot(&dot))
    };
    
    view! {
        <nav class="navbar bg-base-100">
            <div class="flex-1">
                <p class="text-xl">doteur</p>
            </div>
            <div class="flex-none">
                <ul class="menu menu-horizontal px-1">
                <li><a>Link</a></li>
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

    }
}