// This file is part of Mooneye GB.
// Copyright (C) 2014-2016 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// Mooneye GB is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Mooneye GB is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Mooneye GB.  If not, see <http://www.gnu.org/licenses/>.
#version 140

uniform sampler2D tex;
//uniform sampler2D palettes;

uniform mat4 palette;

in vec2 v_tex_coords;
out vec4 f_color;

void main() {
  float color = texture(tex, v_tex_coords).x;
  int ccolor = int(color * 255.0 + 0.5);

        // 0   0   0 black
        // 32  51 123 dark_blue
        // 126  37  83 dark_purple
        // 0 144  61 dark_green
        // 171  82  54 brown
        // 52  54  53 dark_gray
        // 194 195 199 light_gray
        // 255 241 232 white
        // 255   0  77 red
        // 255 155   0 orange
        // 255 231  39 yellow
        // 0 226  50 green
        // 41 173 255 blue
        // 132 112 169 indigo
        // 255 119 168 pink
        // 255 214 197 peach

  // 0   0   0 black
  if (ccolor == 0) {
    f_color = vec4( 0., 0., 0., 0. );
        // 32  51 123 dark_blue
  } else if (ccolor == 1) {
    f_color = vec4( 0.12549019607843137, 0.2, 0.4823529411764706, 0. );
  }
        // 126  37  83 dark_purple
  else if (ccolor == 2) {
    f_color = vec4( 0.49411764705882355, 0.1450980392156863, 0.3254901960784314, 0. );
  }
        // 0 144  61 dark_green
  else if (ccolor == 3) {
    f_color = vec4( 0., 0.5647058823529412, 0.23921568627450981, 0. );
  }
        // 171  82  54 brown
  else if (ccolor == 4) {
    f_color = vec4( 0.6705882352941176, 0.3215686274509804, 0.21176470588235294, 0. );
  }
        // 52  54  53 dark_gray
  else if (ccolor == 5) {
    f_color = vec4( 0.20392156862745098, 0.21176470588235294, 0.20784313725490197, 0. );
  }
        // 194 195 199 light_gray
  else if (ccolor == 6) {
    f_color = vec4( 0.7607843137254902, 0.7647058823529411, 0.7803921568627451, 0. );
  }
        // 255 241 232 white
  else if (ccolor == 7) {
    f_color = vec4( 1., 0.9450980392156862, 0.9098039215686274, 0. );
  }
        // 255   0  77 red
  else if (ccolor == 8) {
    f_color = vec4( 1., 0., 0.30196078431372547, 0. );
  }
        // 255 155   0 orange
  else if (ccolor == 9) {
    f_color = vec4( 1., 0.6078431372549019, 0., 0. );
  }
        // 255 231  39 yellow
  else if (ccolor == 10) {
    f_color = vec4( 1., 0.9058823529411765, 0.15294117647058825, 0. );
  }
        // 0 226  50 green
  else if (ccolor == 11) {
    f_color = vec4( 0., 0.8862745098039215, 0.19607843137254902, 0. );
  }
        // 41 173 255 blue
  else if (ccolor == 12) {
    f_color = vec4( 0.1607843137254902, 0.6784313725490196, 1., 0. );
  }  
        // 132 112 169 indigo
  else if (ccolor == 13) {
    f_color = vec4( 0.5176470588235295, 0.4392156862745098, 0.6627450980392157, 0. );
  }
        // 255 119 168 pink
  else if (ccolor == 14) {
    f_color = vec4( 1., 0.4666666666666667, 0.6588235294117647, 0. );
  }
        // 255 214 197 peach
  else if (ccolor == 15) {
    f_color = vec4( 1., 0.8392156862745098, 0.7725490196078432, 0. );
  }
  else {
    f_color = vec4( 1., 1., 1., 0. );
  }
}
