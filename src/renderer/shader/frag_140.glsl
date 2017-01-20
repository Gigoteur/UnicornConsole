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
  if (ccolor == 0) {
    f_color = vec4( 0., 0., 0., 0. );
  // 29 43 83 dark_blue
  } else if (ccolor == 1) {
    f_color = vec4( 0.11372549019607843, 0.16862745098039217, 0.3254901960784314, 0. );
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
  // 95  87  79 dark_gray
  else if (ccolor == 5) {
    f_color = vec4( 0.37254901960784315, 0.3411764705882353, 0.30980392156862746, 0. );
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
  // 255 63   0 orange
  else if (ccolor == 9) {
    f_color = vec4( 1., 0.24705882352941178, 0., 0. );
  }
  // 255 236  39 yellow
  else if (ccolor == 10) {
    f_color = vec4( 1., 0.9254901960784314, 0.15294117647058825, 0. );
  }
  // 0 228 54 green
  else if (ccolor == 11) {
    f_color = vec4( 0., 0.8941176470588236, 0.21176470588235294, 0. );
  }
  // 41 173 255 blue
  else if (ccolor == 12) {
    f_color = vec4( 0.1607843137254902, 0.6784313725490196, 1., 0. );
  }  
  // 132 118 156 indigo
  else if (ccolor == 13) {
    f_color = vec4( 0.5176470588235295, 0.4627450980392157, 0.611764705882353, 0. );
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
