[[vk::image_format("rgba32f")]]
[[vk::binding(0, 0)]]
RWTexture2D<float4> TestTexture;

[shader("compute")]
[numthreads(8, 8, 1)]
void main(uint3 threadID: SV_DispatchThreadID)
{
    uint2 coords = threadID.xy;

    TestTexture[coords] = float4(0, 1, 0, 0);
}
