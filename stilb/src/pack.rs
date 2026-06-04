// split mesh into charts or treat entire mesh as one chart per renderer and use scale offset for uvs
// calculate a scale multipler for each chart based on world space scale of the object https://github.com/z3y/XatlasLightmap/blob/main/Scripts/XatlasLightmapPacker.cs#L495
// multiply with scale in lightmap property and scale the uvs
// calculate area sum of all charts and use as a maximum
// sort charts by area from largest to smallest
// calculate bounds for each chart
// find an approximate float (something like 75% to 100% coverage) to scale all charts to fit inside area of lightmap texture in texel units (256 x 256 = 65536.0)
// rasterize each uv chart into a bitmap
// pack
// if everything fits repeat with larger approximation or stop and scale charts back into [0, 1] uv range

use crate::math::{Vector2, Vector3};

struct UVPacker {
    charts: Vec<Chart>,
    width: u32,
    height: u32,
}

struct Chart {
    uvs: Vec<Vector2>,
    positions: Vec<Vector3>,
    indices: Vec<u32>,

    original_indices: Vec<u32>,
    mesh_id: usize,

    uv_area: f64,
}

impl Chart {
    fn calculate_area_multiplier(&self) -> f32 {
        let mut uv_area = 0.0;
        let mut world_area = 0.0;

        // todo can be faster in parallel
        for chunk in self.indices.chunks_exact(3) {
            let index_a = chunk[0] as usize;
            let index_b = chunk[1] as usize;
            let index_c = chunk[2] as usize;

            let v1 = self.positions[index_a];
            let v2 = self.positions[index_b];
            let v3 = self.positions[index_c];

            world_area += (v2 - v1).cross(v3 - v1).length() as f64;

            let u1 = self.uvs[index_a];
            let u2 = self.uvs[index_b];
            let u3 = self.uvs[index_c];

            let d = determinant(u1, u2, u3);
            uv_area += d.abs() as f64;
        }

        (world_area.sqrt() / uv_area.sqrt()) as f32
    }

    fn calculate_uv_area(&self) -> f64 {
        let mut uv_area = 0.0;

        // todo can be faster in parallel
        for chunk in self.indices.chunks_exact(3) {
            let index_a = chunk[0] as usize;
            let index_b = chunk[1] as usize;
            let index_c = chunk[2] as usize;

            let u1 = self.uvs[index_a];
            let u2 = self.uvs[index_b];
            let u3 = self.uvs[index_c];

            let d = determinant(u1, u2, u3);
            uv_area += d.abs() as f64;
        }

        uv_area.sqrt()
    }
}

fn determinant(c: Vector2, c2: Vector2, c3: Vector2) -> f32 {
    let num = c2.y - c3.y;
    let num2 = c.y - c3.y;
    let num3 = c.y - c2.y;
    c.x * num - c2.x * num2 + c3.x * num3
}

impl UVPacker {
    fn new() -> Self {
        Self {
            charts: Vec::new(),
            width: 256,
            height: 256,
        }
    }

    // mesh with applied transform
    pub fn add_mesh(
        &mut self,
        positions: &[Vector3],
        uvs: &[Vector2],
        indices: &[u32],
        scale_multiplier: f32,
        mesh_id: usize,
    ) {
        // todo split into charts for non scale offset mode

        let mut chart = Chart {
            uvs: uvs.to_vec(),
            indices: indices.to_vec(),
            positions: positions.to_vec(),
            mesh_id,
            original_indices: indices.to_vec(),
            uv_area: 0.0,
        };

        let mut scale = chart.calculate_area_multiplier();
        scale *= scale_multiplier;

        chart.uvs.iter_mut().for_each(|x| *x *= scale);

        chart.uv_area = chart.calculate_uv_area();

        self.charts.push(chart);
    }
}
