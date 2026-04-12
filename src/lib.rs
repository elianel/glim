use core::slice;

use crate::{
    math::{Vector2, Vector3},
    vulkan_core::{VulkanConfig, VulkanObjects, create_vulkan_objects},
};

mod bvh;
mod math;
mod tests;
mod vulkan_cmd;
mod vulkan_core;

pub struct Stilb {
    pub vk: VulkanObjects,
    pub meshes: Vec<Mesh>,
}

#[repr(C)]
pub struct StilbConfig {
    is_preview: u8,
    preview_width: u32,
    preview_height: u32,
}

#[repr(C)]
pub struct RawMesh {
    vertices: *const Vector3,
    normals: *const Vector3,
    uvs: *const Vector2,
    indices: *const u32,
    vertices_length: u32,
    indices_length: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct Vertex {
    position: Vector3,
    padding: u32,
    normal_octahedron: Vector2,
    uv: Vector2,
}

#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    triangles: Vec<u32>,
}

impl Mesh {
    pub fn pack_normal_octahedron(n: Vector3) -> Vector2 {
        let l1_norm = n.x.abs() + n.y.abs() + n.z.abs();
        let mut x = n.x / l1_norm;
        let mut y = n.y / l1_norm;

        if n.z < 0.0 {
            let old_x = x;
            let old_y = y;
            x = (1.0 - old_y.abs()) * if old_x >= 0.0 { 1.0 } else { -1.0 };
            y = (1.0 - old_x.abs()) * if old_y >= 0.0 { 1.0 } else { -1.0 };
        }

        Vector2::new(x, y)
    }

    pub fn from_raw_mesh(raw: RawMesh) -> Self {
        let vertices = unsafe { slice::from_raw_parts(raw.vertices, raw.vertices_length as usize) };
        let normals = unsafe { slice::from_raw_parts(raw.normals, raw.vertices_length as usize) };
        let uvs = unsafe { slice::from_raw_parts(raw.uvs, raw.vertices_length as usize) };
        let indices = unsafe { slice::from_raw_parts(raw.indices, raw.indices_length as usize) };

        let mut vertices_copy = Vec::with_capacity(vertices.len());
        let mut triangles_copy = Vec::with_capacity(indices.len());

        for i in 0..vertices.len() {
            let normal = Mesh::pack_normal_octahedron(normals[i]);

            let vertex = Vertex {
                position: vertices[i],
                padding: 0,
                normal_octahedron: normal,
                uv: uvs[i],
            };

            vertices_copy.push(vertex);
        }

        triangles_copy.extend(indices);

        Self {
            vertices: vertices_copy,
            triangles: triangles_copy,
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn initialize(config: StilbConfig) -> *mut Stilb {
    let is_debug = cfg!(debug_assertions);

    let vulkan_config = VulkanConfig {
        enable_validation_layers: is_debug,
        enable_window: config.is_preview != 0,
        width: 512,
        height: 512,
    };

    let vk = create_vulkan_objects(&vulkan_config);
    println!("Vulkan Initialized");

    let stilb = Stilb {
        vk,
        meshes: Vec::new(),
    };

    Box::into_raw(Box::new(stilb))
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
