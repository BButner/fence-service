use tokio::spawn;
use windows::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
        CallNextHookEx, GetMessageA, PostQuitMessage, SetWindowsHookExW, UnhookWindowsHookEx,
        HHOOK, MOUSEHOOKSTRUCT, WH_MOUSE_LL, WM_QUIT,
    },
};

use crate::_is_active;

static mut MOUSE_HOOK: Option<HHOOK> = None;

pub fn start_mouse_hook() {
    spawn(async {
        unsafe {
            let result = SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_hook_callback), None, 0);

            match result {
                Ok(hook) => {
                    MOUSE_HOOK = Some(hook);
                    // set _is_active to true
                    _is_active = true;

                    println!("Hook installed: {:?}", hook);

                    while _is_active {
                        // Loop here so it reaches the callback method
                        let mut msg = std::mem::zeroed();
                        GetMessageA(&mut msg, None, 0, 0);
                    }

                    println!("Hook uninstalled");

                    UnhookWindowsHookEx(hook);
                }
                Err(e) => println!("Error: {:?}", e),
            }
        }
    });
}

pub async fn stop_mouse_hook() {
    println!("Stopping mouse hook");
    unsafe {
        _is_active = false;
    }
}

unsafe extern "system" fn mouse_hook_callback(
    code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    let mouse_info = (l_param.0 as *const MOUSEHOOKSTRUCT).read();
    let mouse_x = mouse_info.pt.x;
    let mouse_y = mouse_info.pt.y;

    println!("Mouse x: {}, y: {}", mouse_x, mouse_y);

    if !_is_active {
        PostQuitMessage(0);
    }

    CallNextHookEx(MOUSE_HOOK, code, w_param, l_param)

    // let mut inside = false;

    // for display in _displays.iter() {
    //     if display.selected {
    //         if mouse_x >= display.left + 1
    //             && mouse_x <= display.left + (display.width - 1) as i32
    //             && mouse_y >= display.top + 1
    //             && mouse_y <= display.top + (display.height - 1) as i32
    //         {
    //             inside = true;
    //             break;
    //         }
    //     }
    // }

    // if inside {
    //     _last_good_x = mouse_x;
    //     _last_good_y = mouse_y;
    //     return LRESULT(0);
    // } else {
    //     let mut display = _displays.iter().find(|d| {
    //         _last_good_x >= d.left
    //             && _last_good_x <= d.left + d.width as i32
    //             && _last_good_y >= d.top
    //             && _last_good_y <= d.top + d.height as i32
    //     });

    //     if let Some(good_monitor) = display {
    //         let mut new_x = mouse_x;
    //         let mut new_y = mouse_y;

    //         if mouse_x < good_monitor.left {
    //             new_x = good_monitor.left;
    //         } else if mouse_x > good_monitor.left + good_monitor.width as i32 {
    //             new_x = good_monitor.left + good_monitor.width as i32;
    //         }

    //         if mouse_y < good_monitor.top {
    //             new_y = good_monitor.top;
    //         } else if mouse_y > good_monitor.top + good_monitor.height as i32 {
    //             new_y = good_monitor.top + good_monitor.height as i32;
    //         }

    //         SetCursorPos(new_x, new_y);
    //     } else {
    //         SetCursorPos(_last_good_x, _last_good_y);
    //     }

    //     return LRESULT(1);
    // }
}
