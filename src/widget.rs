use std::ffi::CStr;

use crate::imgui::*;

pub unsafe fn button(label: &CStr) -> bool {
    // igPushStyleColor_Vec4(
    //     ImGuiCol_Text as i32,
    //     ImVec4 {
    //         x: 1.0,
    //         y: 1.0,
    //         z: 1.0,
    //         w: 1.0,
    //     },
    // );

    let is_pressed = igButton(label.as_ptr(), ImVec2 { x: 0.0, y: 0.0 });

    // igPopStyleColor(1);

    is_pressed
}

pub unsafe fn input_text() -> bool {}
