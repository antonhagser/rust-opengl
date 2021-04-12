#version 330 core

in vec3 Position;
in vec3 Color;

out vec4 o_Color;

void main()
{
    o_Color=vec4(Color * Position,1.);
    gl_Position=vec4(Position,1.);
}