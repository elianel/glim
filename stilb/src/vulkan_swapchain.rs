use ash::vk;

use crate::vulkan_core::VulkanContext;

pub struct SwapchainData {}

fn query_swapchain_support(
    vk: &VulkanContext,
) -> (
    vk::SurfaceCapabilitiesKHR,
    Vec<vk::SurfaceFormatKHR>,
    Vec<vk::PresentModeKHR>,
) {
    let capabilities = unsafe {
        vk.surface_instance
            .get_physical_device_surface_capabilities(vk.physical_device, vk.surface)
            .unwrap()
    };

    let formats = unsafe {
        vk.surface_instance
            .get_physical_device_surface_formats(vk.physical_device, vk.surface)
            .unwrap()
    };

    let present_modes = unsafe {
        vk.surface_instance
            .get_physical_device_surface_present_modes(vk.physical_device, vk.surface)
            .unwrap()
    };

    (capabilities, formats, present_modes)
}

impl VulkanContext {
    pub fn create_swapchain(&self, width: u32, height: u32) -> SwapchainData {
        let (capabilities, formats, present_modes) = query_swapchain_support(self);

        let mut selected_format = formats[0];
        for format in formats {
            if format.format == vk::Format::B8G8R8A8_SRGB
                && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            {
                selected_format = format;
                break;
            }
        }

        let mut selected_present_mode = present_modes[0];
        for mode in present_modes {
            if mode == vk::PresentModeKHR::MAILBOX {
                selected_present_mode = mode;
                break;
            }
        }

        let extent = if capabilities.current_extent.width == u32::MAX {
            capabilities.current_extent
        } else {
            vk::Extent2D {
                width: width.clamp(
                    capabilities.min_image_extent.width,
                    capabilities.max_image_extent.width,
                ),
                height: height.clamp(
                    capabilities.min_image_extent.height,
                    capabilities.max_image_extent.height,
                ),
            }
        };

        let image_count = 3.clamp(capabilities.min_image_count, capabilities.max_image_count);

        println!(
            "images: {:?}\nextent: {:?}\nformat: {:?}\nmode: {:?}",
            image_count, extent, selected_format, selected_present_mode
        );

        SwapchainData {}
    }
}
