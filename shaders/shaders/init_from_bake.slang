#include "includes/core.slang"

[[vk::push_constant]]
PushConstants Push;

struct PushConstants
{
    uint width;
    uint height;
    uint groupIndex;
    uint convervative;
};

struct Varyings
{
    float4 positionCS : SV_Position;
    nointerpolation uint primitiveID : TEXCOORD1;
};

[shader("vertex")] Varyings vertex(uint vertexID : SV_VulkanVertexID)
{
    Varyings o;

    uint primitiveID = vertexID / 3;
    uint index = vertexID % 3;

    uint vIndex = Indices[primitiveID * 3 + index];
    Vertex v = Vertices[vIndex];

    if (v.GetLightmapGroup() != Push.groupIndex)
    {
        o.positionCS = -1;
        o.primitiveID = 0;
    }
    else
    {
        float2 resolution = float2(Push.width, Push.height);
        float2 uv = v.GetUV() / resolution;

        o.positionCS = float4(uv.x * 2.0 - 1.0, (1.0 - uv.y) * 2.0 - 1.0, 0.0, 1.0);
        o.primitiveID = primitiveID;
    }

    return o;
}

float2 ClosestPointSegment(float2 p, float2 a, float2 b, out float t)
{
    float2 ab = b - a;

    t = dot(p - a, ab) / dot(ab, ab);
    t = saturate(t);

    return a + t * ab;
}

float3 ClampBarycentric(float2 p, float2 a, float2 b, float2 c)
{
    float bestDist = 1e30;
    float3 bestBary = 0;

    float t;

    // AB edge
    float2 q = ClosestPointSegment(p, a, b, t);
    float d = dot(p - q, p - q);

    if (d < bestDist)
    {
        bestDist = d;
        bestBary = float3(1 - t, t, 0);
    }

    // AC edge
    q = ClosestPointSegment(p, a, c, t);
    d = dot(p - q, p - q);

    if (d < bestDist)
    {
        bestDist = d;
        bestBary = float3(1 - t, 0, t);
    }

    // BC edge
    q = ClosestPointSegment(p, b, c, t);
    d = dot(p - q, p - q);

    if (d < bestDist)
    {
        bestDist = d;
        bestBary = float3(0, 1 - t, t);
    }

    return bestBary;
}

[shader("fragment")] float4 fragment(Varyings varyings)
    : SV_Target
{
    uint primitiveID = varyings.primitiveID;

    uint i0 = Indices[primitiveID * 3 + 0];
    uint i1 = Indices[primitiveID * 3 + 1];
    uint i2 = Indices[primitiveID * 3 + 2];

    float2 resolution = float2(Push.width, Push.height);

    float2 uv0 = Vertices[i0].GetUV() / resolution;
    float2 uv1 = Vertices[i1].GetUV() / resolution;
    float2 uv2 = Vertices[i2].GetUV() / resolution;

    float2 uv = varyings.positionCS.xy / resolution;

    uv.y = 1.0 - uv.y;

    float2 v0 = uv1 - uv0;
    float2 v1 = uv2 - uv0;
    float2 v2 = uv - uv0;

    float d00 = dot(v0, v0);
    float d01 = dot(v0, v1);
    float d11 = dot(v1, v1);
    float d20 = dot(v2, v0);
    float d21 = dot(v2, v1);

    float denom = d00 * d11 - d01 * d01;

    float b1 = (d11 * d20 - d01 * d21) / denom;
    float b2 = (d00 * d21 - d01 * d20) / denom;
    float b0 = 1.0 - b1 - b2;

    float3 bary = float3(b0, b1, b2);

    if (any(bary < 0))
    {
        bary = ClampBarycentric(uv, uv0, uv1, uv2);
    }

    // return float4(varyings.bary.xy, 0, 1);
    return float4(bary.yz, 0, 1);

    // Surface surf = Surface::Get(varyings.primitiveID, bary);

    // float alpha = SampleAlbedo(surf.uv, surf.lightmapGroup).a;
    // bool visible = alpha >= 0.5;

    // return EncodeVisibilityBuffer(bary, varyings.primitiveID, visible, false);

    // float3 positionWS = RayOffset(surf.P, surf.Ng);
    // float packed = PackNormalOctahedron(surf.N);

    // return float4(positionWS, packed);
}
