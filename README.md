# Stilb Lightmap Baker

A GPU accelerated standalone lightmap baker for Unity, powered by Vulkan

## Notes

- Currently requires a GPU with `VK_KHR_ray_query` extension, however it will support any GPU with a software fallback in the future. Check GPU support here `https://vulkan.gpuinfo.org/listdevices.php`, most modern GPUs should work.
- While the lightmapper is fully working, it is still in early stages, theres room for improvement and it might lack some features

## Features

- Works on Windows and Linux
- Fast hardware accelerated ray-tracing (can utilize RTX)
- Realtime Preview
- Denoiser
- Seam stiching with a least squares solver
- Light Probe baking (L2 Spherical Harmonics)
- UV Packing
- Physically correct
- Lightmap Groups
- Easy to use (aims to be mostly a drop in replacement)
- Small binary size
- Emissive materials, Directional, Spot and Point Lights
- Shadow radius
- Fully standalone, with Unity URP and Built-In pipeline support

## How to use

### Denoiser Setup

#### Windows

1. Download the Windows `.zip` from `https://github.com/RenderKit/oidn/releases`
2. Extract it anywhere on your computer (e.g. `C:\oidn`)
3. Set the `OpenImageDenoise_DIR` environment variable to that extracted folder:
   - Press **Start**, type **"environment variables"**, and open **"Edit environment variables for your account"**
   - Click **New...**
   - Name: `OpenImageDenoise_DIR`
   - Value: the path to the extracted folder (e.g. `C:\oidn`)
   - Click **OK** on all windows

#### Linux
- Fedora Linux: `sudo dnf install oidn`

### Baking

- Setup the scene (mark objects as static, generate lightmap uvs, add lights with baked mode etc.)
- Menu Item `Stilb > Bake`
- Adjust settings and press `Generate Lighting`

## Stack

- Written in Rust, using the lightweight Ash vulkan crate, with minimal dependencies
- Shaders written in Slang

## Building

- Add the [slang](https://github.com/shader-slang/slang) compiler at PATH
- `cargo build --release`
