use std::sync::atomic::{AtomicU64, Ordering};

use crate::vulkan_context::VulkanContext;
use ash::vk::{self, Handle};

static ALLOCATED_GPU_MEMORY: AtomicU64 = AtomicU64::new(0);

fn register_gpu_alloc(bytes: u64) -> f64 {
    let val = ALLOCATED_GPU_MEMORY.fetch_add(bytes, Ordering::Relaxed) + bytes;

    let mb = val as f64 / (1024.0 * 1024.0);
    mb
}

fn unregister_gpu_alloc(bytes: u64) {
    ALLOCATED_GPU_MEMORY.fetch_sub(bytes, Ordering::Relaxed);
}

pub struct Buffer {
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory,
    pub address: vk::DeviceAddress,

    bytes: u64,
    name: String,
}

impl Buffer {
    pub fn null() -> Self {
        Self {
            buffer: vk::Buffer::null(),
            memory: vk::DeviceMemory::null(),
            address: 0,
            bytes: 0,
            name: String::new(),
        }
    }

    pub fn new<T>(
        vk: &VulkanContext,
        name: String,
        bytes: &[T],
        usage: vk::BufferUsageFlags,
    ) -> Self {
        let size = (bytes.len() * std::mem::size_of::<T>()) as vk::DeviceSize;
        let (buffer, memory, mem_reqs) =
            vk.create_buffer(size, usage, vk::MemoryPropertyFlags::DEVICE_LOCAL);

        let info = vk::BufferDeviceAddressInfo {
            buffer,
            ..Default::default()
        };

        let address = unsafe { vk.device.get_buffer_device_address(&info) };

        let (a, bytes, b) = unsafe { bytes.align_to::<u8>() };

        assert!(a.len() == 0);
        assert!(b.len() == 0);

        vk.upload_buffer(bytes, buffer);

        let allocated = register_gpu_alloc(mem_reqs.size);

        println!(
            "Created Buffer '{:#x}' VRAM: {:.2} MiB ({})",
            buffer.as_raw(),
            allocated,
            &name,
        );

        Self {
            buffer,
            memory,
            address,
            bytes: size,
            name,
        }
    }

    pub fn destroy(&mut self, vk: &VulkanContext) {
        debug_assert!(!self.buffer.is_null());
        debug_assert!(!self.memory.is_null());

        unsafe {
            vk.device.destroy_buffer(self.buffer, None);
            if !self.memory.is_null() {
                unregister_gpu_alloc(self.bytes);
                vk.device.free_memory(self.memory, None);
            }
        };

        self.buffer = vk::Buffer::null();
        self.memory = vk::DeviceMemory::null();
        self.address = 0;
    }
}
