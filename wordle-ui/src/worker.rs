use std::collections::{HashMap, HashSet};
use yew_agent::{HandlerId, Public, Agent, AgentLink};
use serde::{Deserialize, Serialize};
use wordle_lib::overlap::Overlap;
use crate::BaseTileState;
use crate::tile::TileState;


pub struct Worker {
    words: Vec<&'static str>,
    vs: Vec<Vec<HashSet<usize>>>,
    link: AgentLink<Self>,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerInput {
    pub state: Vec<BaseTileState>,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerOutput {
    pub values: Vec<String>,
}

fn _index(c: char) -> usize {
    ((c as u8) - ('a' as u8)) as usize
}

impl Agent for Worker {
    type Reach = Public<Self>;
    type Message = ();
    type Input = WorkerInput;
    type Output = WorkerOutput;

    fn create(link: AgentLink<Self>) -> Self {
        let mut vs = vec![vec![HashSet::new(); 26]; 5];
        let mut words = Vec::new();

        for word in wordle_dict::WORDS.iter() {
            if word.len() == 5 {
                let index = words.len();
                words.push(*word);
                for (j, c) in word.chars().enumerate() {
                    for c in c.to_lowercase() {
                        let c = _index(c);
                        vs[j][c].insert(index);
                    }
                }
            }
        }

        Self { vs, link, words }
    }

    fn update(&mut self, _msg: Self::Message) {
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        // First get all possible words
        let mut potentials: HashSet<_> = (0..self.words.len()).collect();

        for (i, s) in msg.state.iter().enumerate() {
            match s {
                BaseTileState { state, entry } => {
                    if entry.is_empty() {
                        continue;
                    }

                    match state {
                        TileState::Correct => {
                            let c = entry.chars().next().unwrap().to_lowercase().next().unwrap();
                            potentials = potentials.intersection(&self.vs[i][_index(c)]).copied().collect();
                        },
                        TileState::Absent => {
                            for c in entry.to_lowercase().chars() {
                                let forced: Vec<_> = (0..5)
                                    .filter(|&k| k != i)
                                    .filter_map(|t| {
                                        if msg.state[t].state == TileState::Correct && msg.state[t].entry.to_lowercase().contains(c) {
                                            Some(t)
                                        } else {
                                            None
                                        }
                                    }).collect();

                                if forced.is_empty() {
                                    for p in 0..5 {
                                        potentials = potentials.difference(&self.vs[p][_index(c)]).copied().collect();
                                    }
                                } else {
                                    // there can't be a `c` at any index that isn't forced.
                                    for p in 0..5 {
                                        if !forced.contains(&p) {
                                            potentials = potentials.difference(&self.vs[p][_index(c)]).copied().collect();
                                        }
                                    }
                                }

                                // // is there a correct state at some other place?
                                // let other_positions: Vec<_> = msg.state.iter()
                                //     .enumerate()
                                //     .filter_map(|(k, t)| {
                                //         if i != k && t.state == TileState::Correct && t.entry.contains(c) {
                                //             Some(k)
                                //         } else {
                                //             None
                                //         }
                                //     })
                                //     .collect();
                                //
                                // for p in 0..5 {
                                //     if !other_positions.contains(&p) {
                                //         potentials = potentials.difference(&self.vs[p][_index(c)]).copied().collect();
                                //     }
                                // }
                            }
                        }
                        TileState::Present => {
                            for c in entry.to_lowercase().chars() {
                                // there is _not_ a 'c' at this position.
                                potentials = potentials.difference(&self.vs[i][_index(c)]).copied().collect();

                                // there is a `c` at one other position.
                                let other_positions: HashSet<_> =
                                    (0..5).filter(|&j| j != i).map(|j| &self.vs[j][_index(c)])
                                        .fold(HashSet::new(), |l, r| l.union(r).copied().collect());

                                potentials = potentials.intersection(&other_positions).copied().collect();
                            }
                        }
                    }
                }
            }
        }

        let mut base_overlaps = HashMap::new();

        for cs in self.vs.iter() {
            for indices in cs.iter() {
                let valids: HashSet<_> = indices.intersection(&potentials).collect();
                for &&index in &valids {
                    base_overlaps.entry(index).or_insert_with(Overlap::default).total += valids.len();
                }
            }
        }


        let mut ordered: Vec<_> = potentials.iter().copied().collect();
        ordered.sort_by_key(|i| {
            let overlap = base_overlaps.get(i).copied().unwrap_or_else(Overlap::default);
            (overlap.total + (overlap.partial / 3), self.words[*i])
            // potentials
            //     .iter()
            //     .map(|&v| Overlap::new(self.words[i], self.words[v]))
            //     .sum::<Overlap>()
        });

        // Get the last 5 elements of ordered
        if ordered.len() > 5 {
            ordered.drain(0..ordered.len() - 5);
        }

        let outputs = ordered.into_iter()
            .rev()
            .map(|i| String::from(self.words[i]))
            .collect();

            //      words.sort_by_cached_key(|&w| {
        //             buffer
        //                 .iter()
        //                 .map(|&v| Overlap::new(w, v))
        //                 .sum::<Overlap>()
        //         });

        // let infer = self.engine.infer(
        //     wordle_dict::WORDS.iter().filter(|w| w.len() == 5).copied()
        // );

        // let mut engine = InferenceEngine::<5>::new();
        //
        self.link.respond(id, Self::Output { values: outputs });
    }

    fn name_of_resource() -> &'static str {
        "wasm.js"
    }
}