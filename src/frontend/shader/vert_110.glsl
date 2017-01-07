#version 110

uniform mat4 matrix;

attribute vec2 position;
attribute vec2 tex_coords;

varying vec2 v_tex_coords;

void main() {
  gl_Position = matrix * vec4(position, 0.0, 1.0);
  v_tex_coords = tex_coords;
}
