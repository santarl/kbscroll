extern crate winapi;

use std::env;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use winapi::um::winuser::{
    SendInput, INPUT, INPUT_MOUSE, MOUSEEVENTF_WHEEL, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
    MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP,
    MOUSEINPUT,
};

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

fn click_mouse(button_down: u32, button_up: u32, times: u32) {
    for _ in 0..times {
        let mouse_down = MOUSEINPUT {
            dx: 0,
            dy: 0,
            mouseData: 0,
            dwFlags: button_down,
            time: 0,
            dwExtraInfo: 0,
        };

        let mouse_up = MOUSEINPUT {
            dx: 0,
            dy: 0,
            mouseData: 0,
            dwFlags: button_up,
            time: 0,
            dwExtraInfo: 0,
        };

        let input_down = INPUT {
            type_: INPUT_MOUSE,
            u: unsafe { std::mem::transmute(mouse_down) },
        };

        let input_up = INPUT {
            type_: INPUT_MOUSE,
            u: unsafe { std::mem::transmute(mouse_up) },
        };

        unsafe {
            SendInput(
                1,
                &input_down as *const _ as *mut INPUT,
                std::mem::size_of::<INPUT>() as i32,
            );
            SendInput(
                1,
                &input_up as *const _ as *mut INPUT,
                std::mem::size_of::<INPUT>() as i32,
            );
        }

        // Small delay between clicks to simulate human behavior
        sleep(Duration::from_millis(100));
    }
}

fn click_left(times: u32) {
    click_mouse(MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, times);
}

fn click_middle(times: u32) {
    click_mouse(MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, times);
}

fn click_right(times: u32) {
    click_mouse(MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, times);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage:");
        eprintln!("  kbscroll.exe scroll [amount]");
        eprintln!("  kbscroll.exe click_left [times]");
        eprintln!("  kbscroll.exe click_middle [times]");
        eprintln!("  kbscroll.exe click_right [times]");
        std::process::exit(1);
    }

    let command = &args[1];
    let value_str = &args[2];

    let value = i32::from_str(value_str)
        .expect("Please provide a valid integer for the amount or number of clicks.");

    match command.as_str() {
        "scroll" => scroll_mouse(value),
        "click_left" => click_left(value as u32),
        "click_middle" => click_middle(value as u32),
        "click_right" => click_right(value as u32),
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Usage:");
            eprintln!("  kbscroll.exe scroll [amount]");
            eprintln!("  kbscroll.exe click_left [times]");
            eprintln!("  kbscroll.exe click_middle [times]");
            eprintln!("  kbscroll.exe click_right [times]");
            std::process::exit(1);
        }
    }
}