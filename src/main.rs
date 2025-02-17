extern crate winapi;

use std::env;
use std::str::FromStr;
use winapi::um::winuser::{SendInput, INPUT, INPUT_MOUSE, MOUSEEVENTF_WHEEL, MOUSEINPUT};

fn scroll_mouse(amount: i32) {
    let mouse_input = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: amount as u32,
        dwFlags: MOUSEEVENTF_WHEEL,
        time: 0,
        dwExtraInfo: 0,
    };

    let input = INPUT {
        type_: INPUT_MOUSE,
        u: unsafe { std::mem::transmute(mouse_input) },
    };

    unsafe {
        SendInput(
            1,
            &input as *const _ as *mut INPUT,
            std::mem::size_of::<INPUT>() as i32,
        );
    }
}

fn main() {
    if env::args().len() != 2 {
        eprintln!("Usage: kbscroll <amount>");
        std::process::exit(1);
    }

    let arg = env::args().nth(1).unwrap();
    let amount =
        i32::from_str(&arg).expect("Please provide a valid integer for the scroll amount.");

    scroll_mouse(amount);
}
