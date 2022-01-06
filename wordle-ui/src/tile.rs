use wasm_bindgen::JsCast;
use yew::prelude::*;
use web_sys::{EventTarget, HtmlInputElement};
use serde::{Deserialize, Serialize};

pub struct Tile;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum TileState {
    Correct,
    Absent,
    Present,
}

impl TileState {
    pub fn toggle(self) -> Self {
        match self {
            TileState::Present => TileState::Correct,
            TileState::Correct => TileState::Absent,
            TileState::Absent => TileState::Present,
        }
    }
}

pub enum TileMsg {
    Toggle,
    Change(String),
}

impl ToString for TileState {
    fn to_string(&self) -> String {
        match self {
            TileState::Correct => String::from("correct"),
            TileState::Absent => String::from("absent"),
            TileState::Present => String::from("present"),
        }
    }
}

#[derive(Properties)]
pub struct TileProps {
    pub state: TileState,
    pub entry: String,
    pub on_toggle: Callback<()>,
    pub on_change: Callback<String>,
}

impl PartialEq for TileProps {
    fn eq(&self, other: &Self) -> bool {
        self.state.eq(&other.state) && self.entry.eq(&other.entry)
    }
}

impl Component for Tile {
    type Message = TileMsg;
    type Properties = TileProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TileMsg::Toggle => {
                (ctx.props().on_toggle).emit(());
                false
            },
            TileMsg::Change(entry) => {
                (ctx.props().on_change).emit(entry);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let on_value_change = link.batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();

            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| TileMsg::Change(input.value()))
        });

        let max_length = if ctx.props().state == TileState::Correct {
            1
        } else {
            10
        };

        html! {
            <div>
                <input
                    class="tile"
                    data-state={ ctx.props().state.to_string() }
                    value={ ctx.props().entry.to_string() }
                    onchange={ on_value_change }
                    maxlength={ max_length.to_string() }
                />
                <button class="toggle" onclick={link.callback(|_| TileMsg::Toggle)}>{ "Toggle" }</button>
            </div>
        }
    }
}
