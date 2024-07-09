use crate::{
    model::{Player, Sex, Title},
    str_buffer::StrBuffer,
};
use sqlite3_sys::*;
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    ptr,
    sync::mpsc,
};

pub trait Effect {
    fn inv(self) -> Self;
}

#[derive(Clone)]
pub enum PlayerEffect {
    Add {
        new: Player,
    },
    UpdateName {
        new: StrBuffer<34>,
        old: StrBuffer<34>,
    },
    UpdateFed {
        new: StrBuffer<4>,
        old: StrBuffer<4>,
    },
    UpdateSex {
        new: Sex,
        old: Sex,
    },
    UpdateTitle {
        new: Title,
        old: Title,
    },
    Remove {
        old: Player,
    },
}

impl Effect for PlayerEffect {
    fn inv(self) -> Self {
        match self {
            PlayerEffect::Add { new } => PlayerEffect::Remove { old: new },
            PlayerEffect::UpdateName { new, old } => {
                PlayerEffect::UpdateName { new: old, old: new }
            }
            PlayerEffect::UpdateFed { new, old } => PlayerEffect::UpdateFed { new: old, old: new },
            PlayerEffect::UpdateSex { new, old } => PlayerEffect::UpdateSex { new: old, old: new },
            PlayerEffect::UpdateTitle { new, old } => {
                PlayerEffect::UpdateTitle { new: old, old: new }
            }
            PlayerEffect::Remove { old } => PlayerEffect::Add { new: old },
        }
    }
}

pub enum Task {
    Player { id: i32, task: PlayerEffect },
    Stop,
}

struct Db {
    handle: *mut sqlite3,
}

impl Db {
    fn new(path: &str) -> Db {
        let mut handle: *mut sqlite3 = ptr::null_mut();

        unsafe {
            let path = CString::new(path).unwrap();
            if sqlite3_open(path.as_ptr(), &mut handle) != SQLITE_OK {
                eprintln!(
                    "{}",
                    CStr::from_ptr(sqlite3_errmsg(handle)).to_str().unwrap()
                );
            }

            set_wall_mode(handle);
        }

        Db { handle }
    }

    fn prepare(self: &mut Self, query: &str) -> *mut sqlite3_stmt {
        let mut stmt: *mut sqlite3_stmt = ptr::null_mut();

        unsafe {
            if sqlite3_prepare_v2(
                self.handle,
                query.as_ptr() as *const c_char,
                query.len() as i32,
                &mut stmt,
                ptr::null_mut(),
            ) != SQLITE_OK
            {
                eprintln!(
                    "{}",
                    CStr::from_ptr(sqlite3_errmsg(self.handle))
                        .to_str()
                        .unwrap()
                );
            }
        }

        stmt
    }
}

pub unsafe fn load_players(db_path: &str) -> Result<Vec<Player>, ()> {
    let mut players = Vec::new();

    let mut db = Db::new(db_path);

    let stmt = db.prepare(
        "SELECT name, federation, sex, title, fide_rating, fide_id from players ORDER BY id ASC",
    );

    while sqlite3_step(stmt) != SQLITE_DONE {
        let name = StrBuffer::from_raw_copy(sqlite3_column_text(stmt, 0));
        let federation = StrBuffer::from_raw_copy(sqlite3_column_text(stmt, 1));
        let sex: Sex = std::mem::transmute(sqlite3_column_int(stmt, 2));
        let title: Title = std::mem::transmute(sqlite3_column_int(stmt, 3));
        let fide_rating = sqlite3_column_int(stmt, 4);
        let fide_id = StrBuffer::from_raw_copy(sqlite3_column_text(stmt, 5));

        players.push(Player {
            name,
            federation,
            sex,
            title,
            fide_rating,
            fide_id,
        })
    }

    sqlite3_finalize(stmt);
    sqlite3_close(db.handle);

    Ok(players)
}

pub fn init_database(db: *mut sqlite3) -> Result<(), ()> {
    let script = CString::new(include_str!("../init.sql")).unwrap();

    unsafe {
        if sqlite3_exec(db, script.as_ptr(), None, ptr::null_mut(), ptr::null_mut()) != SQLITE_OK {
            return Err(());
        }
    }

    Ok(())
}

pub fn worker(db_path: &str, rx: mpsc::Receiver<Task>) -> Result<(), ()> {
    println!("[DB_THREAD] Start");

    let mut db = Db::new(db_path);

    for task in rx {
        match task {
            Task::Player { id, task } => match task {
                PlayerEffect::Add { new } => {
                    let stmt = db.prepare(
                        "INSERT INTO players (
                                id,
                                name,
                                federation,
                                sex,
                                title,
                                fide_rating,
                                fide_id
                            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    );

                    unsafe {
                        sqlite3_bind_int(stmt, 1, id);
                        sqlite3_bind_text(stmt, 2, new.name.as_ptr(), -1, None);
                        sqlite3_bind_text(stmt, 3, new.federation.as_ptr(), -1, None);
                        sqlite3_bind_int(stmt, 4, new.sex as i32);
                        sqlite3_bind_int(stmt, 5, new.title as i32);
                        sqlite3_bind_int(stmt, 6, new.fide_rating);
                        sqlite3_bind_text(stmt, 7, new.federation.as_ptr() as *const i8, -1, None);

                        if sqlite3_step(stmt) != SQLITE_DONE {
                            return Err(());
                        }

                        sqlite3_finalize(stmt);
                    };
                }
                PlayerEffect::UpdateName { new, old } => {
                    let stmt = db.prepare("UPDATE players SET name = ?1 WHERE id = ?2;");

                    unsafe {
                        sqlite3_bind_text(stmt, 1, new.as_ptr(), -1, None);
                        sqlite3_bind_int(stmt, 2, id);

                        if sqlite3_step(stmt) != SQLITE_DONE {
                            return Err(());
                        }

                        sqlite3_finalize(stmt);
                    };
                }
                PlayerEffect::UpdateFed { new, old } => {
                    let stmt = db.prepare("UPDATE players SET federation = ?1 WHERE id = ?2;");

                    unsafe {
                        sqlite3_bind_text(stmt, 1, new.as_ptr(), -1, None);
                        sqlite3_bind_int(stmt, 2, id);

                        if sqlite3_step(stmt) != SQLITE_DONE {
                            return Err(());
                        }

                        sqlite3_finalize(stmt);
                    };
                }
                PlayerEffect::UpdateSex { new, old } => {
                    let stmt = db.prepare("UPDATE players SET sex = ?1 WHERE id = ?2;");

                    unsafe {
                        sqlite3_bind_int(stmt, 1, new as i32);
                        sqlite3_bind_int(stmt, 2, id);

                        if sqlite3_step(stmt) != SQLITE_DONE {
                            return Err(());
                        }

                        sqlite3_finalize(stmt);
                    };
                }
                PlayerEffect::UpdateTitle { new, old } => {
                    let stmt = db.prepare("UPDATE players SET title = ?1 WHERE id = ?2;");

                    unsafe {
                        sqlite3_bind_int(stmt, 1, new as i32);
                        sqlite3_bind_int(stmt, 2, id);

                        if sqlite3_step(stmt) != SQLITE_DONE {
                            return Err(());
                        }

                        sqlite3_finalize(stmt);
                    };
                }
                PlayerEffect::Remove { old } => {
                    let stmt = db.prepare("DELETE FROM players WHERE id = ?1;");

                    unsafe {
                        sqlite3_bind_int(stmt, 1, id);

                        if sqlite3_step(stmt) != SQLITE_DONE {
                            return Err(());
                        }

                        sqlite3_finalize(stmt);
                    };

                    let stmt = db.prepare("UPDATE players SET id=id-1 WHERE id > ?1;");

                    unsafe {
                        sqlite3_bind_int(stmt, 1, id);

                        if sqlite3_step(stmt) != SQLITE_DONE {
                            return Err(());
                        }

                        sqlite3_finalize(stmt);
                    };
                }
            },
            Task::Stop => {
                break;
            }
        }
    }

    println!("[DB_THREAD] Stop");

    Ok(())
}

unsafe fn set_wall_mode(db: *mut sqlite3) {
    sqlite3_exec(
        db,
        c"PRAGMA journal_mode=WAL;".as_ptr(),
        None,
        ptr::null_mut(),
        ptr::null_mut(),
    );
}
