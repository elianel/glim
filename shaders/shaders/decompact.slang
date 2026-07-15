#include "includes/core.slang"
#include "includes/util.slang"

[[vk::push_constant]]
PushConstants Push;

struct PushConstants
{
    uint2 resolution;
    uint unused;
    uint compactedCount;

    uint lightmapType;
    uint groupIndex;
    uint pad1;
    uint pad2;
};

[numthreads(8, 8, 1)] void main(uint3 threadID : SV_DispatchThreadID)
{
    uint2 coords = threadID.xy;

    if (coords.x >= Push.resolution.x || coords.y >= Push.resolution.y)
    {
        return;
    }

    uint linearIndex = coords.y * Push.resolution.x + coords.x;
    uint compactIndex = GetCompactedIndex(float2(coords), Push.groupIndex, false);
    bool active = compactIndex != uint::maxValue;

    if (!active)
    {
        DecompactTarget[linearIndex] = 0.0;
        return;
    }

    float3 diffuse = float3(
        CompactedLightmap[compactIndex * 3 + 0],
        CompactedLightmap[compactIndex * 3 + 1],
        CompactedLightmap[compactIndex * 3 + 2]);

    float4 visibility = CompactedVisibility[compactIndex];
    float3 normalWS = normalize(UnpackNormalOctahedron(visibility.w));

    [forcecase] switch (Push.lightmapType)
    {
    case 0:
    {
        DecompactTarget[linearIndex] = float4(diffuse, 1.0);
        break;
    }
    case 1:
    {
        uint compactIndex = compactIndex + Push.compactedCount;
        float luminance = Luminance(diffuse);

        float3 directional = float3(
            CompactedLightmap[compactIndex * 3 + 0],
            CompactedLightmap[compactIndex * 3 + 1],
            CompactedLightmap[compactIndex * 3 + 2]);

        float3 normalizedDirection = normalize(directional / luminance);

        DecompactTarget[linearIndex] = float4(normalizedDirection, 1.0);
        break;
    }
    case 2:
    {
        uint compactIndex = compactIndex + Push.compactedCount;

        float4 directionalLightmap = DecompactTarget[linearIndex];
        float3 normalizedDirection = directionalLightmap.rgb;

        float NoL = saturate(dot(normalizedDirection, normalWS));

        normalizedDirection.z = -normalizedDirection.z; // unity convention
        DecompactTarget[linearIndex] = mad(float4(normalizedDirection, NoL), 0.5, 0.5);
        break;
    }
    }
}
