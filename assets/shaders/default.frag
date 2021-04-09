#version 330 core

layout(location = 0) out vec4 Color;

in vec4 o_Color;

highp float rand(vec2 co)
{
    highp float a = 12.9898;
    highp float b = 78.233;
    highp float c = 43758.5453;
    highp float dt= dot(co.xy ,vec2(a,b));
    highp float sn= mod(dt,3.14);
    return fract(sin(sn) * c);
}

void main()
{
    float rand_1 = rand(vec2(43, 545));
    float rand_2 = rand(vec2(5, -45));
    float rand_3 = rand(vec2(-345, 73));

    // Color = vec4(0.2627 * rand_1, 0.5176 * rand_2, 0.8549 * rand_3, 1.0);
    Color = o_Color;
}