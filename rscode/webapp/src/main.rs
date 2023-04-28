use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h2>{"Hello"}</h2>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
