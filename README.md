# kbscroll

`kbscroll` is a tiny (<20kb) utility written in Rust that allows you to send mousewheel scroll events through the terminal.

## Features
- Tiny
- Inertial Scrolling // TODO
- Horizontal Scrolling // TODO

## Installation

```powershell
scoop bucket add santarl_scoop_bucket https://github.com/santarl/scoop_bucket ; 
scoop install santarl_scoop_bucket/kbscroll
```
or

```powershell
scoop install https://raw.githubusercontent.com/santarl/kbscroll/refs/heads/main/kbscroll.json
```

or

download [kbscroll.exe](https://github.com/santarl/kbscroll/releases/download/main/kbscroll.exe) from [releases](https://github.com/santarl/kbscroll/releases)

## Usage

To use `kbscroll`, run in terminal:

```sh
kbscroll <amount>
```

## Why?

I wanted something with which I can navigate and browse the computer with vim bindings without using a mouse/trackpad but exisitng options in windows (autohotkey, pyautogui) etc were too heavy. Arrow keys do not always scroll on websites (and certain apps)

## License

This project is licensed under the MIT License.
