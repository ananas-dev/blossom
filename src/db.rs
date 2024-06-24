use crate::model;
use rusqlite::{self, Connection};
use std::sync::mpsc;

pub enum Task {
    AddPlayer(model::Player),
    Stop,
}

pub fn worker(db_path: &str, rx: mpsc::Receiver<Task>) -> rusqlite::Result<()> {
    println!("[DB_THREAD] Start");

    let conn = Connection::open(db_path)?;

    for task in rx {
        match task {
            Task::AddPlayer(player) => {
                conn.execute(
                    "INSERT INTO players (
                        name,
                        federation,
                        sex,
                        title,
                        fide_rating,
                        fide_id
                    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                    (
                        &player.name,
                        &player.federation,
                        &(player.sex as u8),
                        &(player.title as u8),
                        &player.fide_rating,
                        &player.fide_id,
                    ),
                )?;
            }
            Task::Stop => {
                break;
            }
        }
    }

    println!("[DB_THREAD] Stop");

    Ok(())
}
