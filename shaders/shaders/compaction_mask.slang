#include "includes/core.slang"

[[vk::push_constant]]
PushConstants Push;

struct PushConstants
{
    uint2 resolution;
    uint offset;
    uint pad1;
};

uint InterleaveWithZero(uint word)
{
    word = (word ^ (word << 8)) & 0x00ff00ff;
    word = (word ^ (word << 4)) & 0x0f0f0f0f;
    word = (word ^ (word << 2)) & 0x33333333;
    word = (word ^ (word << 1)) & 0x55555555;
    return word;
}

uint DeinterleaveWithZero(uint word)
{
    word &= 0x55555555;
    word = (word | (word >> 1)) & 0x33333333;
    word = (word | (word >> 2)) & 0x0f0f0f0f;
    word = (word | (word >> 4)) & 0x00ff00ff;
    word = (word | (word >> 8)) & 0x0000ffff;
    return word;
}

uint2 IndexToUV(uint index)
{
    return uint2(DeinterleaveWithZero(index), DeinterleaveWithZero(index >> 1));
}

[WaveSize(32)]
    [numthreads(32, 1, 1)] void
    main(uint3 threadID : SV_DispatchThreadID, uint3 groupID : SV_GroupID)
{
    uint2 resolution = Push.resolution;

    bool visible = false;

    if (threadID.x < resolution.x * resolution.y)
    {
        // uint2 coords;
        // coords.x = threadID.x % resolution.x;
        // coords.y = threadID.x / resolution.x;

        uint2 coords = IndexToUV(threadID.x);

        uint2 visibility = VisibilityBuffer[coords];
        visible = visibility.x != uint::maxValue;
    }

    uint mask = WaveActiveBallot(visible).x;

    if (WaveIsFirstLane())
    {
        CompactionBuffer[groupID.x + Push.offset] = uint2(mask, countbits(mask));
    }
}
