use std::ptr;

use ash::vk::Handle;

use glfw_sys::{
    GLFW_KEY_ESCAPE, GLFW_PRESS, GLFWwindow, glfwCreateWindowSurface, glfwGetKey,
    glfwGetRequiredInstanceExtensions, glfwPollEvents, glfwSetWindowShouldClose,
    glfwWindowShouldClose,
};

use crate::{
    mesh::{Mesh, RawMesh},
    vulkan_core::{VulkanConfig, VulkanContext},
    window::create_window,
};

mod math;
mod mesh;
mod tests;
mod vulkan_cmd;
mod vulkan_core;
mod window;

pub struct Stilb {
    pub vk: VulkanContext,
    pub meshes: Vec<Mesh>,
    pub window: *mut GLFWwindow,
}

#[repr(C)]
pub struct StilbConfig {
    is_preview: u8,
    preview_width: u32,
    preview_height: u32,
}

#[unsafe(no_mangle)]
pub extern "C" fn initialize(config: StilbConfig) -> *mut Stilb {
    let is_debug = cfg!(debug_assertions);

    let mut vulkan_config = VulkanConfig {
        enable_validation_layers: is_debug,
        enable_window: config.is_preview != 0,
        window_extensions: Vec::new(),
        width: 512,
        height: 512,
    };

    let window = initialize_window(config, &mut vulkan_config);

    let create_surface_callback = |instance: &ash::Instance| unsafe {
        let instance = instance.handle().as_raw() as glfw_sys::VkInstance;
        let mut surface: glfw_sys::VkSurfaceKHR = ptr::null_mut();
        glfwCreateWindowSurface(instance, window, std::ptr::null(), &mut surface);
        ash::vk::SurfaceKHR::from_raw(surface as u64)
    };

    let vk = VulkanContext::new(&vulkan_config, create_surface_callback);
    println!("Vulkan Initialized");

    let stilb = Stilb {
        vk,
        meshes: Vec::new(),
        window: window,
    };

    Box::into_raw(Box::new(stilb))
}

fn initialize_window(config: StilbConfig, vulkan_config: &mut VulkanConfig) -> *mut GLFWwindow {
    let mut window = ptr::null_mut();
    if vulkan_config.enable_window {
        window = create_window(config.preview_width, config.preview_height);

        unsafe {
            let mut window_extensions_count: u32 = 0;
            let window_extensions = glfwGetRequiredInstanceExtensions(&mut window_extensions_count);

            for i in 0..window_extensions_count {
                let str = *window_extensions.add(i as usize);
                vulkan_config.window_extensions.push(str);
            }

            while glfwWindowShouldClose(window) == 0 {
                glfwPollEvents();

                if glfwGetKey(window, GLFW_KEY_ESCAPE) == GLFW_PRESS {
                    glfwSetWindowShouldClose(window, 1);
                    println!("ESC")
                }
            }
        }
    }
    window
}

#[unsafe(no_mangle)]
pub extern "C" fn deinitialize(stilb: *mut Stilb) {
    if !stilb.is_null() {
        // Take ownership back from the pointer and let Box drop it
        let _ = unsafe { Box::from_raw(stilb) };
        println!("Stilb destroyed");
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn add_mesh(stilb: *mut Stilb, raw: RawMesh) {
    unsafe {
        let stilb_obj = &mut *stilb;

        let mesh = Mesh::from_raw_mesh(raw);

        // println!("Added mesh: {:#?}", mesh);

        stilb_obj.meshes.push(mesh);
    }
}
