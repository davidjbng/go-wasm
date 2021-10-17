use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1 class="pt-2 text-6xl text-center">{ "Go" }</h1>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
