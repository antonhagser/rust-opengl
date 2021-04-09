#version 330 core

in vec3 Position;
in vec3 Color;

out vec4 o_Color;

void main()
{
    gl_Position=vec4(Position,1.);
    o_Color=vec4(Color,1.);
}