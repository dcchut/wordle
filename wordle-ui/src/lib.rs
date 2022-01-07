mod model;
mod tile;
mod worker;

use crate::model::TileMode;
use crate::tile::TileProps;
use crate::worker::{Worker, WorkerInput, WorkerOutput};
use model::Board;
use tile::Tile;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged, Threaded};

pub enum BoardMsg {
    Toggle(usize),
    Change(usize, Option<char>),
    AddBoard,
    RunWorker,
    WorkerMsg(WorkerOutput),
}

pub struct Model {
    boards: Vec<Board>,
    worker: Box<dyn Bridge<Worker>>,
    outputs: Vec<String>,
}

impl Component for Model {
    type Message = BoardMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let boards = vec![Board::default()];
        let cb = {
            let link = ctx.link().clone();
            Callback::from(move |e| link.send_message(Self::Message::WorkerMsg(e)))
        };
        let worker = Worker::bridge(cb);

        Self {
            boards,
            worker,
            outputs: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BoardMsg::Toggle(index) => {
                let mode = self.boards[self.boards.len() - 1].tiles[index]
                    .mode
                    .toggle();
                self.boards.last_mut().unwrap().tiles[index].mode = mode;
                ctx.link().send_message(BoardMsg::RunWorker);
                true
            }
            BoardMsg::Change(index, entry) => {
                self.boards.last_mut().unwrap().tiles[index].char = entry;
                ctx.link().send_message(BoardMsg::RunWorker);
                true
            }
            BoardMsg::AddBoard => {
                // Can only add a board if the last board is filled.
                if let Some(mut last) = self.boards.last().cloned() {
                    if last.tiles.iter().all(|tile| tile.char.is_some()) {
                        // Replace all the non-correct tiles with emptyness.
                        for tile_state in &mut last.tiles {
                            if tile_state.mode != TileMode::Correct {
                                tile_state.char = None;
                                tile_state.mode = TileMode::Absent;
                            }
                        }
                        self.boards.push(last);
                        return true;
                    }
                }
                false
            }
            BoardMsg::RunWorker => {
                self.worker.send(WorkerInput {
                    boards: self.boards.clone(),
                });
                false
            }
            BoardMsg::WorkerMsg(WorkerOutput { values }) => {
                self.outputs = values;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let mut tiles = Vec::new();

        // Split out the last board from the rest.
        let (last, boards) = self.boards.split_last().unwrap();
        for tile_state in boards.iter().map(|b| &b.tiles).flatten() {
            let props = TileProps {
                state: tile_state.mode,
                _entry: tile_state.char,
                active: false,
                // this should be an enum or something to represent this properly
                on_toggle: link.callback_once(|_| BoardMsg::Toggle(0)),
                on_change: link.callback_once(|_| BoardMsg::Change(0, None)),
            };
            tiles.push(html! { <Tile ..props />});
        }

        for (i, tile_state) in last.tiles.iter().enumerate() {
            let active = if self.boards.len() > 1
                && self.boards[self.boards.len() - 2].tiles[i].mode == TileMode::Correct
            {
                false
            } else {
                true
            };

            let props = TileProps {
                state: tile_state.mode,
                _entry: tile_state.char,
                active,
                on_toggle: link.callback_once(move |_| BoardMsg::Toggle(i)),
                on_change: link.callback_once(move |e| BoardMsg::Change(i, e)),
            };
            tiles.push(html! { <Tile ..props />});
        }

        let outputs = self.outputs.clone();

        html! {
            <div class="App">
                <div class="board-container">
                    <div class="board">
                        { for tiles.into_iter() }
                    </div>
                </div>
                <div>
                    <button onclick={link.callback(|_| BoardMsg::AddBoard)}>{ "Next" }</button>
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
    wasm_logger::init(wasm_logger::Config::default());

    if Reflect::has(&global(), &JsValue::from_str("window")).unwrap() {
        log::info!("Starting application");
        yew::start_app::<Model>();
    } else {
        log::info!("Starting worker");
        crate::worker::Worker::register();
    }
}
