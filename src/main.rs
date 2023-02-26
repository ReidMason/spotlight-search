mod app;
mod mycomponent;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
