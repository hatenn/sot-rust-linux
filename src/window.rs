extern crate x11;

use std::ptr;
use std::ffi::CString;
use std::os::raw::c_int;
use x11::xlib;
use crate::settings::settings::SCREEN_WIDTH;

pub fn get_window_info(window_name : &str) -> Option<(f64, f64, i32, i32)> {
    unsafe {
        let display = xlib::XOpenDisplay(ptr::null());
        if display.is_null() {
            eprintln!("Cannot open display");
            return None;
        }

        let root_window = xlib::XDefaultRootWindow(display);
        let mut target_window = None;
        let window_title_to_find = CString::new(window_name).unwrap();

        unsafe fn search_window(display: *mut xlib::Display, window: xlib::Window, title_to_find: &CString) -> Option<xlib::Window> {
            let mut return_window = None;
            let mut root_return = 0;
            let mut parent_return = 0;
            let mut children_return = ptr::null_mut();
            let mut nchildren_return = 0;

            if xlib::XQueryTree(display, window, &mut root_return, &mut parent_return, &mut children_return, &mut nchildren_return) != 0 {
                for i in 0..nchildren_return {
                    let child = *children_return.offset(i as isize);

                    // Attempt to get the window title
                    let mut name = ptr::null_mut();
                    if xlib::XFetchName(display, child, &mut name) != 0 {
                        let title = CString::from_raw(name);
                        if title == *title_to_find {
                            return_window = Some(child);
                            break;
                        }
                    }

                    // Recurse into child windows
                    let child_result = search_window(display, child, title_to_find);
                    if child_result.is_some() {
                        return_window = child_result;
                        break;
                    }
                }
                if !children_return.is_null() {
                    xlib::XFree(children_return as *mut _);
                }
            }

            return_window
        }

        target_window = search_window(display, root_window, &window_title_to_find);

        if let Some(window) = target_window {
            let mut attr: xlib::XWindowAttributes = std::mem::zeroed();
            xlib::XGetWindowAttributes(display, window, &mut attr);

            // Now attr.x and attr.y contain the position of the window relative to its parent
            // For top-level windows, you often want to translate this position to root window coordinates
            let mut x_root = 0;
            let mut y_root = 0;
            let mut child_return: xlib::Window = 0;
            xlib::XTranslateCoordinates(display, window, root_window, attr.x, attr.y, &mut x_root, &mut y_root, &mut child_return);

            println!("Window Position: {}, {}", x_root, y_root);
            println!("Window Size: {}x{}", attr.width, attr.height);

            xlib::XCloseDisplay(display);

            return Some((attr.width as f64, attr.height as f64, x_root as i32, y_root as i32));
        } else {
            println!("Window not found.");
            xlib::XCloseDisplay(display);
        }

    }

    return None;
}