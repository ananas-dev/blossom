use std::thread::{self, JoinHandle};

mod imgui;

pub struct PlugState {
    counter: i32,
    db_thread: Option<JoinHandle<()>>,
}

#[no_mangle]
pub extern "C" fn plug_state_init() -> *mut PlugState {
    let state = Box::new(PlugState {
        counter: 0,
        db_thread: None,
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

    state.db_thread = Some(thread::spawn(|| {
        println!("Hello from thread");
    }));
}

#[no_mangle]
pub extern "C" fn plug_update(state: *mut PlugState) {
    println!("[PLUGIN] Update");

    let state = unsafe { &mut *state };

    println!("Counter: {}", state.counter);

    state.counter += 1;
}

#[no_mangle]
pub extern "C" fn plug_free(state: *mut PlugState) {
    println!("[PLUGIN] Free");

    let state = unsafe { &mut *state };

    if let Some(handle) = state.db_thread.take() {
        handle.join().expect("Could not join thread");
    }
}
