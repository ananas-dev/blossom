use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::{sync::mpsc, thread};

use crate::db::{self, Effect, PlayerEffect};
use crate::model::{Player, Round, Tournament};

#[derive(Clone)]
enum UndoEffect {
    Player { id: i32, task: PlayerEffect },
}

impl UndoEffect {
    pub fn inverse(self) -> UndoEffect {
        match self {
            UndoEffect::Player { id, task } => UndoEffect::Player {
                id,
                task: match task {
                    PlayerEffect::Add { new } => PlayerEffect::Remove { old: new },
                    PlayerEffect::UpdateName { new, old } => {
                        PlayerEffect::UpdateName { new: old, old: new }
                    }
                    PlayerEffect::UpdateFed { new, old } => {
                        PlayerEffect::UpdateFed { new: old, old: new }
                    }
                    PlayerEffect::UpdateSex { new, old } => {
                        PlayerEffect::UpdateSex { new: old, old: new }
                    }
                    PlayerEffect::UpdateTitle { new, old } => {
                        PlayerEffect::UpdateTitle { new: old, old: new }
                    }
                    PlayerEffect::Remove { old } => PlayerEffect::Add { new: old },
                },
            },
        }
    }
}

pub struct PlugState {
    pub db_thread: Option<thread::JoinHandle<()>>,
    pub db_tx: Option<mpsc::Sender<db::Task>>,
    pub players: Vec<Player>,
    pub tournament: Tournament,
    pub rounds: Vec<Round>,
    pub selection: Option<usize>,
    pub undo_stack: Arc<Mutex<VecDeque<UndoEffect>>>,
    pub redo_stack: Arc<Mutex<VecDeque<UndoEffect>>>,
}

impl PlugState {
    pub fn undo(&mut self) {
        if let Some(effect) = self.undo_stack.lock().unwrap().pop_back() {
            match effect.clone() {
                UndoEffect::Player { id, task } => {
                    let inv_effect = task.inv();

                    match inv_effect.clone() {
                        PlayerEffect::Add { new } => self.players.push(new),
                        PlayerEffect::UpdateName { new, old: _ } => {
                            self.players[id as usize].name = new;
                        }
                        PlayerEffect::UpdateFed { new, old: _ } => {
                            self.players[id as usize].federation = new;
                        }
                        PlayerEffect::UpdateSex { new, old: _ } => {
                            self.players[id as usize].sex = new;
                        }
                        PlayerEffect::UpdateTitle { new, old: _ } => {
                            self.players[id as usize].title = new;
                        }
                        PlayerEffect::Remove { old: _ } => {
                            self.players.remove(id as usize);
                        }
                    }

                    self.db_tx
                        .as_ref()
                        .unwrap()
                        .send(db::Task::Player {
                            id,
                            task: inv_effect,
                        })
                        .unwrap();
                }
            }

            self.redo_stack.lock().unwrap().push_back(effect);
        }
    }

    pub fn redo(&mut self) {
        if let Some(effect) = self.undo_stack.lock().unwrap().pop_back() {
            match effect.clone() {
                UndoEffect::Player { id, task } => {
                    match task.clone() {
                        PlayerEffect::Add { new } => self.players.push(new),
                        PlayerEffect::UpdateName { new, old: _ } => {
                            self.players[id as usize].name = new;
                        }
                        PlayerEffect::UpdateFed { new, old: _ } => {
                            self.players[id as usize].federation = new;
                        }
                        PlayerEffect::UpdateSex { new, old: _ } => {
                            self.players[id as usize].sex = new;
                        }
                        PlayerEffect::UpdateTitle { new, old: _ } => {
                            self.players[id as usize].title = new;
                        }
                        PlayerEffect::Remove { old: _ } => {
                            self.players.remove(id as usize);
                        }
                    }

                    self.db_tx
                        .as_ref()
                        .unwrap()
                        .send(db::Task::Player { id, task })
                        .unwrap();
                }
            }

            self.redo_stack.lock().unwrap().push_back(effect);
        }
    }
}
