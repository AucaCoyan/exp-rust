use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{"hola world"}</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}