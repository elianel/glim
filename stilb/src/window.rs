use std::{
    ffi::{CStr, c_int},
    ptr,
};

use glfw_sys::*;

pub fn create_window(width: u32, height: u32) -> *mut GLFWwindow {
    const TITLE: &CStr = c"Stilb Preview";
    let width = width as c_int;
    let height = height as c_int;

    unsafe {
        glfwInit();

        glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
        glfwWindowHint(GLFW_RESIZABLE, GLFW_TRUE);

        let window = glfwCreateWindow(
            width,
            height,
            TITLE.as_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        return window;
    }
}
