use crate::model::TileMode;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

pub struct Tile;

pub enum TileMsg {
    Toggle,
    Change(Option<char>),
}

#[derive(Debug, Properties)]
pub struct TileProps {
    pub state: TileMode,
    pub _entry: Option<char>,
    pub active: bool,
    pub on_toggle: Callback<()>,
    pub on_change: Callback<Option<char>>,
}

impl PartialEq for TileProps {
    fn eq(&self, other: &Self) -> bool {
        self.state.eq(&other.state)
            && self._entry.eq(&other._entry)
            && self.active.eq(&other.active)
    }
}

impl Component for Tile {
    type Message = TileMsg;
    type Properties = TileProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        if !ctx.props().active {
            return false;
        }

        match msg {
            TileMsg::Toggle => {
                (ctx.props().on_toggle).emit(());
                false
            }
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
            input.map(|input| TileMsg::Change(input.value().chars().next()))
        });

        let max_length = if ctx.props().state == TileMode::Correct {
            1
        } else {
            10
        };

        let button = if ctx.props().active {
            html! {
                <button class="toggle" onclick={link.callback(|_| TileMsg::Toggle)} tabindex="-1">{ "Toggle" }</button>
            }
        } else {
            html! {}
        };

        html! {
            <div>
                <input
                    class="tile"
                    data-state={ ctx.props().state.to_string() }
                    value={ ctx.props()._entry.map(String::from).unwrap_or_else(String::new) }
                    onchange={ on_value_change }
                    maxlength={ max_length.to_string() }
                    disabled= {!ctx.props().active }
                />
                { button }
            </div>
        }
    }
}
