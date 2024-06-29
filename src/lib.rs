use std::{
    ffi::{c_char, CString},
    fs::File,
    io::Write,
    ptr,
    sync::mpsc,
    thread,
};

mod db;
mod imgui;
mod model;
mod pairings;
mod str_buffer;
mod trf;

use crate::model::Player;
use imgui::*;
use model::Tournament;
use pairings::pair_players;

pub struct PlugState {
    db_thread: Option<thread::JoinHandle<()>>,
    db_tx: Option<mpsc::Sender<db::Task>>,
    players: Vec<model::Player>,
    tournament: Tournament,
    selection: Option<usize>,
}

#[no_mangle]
pub extern "C" fn plug_state_init() -> *mut PlugState {
    let players = unsafe { db::load_players("test.db").unwrap() };

    let state = Box::new(PlugState {
        db_thread: None,
        db_tx: None,
        players,
        tournament: Tournament::default(),
        selection: None,
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

        if igBeginMenu(c"Tournament".as_ptr(), true) {
            igEndMenu();
        }

        if igBeginMenu(c"Theme".as_ptr(), true) {
            if igMenuItem_Bool(c"Light".as_ptr(), ptr::null(), false, true) {
                igStyleColorsLight(ptr::null_mut());
            }

            if igMenuItem_Bool(c"Dark".as_ptr(), ptr::null(), false, true) {
                igStyleColorsDark(ptr::null_mut());
            }

            if igMenuItem_Bool(c"Classic".as_ptr(), ptr::null(), false, true) {
                igStyleColorsClassic(ptr::null_mut());
            }

            igEndMenu();
        }

        igEndMainMenuBar();

        igBegin(c"Players".as_ptr(), ptr::null_mut(), 0);

        let table_flags =
            ImGuiTableFlags_SizingStretchSame | ImGuiTableFlags_BordersH | ImGuiTableFlags_BordersV;

        if igBeginTable(
            c"tournament_players".as_ptr(),
            7,
            table_flags as i32,
            ImVec2 { x: 0.0, y: 0.0 },
            0.0,
        ) {
            igTableSetupColumn(c"ID".as_ptr(), 0, 0.0, 0);
            igTableSetupColumn(c"Name".as_ptr(), 0, 0.0, 0);
            igTableSetupColumn(c"Fed".as_ptr(), 0, 0.0, 0);
            igTableSetupColumn(c"Sex".as_ptr(), 0, 0.0, 0);
            igTableSetupColumn(c"Title".as_ptr(), 0, 0.0, 0);
            igTableSetupColumn(c"ID FIDE".as_ptr(), 0, 0.0, 0);
            igTableSetupColumn(c"Rtg FIDE".as_ptr(), 0, 0.0, 0);
            igTableHeadersRow();

            for (row, player) in state.players.iter_mut().enumerate() {
                let id = row as i32 + 1;

                igTableNextRow(0, 26.0);

                igTableSetColumnIndex(0);
                let is_selected = state
                    .selection
                    .is_some_and(|selected_row| row == selected_row);

                let selectable_flags = ImGuiSelectableFlags_AllowOverlap;

                let id_string = CString::new(id.to_string()).unwrap();

                if igSelectable_Bool(
                    id_string.as_ptr(),
                    is_selected,
                    selectable_flags as i32,
                    ImVec2 { x: 0.0, y: 26.0 },
                ) {
                    if is_selected {
                        state.selection = None;
                    } else {
                        state.selection = Some(row);
                    }
                }

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

                if igInputText(
                    c"##name".as_ptr(),
                    player.name.as_mut_ptr(),
                    player.name.capacity(),
                    0,
                    None,
                    ptr::null_mut(),
                ) {
                    state
                        .db_tx
                        .as_mut()
                        .unwrap()
                        .send(db::Task::UpdatePlayerName {
                            id,
                            name: player.name.clone(),
                        })
                        .expect("Could not send message to db");
                }

                igPopID();
                igPopItemWidth();

                igTableSetColumnIndex(2);
                igPushItemWidth(-1.0);
                igPushID_Int((7 * row + 2) as i32);
                let fed_flags =
                    ImGuiInputTextFlags_CharsNoBlank | ImGuiInputTextFlags_CharsUppercase;

                igInputText(
                    c"##fed".as_ptr(),
                    player.federation.as_mut_ptr(),
                    player.federation.capacity(),
                    fed_flags as i32,
                    None,
                    ptr::null_mut(),
                );

                igPopID();
                igPopItemWidth();

                igPopStyleColor(1);

                igTableSetColumnIndex(3);
                igPushItemWidth(-1.0);
                igPushID_Int((11 * row + 3) as i32);

                if igCombo_Str(
                    c"##sex".as_ptr(),
                    &mut player.sex as *mut model::Sex as *mut i32,
                    b"-\0M\0F\0\0".as_ptr() as *const c_char,
                    -1,
                ) {
                    state
                        .db_tx
                        .as_mut()
                        .unwrap()
                        .send(db::Task::UpdatePlayerSex {
                            id,
                            sex: player.sex,
                        })
                        .expect("Could not send message to db");
                }

                igPopID();
                igPopItemWidth();

                igTableSetColumnIndex(4);
                igPushItemWidth(-1.0);
                igPushID_Int((11 * row + 4) as i32);

                if igCombo_Str(
                    c"##title".as_ptr(),
                    &mut player.title as *mut model::Title as *mut i32,
                    b"-\0GM\0IM\0FM\0CM\0\0".as_ptr() as *const c_char,
                    -1,
                ) {
                    state
                        .db_tx
                        .as_mut()
                        .unwrap()
                        .send(db::Task::UpdatePlayerTitle {
                            id,
                            title: player.title,
                        })
                        .expect("Could not send message to db");
                }

                igPopID();
                igPopItemWidth();

                igPushStyleColor_Vec4(
                    ImGuiCol_FrameBg as i32,
                    ImVec4 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                        w: 0.0,
                    },
                );

                igTableSetColumnIndex(5);
                igPushItemWidth(-1.0);
                igPushID_Int((7 * row + 5) as i32);
                igInputText(
                    c"##fide_id".as_ptr(),
                    player.fide_id.as_mut_ptr(),
                    player.fide_id.capacity(),
                    ImGuiInputTextFlags_CharsDecimal as i32,
                    None,
                    ptr::null_mut(),
                );
                igPopID();
                igPopItemWidth();

                igTableSetColumnIndex(6);
                igPushItemWidth(-1.0);
                igPushID_Int((7 * row + 6) as i32);
                igInputInt(
                    c"##fide_id".as_ptr(),
                    &mut player.fide_rating as *mut i32,
                    0,
                    0,
                    0,
                );
                igPopID();
                igPopItemWidth();

                igPopStyleColor(1);
            }

            igEndTable();
        }

        igEnd();

        igShowDemoWindow(ptr::null_mut());

        igBegin(c"Control".as_ptr(), ptr::null_mut(), 0);

        if igButton(c"Add player".as_ptr(), ImVec2 { x: 0.0, y: 0.0 }) {
            let id = state.players.len() as i32 + 1;
            let player = Player::default();

            state.players.push(player.clone());

            state
                .db_tx
                .as_ref()
                .unwrap()
                .send(db::Task::AddPlayer { id, player })
                .expect("Could not send message to db");
        }

        igSameLine(0.0, -1.0);

        if igButton(c"Remove player".as_ptr(), ImVec2 { x: 0.0, y: 0.0 }) {
            if state.players.len() != 0 {
                let index = match state.selection.take() {
                    Some(index) => index,
                    None => state.players.len() - 1,
                };

                state.players.remove(index);

                state
                    .db_tx
                    .as_ref()
                    .unwrap()
                    .send(db::Task::RemovePlayer {
                        id: index as i32 + 1,
                    })
                    .expect("Could not send message to db");
            }
        }

        // if igButton(c"Move up".as_ptr(), ImVec2 { x: 0.0, y: 0.0 }) {
        //     match state.selection {
        //         Some(index) if index > 0 => {
        //             state.players.swap(index, index - 1);
        //             state.selection = Some(index - 1);
        //         }
        //         _ => (),
        //     }
        // }

        // igSameLine(0.0, -1.0);

        // if igButton(c"Move down".as_ptr(), ImVec2 { x: 0.0, y: 0.0 }) {
        //     match state.selection {
        //         Some(index) if index < state.players.len() - 1 => {
        //             state.players.swap(index, index + 1);
        //             state.selection = Some(index + 1);
        //         }
        //         _ => (),
        //     }
        // }

        if igButton(c"Pair".as_ptr(), ImVec2 { x: 0.0, y: 0.0 }) {
            let pairings = pair_players(
                &state.tournament,
                &state.players,
                "bbpPairings/bbpPairings.exe",
            );
            println!("{:?}", pairings);
        }

        if igButton(c"Export".as_ptr(), ImVec2 { x: 0.0, y: 0.0 }) {
            let trf = trf::export(&state.tournament, &state.players).unwrap();
            thread::spawn(move || {
                File::create("tournament.txt")
                    .unwrap()
                    .write_all(trf.as_bytes())
                    .unwrap();
            });
        }

        igEnd();

        igBegin(c"Tournament settings".as_ptr(), ptr::null_mut(), 0);

        igInputText(
            c"Name".as_ptr(),
            state.tournament.name.as_mut_ptr(),
            state.tournament.name.capacity(),
            0,
            None,
            ptr::null_mut(),
        );

        igInputText(
            c"City".as_ptr(),
            state.tournament.city.as_mut_ptr(),
            state.tournament.city.capacity(),
            0,
            None,
            ptr::null_mut(),
        );

        igInputText(
            c"Federation".as_ptr(),
            state.tournament.federation.as_mut_ptr(),
            state.tournament.federation.capacity(),
            0,
            None,
            ptr::null_mut(),
        );

        igInputText(
            c"Chief arbiter".as_ptr(),
            state.tournament.chief_arbiter.as_mut_ptr(),
            state.tournament.chief_arbiter.capacity(),
            0,
            None,
            ptr::null_mut(),
        );

        igEnd();
        // igShowStyleEditor(ptr::null_mut());
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

// struct InputTextData<'a> {
//     str: &'a mut String,
//     max_length: usize,
// }

// #[no_mangle]
// pub extern "C" fn input_text_callback(data: *mut ImGuiInputTextCallbackData) {
//     let user_data = unsafe { &mut *(((*data).UserData) as *mut InputTextData) };
//     let data = unsafe { &mut *data };

//     if data.EventFlag as u32 == ImGuiInputTextFlags_CallbackResize {
//         let str = &mut user_data.str;

//         if str.len() > user_data.max_length {
//             return;
//         }

//         str.reserve(data.BufTextLen as usize - str.len());
//         data.Buf = unsafe { str.as_mut_vec().as_mut_ptr() as *mut i8 };
//     }
// }
