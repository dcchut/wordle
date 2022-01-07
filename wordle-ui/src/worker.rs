use crate::model::{Board, TileMode};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use wordle_lib::{Engine, Inference, InferenceKind};
use yew_agent::{Agent, AgentLink, HandlerId, Public};

fn determine_inferences(boards: &[Board]) -> Vec<Inference> {
    let mut inferences = HashSet::new();

    for board in boards {
        let mut states = HashMap::new();

        for (i, tile) in board.tiles.iter().enumerate() {
            if let Some(c) = tile.char {
                match tile.mode {
                    TileMode::Correct => {
                        inferences.insert(Inference::new(c, i, InferenceKind::Correct));
                        states
                            .entry(c)
                            .or_insert_with(|| (Vec::new(), Vec::new(), Vec::new()))
                            .0
                            .push(i);
                    }
                    TileMode::Present => {
                        inferences.insert(Inference::new(c, i, InferenceKind::Present));
                        states
                            .entry(c)
                            .or_insert_with(|| (Vec::new(), Vec::new(), Vec::new()))
                            .1
                            .push(i);
                    }
                    TileMode::Absent => {
                        inferences.insert(Inference::new(c, i, InferenceKind::AbsentLocal));
                        states
                            .entry(c)
                            .or_insert_with(|| (Vec::new(), Vec::new(), Vec::new()))
                            .2
                            .push(i);
                    }
                }
            }
        }

        // Board-level inferences
        for (c, (correct, present, absent)) in states.into_iter() {
            // We can put a lower (or exact) bound on the number of times `c` appears in the word.
            let kind = if !correct.is_empty() || !present.is_empty() {
                // We have correct or present tiles; this puts some restriction on valid words.
                if !absent.is_empty() {
                    // If we have absent tiles then we have an _exact_ bound on valid words.
                    InferenceKind::Count(correct.len() + present.len())
                } else {
                    // If we don't have absent tiles then we only have a lower bound.
                    InferenceKind::AtLeast(correct.len() + present.len())
                }
            } else {
                InferenceKind::AbsentGlobal
            };
            inferences.insert(Inference::new(c, 0, kind));
        }
    }

    inferences.into_iter().collect()
}

pub struct Worker {
    words: Vec<&'static str>,
    link: AgentLink<Self>,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerInput {
    pub boards: Vec<Board>,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerOutput {
    pub values: Vec<String>,
}

fn _index(c: char) -> usize {
    ((c as u8) - (b'a')) as usize
}

impl Agent for Worker {
    type Reach = Public<Self>;
    type Message = ();
    type Input = WorkerInput;
    type Output = WorkerOutput;

    fn create(link: AgentLink<Self>) -> Self {
        let mut words = Vec::new();

        for word in wordle_dict::WORDS.iter() {
            if word.len() == 5 {
                words.push(*word);
            }
        }

        Self { link, words }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        let engine = Engine::<5>::new(self.words.iter().copied());
        let inferences = determine_inferences(&msg.boards);

        let mut results = engine.evaluate(&inferences);
        results.sort_by_key(|(w, overlap)| {
            std::cmp::Reverse((overlap.total + (overlap.partial / 3), *w))
        });

        // take the top 20 results
        let outputs = results
            .into_iter()
            .take(20)
            .map(|(w, _)| String::from(w))
            .collect();

        self.link.respond(id, Self::Output { values: outputs });
    }

    fn name_of_resource() -> &'static str {
        "./wasm.js"
    }
}
