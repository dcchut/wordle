mod tile;
mod worker;

use crate::tile::TileProps;
use tile::{Tile, TileState};
use yew::prelude::*;
use serde::{Deserialize, Serialize};
use yew_agent::{Bridge, Bridged, Threaded};
use crate::worker::{Worker, WorkerInput, WorkerOutput};
use wasm_bindgen::prelude::*;

pub enum BoardMsg {
    Toggle(usize),
    Change(usize, String),
    RunWorker,
    WorkerMsg(WorkerOutput),
}

#[derive(Clone, Deserialize, Serialize)]
pub struct BaseTileState {
    pub state: TileState,
    pub entry: String,
}

pub struct Model {
    tiles: Vec<BaseTileState>,
    worker: Box<dyn Bridge<Worker>>,
    outputs: Vec<String>,
}


impl Component for Model {
    type Message = BoardMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let tiles = vec![
            BaseTileState {
                state: TileState::Absent,
                entry: String::new(),
            },
            BaseTileState {
                state: TileState::Absent,
                entry: String::new(),
            },
            BaseTileState {
                state: TileState::Absent,
                entry: String::new(),
            },
            BaseTileState {
                state: TileState::Absent,
                entry: String::new(),
            },
            BaseTileState {
                state: TileState::Absent,
                entry: String::new(),
            },
        ];

        let cb = {
            let link = ctx.link().clone();
            Callback::from(move |e| link.send_message(Self::Message::WorkerMsg(e)))
        };
        let worker = Worker::bridge(cb);

        Self {
            tiles,
            worker,
            outputs: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BoardMsg::Toggle(index) => {
                self.tiles[index].state = self.tiles[index].state.toggle();
                ctx.link().send_message(BoardMsg::RunWorker);
                true
            },
            BoardMsg::Change(index, entry) => {
                self.tiles[index].entry = entry;
                ctx.link().send_message(BoardMsg::RunWorker);
                true
            }
            BoardMsg::RunWorker => {
                self.worker.send(WorkerInput { state: self.tiles.clone() });
                false
            }
            BoardMsg::WorkerMsg(WorkerOutput{ values }) => {
                self.outputs = values;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let tiles = self
            .tiles
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, state)| {
                let props = TileProps {
                    state: state.state,
                    entry: state.entry,
                    on_toggle: link.callback_once(move |_| BoardMsg::Toggle(i)),
                    on_change: link.callback_once(move |e| BoardMsg::Change(i, e)),
                };
                html! {
                    <Tile ..props />
                }
            });

        let outputs = self.outputs.clone();

        html! {
            <div class="App">
                <div class="board-container">
                    <div class="board">
                        { for tiles.into_iter() }
                    </div>
                </div>
                <ul class="item-list">
                    {
                        outputs.into_iter().map(|w| {
                            html!{
                                <li key={w.as_str()}>{ w.as_str() }</li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    use js_sys::{global, Reflect};


    if Reflect::has(&global(), &JsValue::from_str("window")).unwrap() {
        yew::start_app::<Model>();
    } else {
        crate::worker::Worker::register();
    }
}