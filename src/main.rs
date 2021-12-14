mod go_types;

use yew::prelude::*;

use crate::go_types::*;

#[function_component(App)]
fn app() -> Html {
    let board = use_state(Board::empty);

    html! {
        <div class="grid w-screen h-screen place-items-center">
            <h1 class="text-6xl text-center">{ "Go" }</h1>
            <div class="grid w-full max-w-xl gap-1 grid-cols-19">
                { for board.positions.iter().flatten().map(|&state| html! {
                        <BoardNode state={state}/>
                    })}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct BoardNodeProps {
    pub state: State,
}

#[function_component(BoardNode)]
fn board_node(props: &BoardNodeProps) -> Html {
    match &props.state {
        State::White => html! {
            <div class="w-6 h-6 bg-white rounded-full"></div>
        },
        State::Black => html! {
            <div class="w-6 h-6 bg-gray-500 rounded-full"></div>
        },
        State::Empty => html! {
            <div>{"+"}</div>
        },
    }
}

fn main() {
    yew::start_app::<App>();
}
