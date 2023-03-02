use crate::mycomponent::MyComponent;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct WindowResizeArgs {
    height: f64,
}

#[derive(Serialize, Deserialize)]
struct SearchArgs {
    search: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FileListArgs {
    files: Vec<String>,
}

#[function_component(App)]
pub fn app() -> Html {
    let main_body_ref = use_node_ref();
    let search_input_ref = use_node_ref();
    let height = use_state(|| 50.0);
    let files: UseStateHandle<Vec<String>> = use_state(|| vec![]);

    {
        let files = files.clone();
        let main_body_ref = main_body_ref.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let element = main_body_ref.cast::<web_sys::HtmlInputElement>().unwrap();
                    let height_value = f64::from(element.client_height());
                    invoke(
                        "resize_window",
                        to_value(&WindowResizeArgs {
                            height: height_value,
                        })
                        .unwrap(),
                    )
                    .await;
                });
            },
            files,
        );
    }

    let update_height = {
        let main_body_ref = main_body_ref.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let element = main_body_ref.cast::<web_sys::HtmlInputElement>().unwrap();
            let height_value = f64::from(element.client_height());
            height.set(height_value);
        })
    };

    let submit_search = {
        let search_input_ref = search_input_ref.clone();
        let files = files.clone();

        Callback::from(move |e: InputEvent| {
            e.prevent_default();
            let search_input_ref = search_input_ref.clone();
            let files = files.clone();

            spawn_local(async move {
                let new_search_term = search_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value();

                let result = invoke(
                    "get_apps",
                    to_value(&SearchArgs {
                        search: new_search_term,
                    })
                    .unwrap(),
                )
                .await;
                let new_files: Vec<String> = from_value(result).unwrap();
                files.set(new_files);
            })
        })
    };

    html! {
        <main >
            <div ref={main_body_ref}>
                <div data-tauri-drag-region="true"   class="search-container">
                    <button onclick={update_height}>
                        <svg data-tauri-drag-region="true"    class="search-icon" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z" />
                        </svg>
                    </button>
                    <form>
                        <input class="input" oninput={submit_search} placeholder="Spotlight Search" ref={search_input_ref} />
                    </form>
                </div>
                <div class="results">
                    {files
                       .iter()
                       .enumerate()
                       .map(|(i, file)| {
                           html! {<li  key={i}>
                                    <button class="app-result">
                                        { file }
                                    </button>
                                  </li>}
                       })
                       .collect::<Html>()}
                </div>
            </div>
        </main>
    }
}
