use std::os::raw::c_int;

enum SoSeam {}

unsafe extern "C" {
    fn so_seams_find_c(
        positions: *mut f32,
        texcoords: *mut f32,
        vertices: c_int,
        cos_normal_threshold: f32,
        data: *mut f32,
        w: c_int,
        h: c_int,
        c: c_int,
    ) -> *mut SoSeam;

    fn so_seam_optimize_c(
        seam: *mut SoSeam,
        data: *mut f32,
        w: c_int,
        h: c_int,
        c: c_int,
        lambda: f32,
    ) -> c_int;

    fn so_seam_next_c(seam: *mut SoSeam) -> *mut SoSeam;
    fn so_seams_free_c(seams: *mut SoSeam);
}

pub struct Seams {
    head: *mut SoSeam,
    width: c_int,
    height: c_int,
}

// so_seam_optimize is explicitly documented as safe to call in parallel
// on separate seams, and we never mutate the seam structure itself
unsafe impl Send for Seams {}
unsafe impl Sync for Seams {}

impl Seams {
    /// Call once per mesh. Positions and texcoords are expanded triangle soup
    /// (already indexed out), pixels is any representative RGBA f32 buffer —
    /// only used for fill_with_closest on empty edge texels, which you can
    /// ignore since you have your own padding.
    pub fn find(
        flat_positions: &mut [f32],
        flat_texcoords: &mut [f32],
        pixels: &mut [f32],
        width: u32,
        height: u32,
        cos_normal_threshold: f32,
    ) -> Self {
        assert!(
            flat_positions.len() % 9 == 0,
            "positions must be triangle soup (9 floats per tri)"
        );
        assert_eq!(flat_positions.len() / 3, flat_texcoords.len() / 2);

        let vertices =
            c_int::try_from(flat_positions.len() / 3).expect("vertex count overflows c_int");

        let head = unsafe {
            so_seams_find_c(
                flat_positions.as_mut_ptr(),
                flat_texcoords.as_mut_ptr(),
                vertices,
                cos_normal_threshold,
                pixels.as_mut_ptr(),
                width as c_int,
                height as c_int,
                4,
            )
        };

        Self {
            head,
            width: width as c_int,
            height: height as c_int,
        }
    }

    /// Call for each lightmap. pixels is a flat RGBA f32 buffer, row-major.
    pub fn optimize(&self, pixels: &mut [f32], lambda: f32) {
        assert_eq!(pixels.len(), (self.width * self.height * 4) as usize);

        unsafe {
            let mut seam = self.head;
            while !seam.is_null() {
                so_seam_optimize_c(
                    seam,
                    pixels.as_mut_ptr(),
                    self.width,
                    self.height,
                    4,
                    lambda,
                );
                seam = so_seam_next_c(seam);
            }
        }
    }
}

impl Drop for Seams {
    fn drop(&mut self) {
        unsafe {
            so_seams_free_c(self.head);
        }
    }
}
