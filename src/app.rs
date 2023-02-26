use crate::mycomponent::MyComponent;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
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
struct GreetArgs {
    height: f64,
}
#[function_component(App)]
pub fn app() -> Html {
    let main_body_ref = use_node_ref();

    let height = use_state(|| 0.0);

    let height2 = height.clone();
    let height3 = height.clone();
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                invoke(
                    "resize_window",
                    to_value(&GreetArgs { height: *height2 }).unwrap(),
                )
                .await;
            });
        },
        height3,
    );

    let update_height = {
        let main_body_ref = main_body_ref.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let element = main_body_ref.cast::<web_sys::HtmlInputElement>().unwrap();
            let height_value = f64::from(element.client_height());
            height.set(height_value);
        })
    };

    html! {
            <main >
        <div ref={main_body_ref}>
                    <div data-tauri-drag-region="true"   class="search-container">

                    <button class="invisible-button" onclick={update_height}>
                        <svg data-tauri-drag-region="true"    class="search-icon" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z" />
                        </svg>
                    </button>

                    <input class="input" placeholder="Spotlight Search"/>
                    </div>
                    <MyComponent />
                    <div>
                        <p>{"testing"}</p>
                        <p>{"testing"}</p>
                        <p>{"testing"}</p>
                        <p>{"testing"}</p>
                        <p>{"testing"}</p>
                        <p>{"testing"}</p>
                        <p>{"testing"}</p>
                        <p>{"testing"}</p>
                    </div>
    </div>
                // <div class="row">
                //     <a href="https://tauri.app" target="_blank">
                //         <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                //     </a>
                //     <a href="https://yew.rs" target="_blank">
                //         <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                //     </a>
                // </div>

                // <p>{"Click on the Tauri and Yew logos to learn more."}</p>

                // <p>
                //     {"Recommended IDE setup: "}
                //     <a href="https://code.visualstudio.com/" target="_blank">{"VS Code"}</a>
                //     {" + "}
                //     <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">{"Tauri"}</a>
                //     {" + "}
                //     <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">{"rust-analyzer"}</a>
                // </p>

                // <form class="row" onsubmit={greet}>
                //     <input id="greet-input" ref={greet_input_ref} placeholder="Enter a name..." />
                //     <button type="submit">{"Greet"}</button>
                // </form>

                // <p><b>{ &*name }</b></p>
        </main>
    }
}
