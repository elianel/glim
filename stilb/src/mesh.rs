use crate::math::*;
use core::slice;

#[repr(C)]
pub struct RawMesh {
    pub vertices: *const Vector3,
    pub normals: *const Vector3,
    pub uvs: *const Vector2,
    pub indices: *const u32,
    pub vertices_length: u32,
    pub indices_length: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct Vertex {
    position: Vector3,
    normal: Vector3,
    uv: Vector2,
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<u32>,
}

impl Mesh {
    pub fn from_raw_mesh(raw: RawMesh) -> Self {
        let vertices = unsafe { slice::from_raw_parts(raw.vertices, raw.vertices_length as usize) };
        let normals = unsafe { slice::from_raw_parts(raw.normals, raw.vertices_length as usize) };
        let uvs = unsafe { slice::from_raw_parts(raw.uvs, raw.vertices_length as usize) };
        let indices = unsafe { slice::from_raw_parts(raw.indices, raw.indices_length as usize) };

        let mut vertices_copy = Vec::with_capacity(vertices.len());
        let mut triangles_copy = Vec::with_capacity(indices.len());

        for i in 0..vertices.len() {
            let vertex = Vertex {
                position: vertices[i],
                normal: normals[i],
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
