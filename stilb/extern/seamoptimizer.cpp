#define SEAMOPTIMIZER_IMPLEMENTATION
#include "seamoptimizer.h"

extern "C"
{

    so_seam_t *so_seams_find_c(
        float *positions,
        float *texcoords,
        int vertices,
        float cos_normal_threshold,
        float *data,
        int w,
        int h,
        int c)
    {
        return so_seams_find(
            positions,
            texcoords,
            vertices,
            cos_normal_threshold,
            data,
            w,
            h,
            c);
    }

    int so_seam_optimize_c(
        so_seam_t *seam,
        float *data,
        int w,
        int h,
        int c,
        float lambda)
    {
        return so_seam_optimize(seam, data, w, h, c, lambda);
    }

    so_seam_t *so_seam_next_c(so_seam_t *seam)
    {
        return so_seam_next(seam);
    }

    void so_seams_free_c(so_seam_t *seams)
    {
        so_seams_free(seams);
    }
}
