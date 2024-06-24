use std::{ffi::CString, ptr, sync::mpsc, thread};

mod db;
mod imgui;
mod model;

use crate::model::Player;
use imgui::*;

pub struct PlugState {
    db_thread: Option<thread::JoinHandle<()>>,
    db_tx: Option<mpsc::Sender<db::Task>>,
    players: Vec<model::Player>,
}

macro_rules! cstr {
    ($str:expr) => {
        std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($str, "\0").as_bytes()).as_ptr()
    };
}

#[no_mangle]
pub extern "C" fn plug_state_init() -> *mut PlugState {
    let state = Box::new(PlugState {
        db_thread: None,
        db_tx: None,
        players: Vec::new(),
    });

    Box::into_raw(state) as *mut PlugState
}

#[no_mangle]
pub extern "C" fn plug_state_free(state: *mut PlugState) {
    if state.is_null() {
        return;
    }

    let _ = unsafe {
        // Will free
        Box::from_raw(state)
    };
}

#[no_mangle]
pub extern "C" fn plug_init(state: *mut PlugState) {
    println!("[PLUGIN] Init");

    let state = unsafe { &mut *state };

    let (tx, rx) = mpsc::channel();

    state.db_tx = Some(tx);

    state.db_thread = Some(thread::spawn(move || {
        let err = db::worker("test.db", rx);
        println!("[SQLITE] {:?}", err);
    }));
}

#[no_mangle]
pub extern "C" fn plug_update(state: *mut PlugState) {
    let state = unsafe { &mut *state };

    unsafe {
        igDockSpaceOverViewport(0, ptr::null(), 0, ptr::null());

        igBeginMainMenuBar();

        if igBeginMenu(cstr!("Tournament"), true) {
            igEndMenu();
        }

        if igBeginMenu(cstr!("Theme"), true) {
            if igMenuItem_Bool(cstr!("Light"), ptr::null(), false, true) {
                igStyleColorsLight(ptr::null_mut());
            }

            if igMenuItem_Bool(cstr!("Dark"), ptr::null(), false, true) {
                igStyleColorsDark(ptr::null_mut());
            }

            if igMenuItem_Bool(cstr!("Classic"), ptr::null(), false, true) {
                igStyleColorsClassic(ptr::null_mut());
            }

            igEndMenu();
        }

        igEndMainMenuBar();

        igBegin(cstr!("Players"), ptr::null_mut(), 0);

        let table_flags =
            ImGuiTableFlags_SizingStretchSame | ImGuiTableFlags_BordersH | ImGuiTableFlags_BordersV;

        if igBeginTable(
            cstr!("tournament_players"),
            7,
            table_flags as i32,
            ImVec2 { x: 0.0, y: 0.0 },
            0.0,
        ) {
            igTableSetupColumn(cstr!("ID"), 0, 0.0, 0);
            igTableSetupColumn(cstr!("Name"), 0, 0.0, 0);
            igTableSetupColumn(cstr!("Fed"), 0, 0.0, 0);
            igTableSetupColumn(cstr!("Sex"), 0, 0.0, 0);
            igTableSetupColumn(cstr!("Title"), 0, 0.0, 0);
            igTableSetupColumn(cstr!("ID FIDE"), 0, 0.0, 0);
            igTableSetupColumn(cstr!("Rtg FIDE"), 0, 0.0, 0);
            igTableHeadersRow();

            for (row, player) in state.players.iter().enumerate() {
                igTableNextRow(0, 0.0);

                igTableSetColumnIndex(0);
                igText(cstr!("%ld"), row);

                igPushStyleColor_Vec4(
                    ImGuiCol_FrameBg as i32,
                    ImVec4 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                        w: 0.0,
                    },
                );
                igTableSetColumnIndex(1);
                igPushItemWidth(-1.0);
                igPushID_Int((7 * row + 1) as i32);

                let test_str = CString::new("TEsta aaa");

                // igInputText(cstr!("##name"), buf, buf_size, flags, callback, user_data)
                igPopItemWidth();
                igPopStyleColor(1);
            }

            igEndTable();
        }

        igEnd();

        igShowDemoWindow(ptr::null_mut());

        igBegin(cstr!("Control"), ptr::null_mut(), 0);

        if igButton(cstr!("Add player"), ImVec2 { x: 0.0, y: 0.0 }) {
            let player = Player::default();

            state.players.push(player.clone());

            state
                .db_tx
                .as_ref()
                .unwrap()
                .send(db::Task::AddPlayer(player))
                .expect("Could not send message to db");
        }

        igEnd();
    }
}

#[no_mangle]
pub extern "C" fn plug_free(state: *mut PlugState) {
    println!("[PLUGIN] Free");

    let state = unsafe { &mut *state };

    if let Some(tx) = state.db_tx.take() {
        let _ = tx.send(db::Task::Stop);
    }

    if let Some(handle) = state.db_thread.take() {
        handle.join().expect("Could not join thread");
    }
}
