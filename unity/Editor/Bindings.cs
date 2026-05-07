using System;
using System.Drawing.Drawing2D;
using System.Runtime.InteropServices;
using UnityEngine;

namespace stilb
{
    public static class Bindings
    {
        public enum CoordinateSystem : uint
        {
            Default = 0,
            Unity = 1,
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct StilbConfig
        {
            public CoordinateSystem coordinate_system;

            [MarshalAs(UnmanagedType.I1)]
            public bool is_preview;
            public uint throttle_preview_ms;
            public LightmapSettings preview_settings;

            public Vector3 camera_position;
            public Vector3 camera_forward;

            public ReadbackCallback callback;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct LightmapSettings
        {
            public uint width;
            public uint height;

            public uint max_samples;
            public uint bounce_count;

            [MarshalAs(UnmanagedType.I1)]
            public bool denoise;
        }

        const string DLL_NAME = "stilb";

        [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr app_new(StilbConfig config);

        [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern void app_run(IntPtr app);

        [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern void app_destroy(IntPtr app);

        [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern void app_add_mesh(IntPtr app, Mesh mesh);

        [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern void app_add_light(IntPtr app, Light light);

        [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static unsafe extern void app_add_lightmap_group(IntPtr app, LightmapSettings settings, byte* albedoPixels, uint albedoPixelsLength, float* emissionPixels, uint emissionPixelsLength);

        [StructLayout(LayoutKind.Sequential)]
        public unsafe struct Mesh
        {
            public Vector3* vertices;
            public Vector3* normals;
            public Vector2* uvs;
            public uint* indices;

            public uint vertices_length;
            public uint indices_length;
            public uint lightmap_group;
        }

        public enum LightType : uint
        {
            Directional = 0,
            Point = 1,
            Spot = 2,
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct Light
        {
            public LightType ty;
            public Vector3 position;

            public Vector3 direction;
            public float range;

            public Vector3 color;
            public float shadow_radius_or_angle;
        }

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        public delegate void ReadbackCallback(ReadbackData data);

        [StructLayout(LayoutKind.Sequential)]
        public struct ReadbackData
        {
            public uint group_index;
            public uint ty;
            public uint width;
            public uint height;
            public IntPtr pixels;
            public uint pixels_count;

            public unsafe Color[] GetPixels()
            {
                if (pixels == IntPtr.Zero || pixels_count == 0)
                    return Array.Empty<Color>();

                int colorCount = (int)pixels_count / 4;
                Color[] managedArray = new Color[colorCount];

                fixed (Color* destPtr = managedArray)
                {
                    long byteCount = pixels_count * sizeof(float);
                    Buffer.MemoryCopy((void*)pixels, destPtr, byteCount, byteCount);
                }

                return managedArray;
            }
        }
    }
}