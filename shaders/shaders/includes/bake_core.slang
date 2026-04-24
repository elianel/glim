#include "bake_bindings.slang"

static const float RAY_MAX = 10000.0;

struct Ray
{
    float3 P;
    float tMin;
    float3 D;
    float tMax;
}

struct Hit
{
    float2 bary;
    float t;
    uint primitiveID;
}
