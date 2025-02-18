use std::env;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use winapi::um::winuser::{
    SendInput, INPUT, INPUT_MOUSE, MOUSEEVENTF_HWHEEL, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
    MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP,
    MOUSEEVENTF_WHEEL, MOUSEEVENTF_XDOWN, MOUSEEVENTF_XUP, MOUSEINPUT,
};

const XBUTTON1: u32 = 0x0001;
const XBUTTON2: u32 = 0x0002;

fn scroll_mouse(amount: i32, horizontal: bool) {
    let dw_flags = if horizontal {
        MOUSEEVENTF_HWHEEL
    } else {
        MOUSEEVENTF_WHEEL
    };

    let mouse_input = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: amount as u32,
        dwFlags: dw_flags,
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

fn click_mouse_button(button_down: u32, button_up: u32, mouse_data: u32, times: u32) {
    for _ in 0..times {
        mouse_event(button_down, mouse_data);
        mouse_event(button_up, mouse_data);
        sleep(Duration::from_millis(100));
    }
}

fn mouse_event(event: u32, mouse_data: u32) {
    let mouse_input = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: mouse_data,
        dwFlags: event,
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

fn click_left(times: u32) {
    click_mouse_button(MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, 0, times);
}

fn click_middle(times: u32) {
    click_mouse_button(MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, 0, times);
}

fn click_right(times: u32) {
    click_mouse_button(MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, 0, times);
}

fn click_back(times: u32) {
    click_mouse_button(MOUSEEVENTF_XDOWN, MOUSEEVENTF_XUP, XBUTTON1, times);
}

fn click_forward(times: u32) {
    click_mouse_button(MOUSEEVENTF_XDOWN, MOUSEEVENTF_XUP, XBUTTON2, times);
}

// Macro to define commands and aliases
macro_rules! commands {
    (
        $( $name:ident => [$($alias:expr),+] => $action:expr, $down_action:expr, $up_action:expr; $desc:expr; )*
    ) => {
        const COMMANDS: &[(&str, &[&str], fn(i32), fn(), fn(), &str)] = &[
            $(
                (
                    stringify!($name), // This converts the function name to a string
                    &[$($alias),+],
                    $action,
                    $down_action,
                    $up_action,
                    $desc
                ),
            )*
        ];
    };
}

// Define commands using the macro
commands! {
    scroll => ["scroll", "wheel", "sc", "wh", "vs", "s1"] => |v| scroll_mouse(v, false), || {}, || {}; "Scroll the mouse wheel up (+) or down (-) by the specified amount.";
    scroll_horizontal => ["scroll_horizontal", "hwheel", "sch", "whw", "hs", "s2"] => |v| scroll_mouse(v, true), || {}, || {}; "Scroll the mouse wheel left (-) or right (+) by the specified amount.";
    click_left => ["click_left", "lclick", "lc", "c1"] => |v| click_left(v as u32), || mouse_event(MOUSEEVENTF_LEFTDOWN, 0), || mouse_event(MOUSEEVENTF_LEFTUP, 0); "Perform left mouse clicks the specified number of times.";
    click_right => ["click_right", "rclick", "rc", "c2"] => |v| click_right(v as u32), || mouse_event(MOUSEEVENTF_RIGHTDOWN, 0), || mouse_event(MOUSEEVENTF_RIGHTUP, 0); "Perform right mouse clicks the specified number of times.";
    click_middle => ["click_middle", "mclick", "mc", "c3"] => |v| click_middle(v as u32), || mouse_event(MOUSEEVENTF_MIDDLEDOWN, 0), || mouse_event(MOUSEEVENTF_MIDDLEUP, 0); "Perform middle mouse clicks the specified number of times.";
    click_back => ["click_back", "back", "cb", "c4", "x1"] => |v| click_back(v as u32), || mouse_event(MOUSEEVENTF_XDOWN, XBUTTON1), || mouse_event(MOUSEEVENTF_XUP, XBUTTON1); "Simulate the browser's back button the specified number of times.";
    click_forward => ["click_forward", "forward", "cf", "c5", "x2"] => |v| click_forward(v as u32), || mouse_event(MOUSEEVENTF_XDOWN, XBUTTON2), || mouse_event(MOUSEEVENTF_XUP, XBUTTON2); "Simulate the browser's forward button the specified number of times.";
}

fn execute_command(command: &str, value: i32, modifier: Option<&str>) {
    let is_down = modifier == Some("+");
    let is_up = modifier == Some("-");

    let base_command = command.trim_end_matches(&['+', '-'][..]);

    for (_, aliases, action, down_action, up_action, _) in COMMANDS {
        if aliases.contains(&base_command) {
            if is_down {
                down_action();
            } else if is_up {
                up_action();
            } else {
                action(value);
            }
            return;
        }
    }
    print_usage();
}

fn print_usage() {
    eprintln!("Usage:");
    for (name, _, _, _, _, _) in COMMANDS {
        eprintln!("  kbscroll.exe {} amount", name);
    }
    eprintln!("Use: kbscroll.exe help command_name for extended help");
    std::process::exit(1);
}

fn print_help(command: &str) {
    for (name, aliases, _, _, _, desc) in COMMANDS {
        if aliases.contains(&command) {
            eprintln!("Command: {}", name);
            eprintln!("Aliases: [{}]", aliases.join(", "));
            eprintln!("Description: {}", desc);
            eprintln!("Usage: kbscroll.exe {} amount", name);
            return;
        }
    }
    eprintln!("Unknown command: {}", command);
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
    }

    let command = &args[1];

    if command == "help" {
        if args.len() != 3 {
            eprintln!("Usage: kbscroll.exe help function_name");
            std::process::exit(1);
        }
        print_help(&args[2]);
    } else {
        let modifier = if args.len() == 4 {
            let modifier_str = &args[3];
            if modifier_str == "+" || modifier_str == "-" {
                Some(modifier_str)
            } else {
                None
            }
        } else {
            None
        };

        let value = match args.get(2) {
            Some(value_str) if modifier.is_none() => {
                i32::from_str(value_str).unwrap_or_else(|_| {
                    eprintln!("Please provide a valid integer for the amount or number of clicks.");
                    std::process::exit(1);
                })
            },
            _ => 1,
        };

        execute_command(command, value, modifier.map(|x| x.as_str()));
    }
}