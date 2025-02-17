use std::env;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use winapi::um::winuser::{
    SendInput, INPUT, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
    MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP,
    MOUSEEVENTF_WHEEL, MOUSEINPUT,
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

// Macro to define commands and aliases
macro_rules! commands {
    (
        $(
            $name:ident => [$($alias:expr),+] => $action:expr;
        )*
    ) => {
        fn print_usage() {
            eprintln!("Usage:");
            $(
                eprintln!("  kbscroll.exe {} [{}]", stringify!($name), vec![$($alias),+].join(", "));
            )*
            std::process::exit(1);
        }

        fn execute_command(command: &str, value: i32) {
            match command {
                $(
                    $(
                        $alias => $action(value),
                    )+
                )*
                _ => print_usage(),
            }
        }
    }
}

// Define the commands using the macro
commands! {
    scroll => ["scroll", "wheel", "sc", "wh"] => |v| scroll_mouse(v);
    click_left => ["click_left", "lclick", "lc", "c1"] => |v| click_left(v as u32);
    click_right => ["click_right", "rclick", "rc", "c2"] => |v| click_right(v as u32);
    click_middle => ["click_middle", "mclick", "mc", "c3"] => |v| click_middle(v as u32);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        print_usage();
    }

    let command = &args[1];
    let value_str = &args[2];

    let value = i32::from_str(value_str)
        .expect("Please provide a valid integer for the amount or number of clicks.");

    execute_command(command, value);
}
