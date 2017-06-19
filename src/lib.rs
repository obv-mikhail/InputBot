extern crate winapi;
extern crate user32;

use winapi::{MSLLHOOKSTRUCT, MOUSEINPUT, HWND, MSG, INPUT, KEYBDINPUT, c_int, WPARAM, LPARAM, LRESULT, HINSTANCE, HHOOK, KBDLLHOOKSTRUCT};
use user32::{GetAsyncKeyState, UnhookWindowsHookEx, PostMessageA, GetMessageW, GetKeyState, MapVirtualKeyA, SendInput, SetWindowsHookExA, CallNextHookEx};
use std::mem::{transmute, size_of, uninitialized};

pub mod vk;

pub type VKCode = u8;
pub type X = i32;
pub type Y = i32;
pub type ScrollAmount = u32;

#[derive(Debug)] pub enum Input {Keybd(KeybdInput, VKCode), Mouse(MouseInput, X, Y)}
#[derive(Debug)] pub enum KeybdInput {Press, Release}
#[derive(Debug)] pub enum MouseInput {PressLeft, ReleaseLeft, PressRight, ReleaseRight, PressMiddle, ReleaseMiddle, Move, Scroll(ScrollAmount)}

use Input::*;
use KeybdInput::*;
use MouseInput::*;


impl Input {
    /// Sends an input.
    ///
    /// #Example
    /// ```
    /// use inputbot::Input::{Keybd, Mouse};
    /// use inputbot::KeybdInput::{Press, Release};
    /// use inputbot::MouseInput::{PressLeft, ReleaseLeft};
    /// 
    /// // Toggle NumLock
    /// Keybd(Press, 144);
    /// Keybd(Release, 144);
    ///
    /// // Click left mouse button
    /// Mouse(PressLeft, 0, 0);
    /// Mouse(ReleaseLeft, 0, 0);
    /// ```
    pub fn send(self) {
        let mut _input: INPUT = match self {
            Keybd(keybd_input, vk_code) => INPUT {
                type_: 1, 
                u: unsafe{transmute((KEYBDINPUT{
                    wVk: 0,
                    wScan: MapVirtualKeyA(vk_code as u32, 0) as u16,
                    dwFlags: match keybd_input {Press => 0x0008, Release => 0x0008 | 0x0002},
                    time: 0,
                    dwExtraInfo: 0
                }, 0 as u32))}
            },
            Mouse(mouse_input, x, y) => INPUT {
                type_: 0, 
                u: unsafe{transmute(MOUSEINPUT{
                    dx: x,
                    dy: y,
                    mouseData: match mouse_input {Scroll(scroll_amount) => scroll_amount, _ => 0},
                    dwFlags: 0x0001 | match mouse_input {
                        PressLeft => 0x0002,
                        ReleaseLeft => 0x0004,
                        PressMiddle => 0x0020,
                        ReleaseMiddle => 0x0040,
                        PressRight => 0x0008,
                        ReleaseRight => 0x0010,
                        Move => 0x0001,
                        Scroll(_) => 0x0800,
                    },
                    time: 0,
                    dwExtraInfo: 0
                })}
            }
        };
        unsafe{SendInput(1, &mut _input, size_of::<INPUT>() as i32)};
    }
}

static mut KEYBD_HHOOK: HHOOK = 0 as HHOOK;
static mut MOUSE_HHOOK: HHOOK = 0 as HHOOK;

unsafe extern "system" fn hhook_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    PostMessageA(0 as HWND, 0, w_param, l_param);
    CallNextHookEx(KEYBD_HHOOK, code, w_param, l_param)
}

/// The function captures and returns inputs detected anywhere on the system.
///
/// #Example
/// ```
/// use inputbot::capture_input;
/// use inputbot::Input::Keybd;
/// use inputbot::KeybdInput::Release;
///
/// while let Some(input) = capture_input() {
///  match input {
///   //Exit if NumLock gets pressed
///   Keybd(Release, 144) => break,
///   //Log all inputs
///   _ => println!("{:?}", input)
///  }
/// }
/// ```
/// Input handling can be done in a new thread if multiple inputs need to be handled simultaneously.
pub fn capture_input() -> Option<Input> {
    // The function resets hooks during every call to prevent capture emulated input.
    unsafe{KEYBD_HHOOK = SetWindowsHookExA(13, Some(hhook_proc), 0 as HINSTANCE, 0)};
    unsafe{MOUSE_HHOOK = SetWindowsHookExA(14, Some(hhook_proc), 0 as HINSTANCE, 0)};
    let mut msg: MSG = unsafe{uninitialized()};
    if unsafe {GetMessageW(&mut msg, 0 as HWND, 0, 0)} <= 0 {return None}
    unsafe{UnhookWindowsHookEx(KEYBD_HHOOK)};
    unsafe{UnhookWindowsHookEx(MOUSE_HHOOK)};
    // Windows Message Codes can be found here: https://wiki.winehq.org/List_Of_Windows_Messages
    match msg.wParam {
        256 => Some(Keybd(Press, unsafe{*(msg.lParam as *const KBDLLHOOKSTRUCT)}.vkCode as u8)),
        257 => Some(Keybd(Release, unsafe{*(msg.lParam as *const KBDLLHOOKSTRUCT)}.vkCode as u8)),
        512...522 => {
            let x = unsafe{*(msg.lParam as *const MSLLHOOKSTRUCT)}.pt.x;
            let y = unsafe{*(msg.lParam as *const MSLLHOOKSTRUCT)}.pt.y;
            match msg.wParam {
                512 => Some(Mouse(Move, x, y)),
                513 => Some(Mouse(PressLeft, x, y)),
                514 => Some(Mouse(ReleaseLeft, x, y)),
                516 => Some(Mouse(PressRight, x, y)),
                517 => Some(Mouse(ReleaseRight, x, y)),
                519 => Some(Mouse(PressMiddle, x, y)),
                520 => Some(Mouse(ReleaseMiddle, x, y)),
                522 => Some(Mouse(Scroll(0), x, y)),
                _ => None
            }
        },
        _ => None
    }
}

/// For NumLock, CapsLock, and ScrollLock, returns the toggle state.
pub fn get_toggle_state(vk_code: VKCode) -> bool {
    unsafe {GetKeyState(vk_code as i32) & 15 != 0}
}

/// Returns the logical state (pressed | not pressed).
pub fn get_key_state(vk_code: VKCode) -> bool {
    let state = unsafe {GetAsyncKeyState(vk_code as i32)};
    match state {
        -32767 | -32768 => true,
        _ => false
    }
}