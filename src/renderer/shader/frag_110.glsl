#version 110

uniform sampler2D tex;
uniform sampler1D palette;

varying vec2 v_tex_coords;

void main() {
  float color = texture2D(tex, v_tex_coords).x;

 int ccolor = int(color * 255.0 + 0.5);



  // 0   0   0 black
  if (ccolor == 0) {
    gl_FragColor = vec4( 0., 0., 0., 0. );
        // 32  51 123 dark_blue
  } else if (ccolor == 1) {
    gl_FragColor = vec4( 0.12549019607843137, 0.2, 0.4823529411764706, 0. );
  }
        // 126  37  83 dark_purple
  else if (ccolor == 2) {
    gl_FragColor = vec4( 0.49411764705882355, 0.1450980392156863, 0.3254901960784314, 0. );
  }
        // 0 144  61 dark_green
  else if (ccolor == 3) {
    gl_FragColor = vec4( 0., 0.5647058823529412, 0.23921568627450981, 0. );
  }
        // 171  82  54 brown
  else if (ccolor == 4) {
    gl_FragColor = vec4( 0.6705882352941176, 0.3215686274509804, 0.21176470588235294, 0. );
  }
        // 52  54  53 dark_gray
  else if (ccolor == 5) {
    gl_FragColor = vec4( 0.20392156862745098, 0.21176470588235294, 0.20784313725490197, 0. );
  }
        // 194 195 199 light_gray
  else if (ccolor == 6) {
    gl_FragColor = vec4( 0.7607843137254902, 0.7647058823529411, 0.7803921568627451, 0. );
  }
        // 255 241 232 white
  else if (ccolor == 7) {
    gl_FragColor = vec4( 1., 0.9450980392156862, 0.9098039215686274, 0. );
  }
        // 255   0  77 red
  else if (ccolor == 8) {
    gl_FragColor = vec4( 1., 0., 0.30196078431372547, 0. );
  }
        // 255 155   0 orange
  else if (ccolor == 9) {
    gl_FragColor = vec4( 1., 0.6078431372549019, 0., 0. );
  }
        // 255 231  39 yellow
  else if (ccolor == 10) {
    gl_FragColor = vec4( 1., 0.9058823529411765, 0.15294117647058825, 0. );
  }
        // 0 226  50 green
  else if (ccolor == 11) {
    gl_FragColor = vec4( 0., 0.8862745098039215, 0.19607843137254902, 0. );
  }
        // 41 173 255 blue
  else if (ccolor == 12) {
    gl_FragColor = vec4( 0.1607843137254902, 0.6784313725490196, 1., 0. );
  }  
        // 132 112 169 indigo
  else if (ccolor == 13) {
    gl_FragColor = vec4( 0.5176470588235295, 0.4392156862745098, 0.6627450980392157, 0. );
  }
        // 255 119 168 pink
  else if (ccolor == 14) {
    gl_FragColor = vec4( 1., 0.4666666666666667, 0.6588235294117647, 0. );
  }
        // 255 214 197 peach
  else if (ccolor == 15) {
    gl_FragColor = vec4( 1., 0.8392156862745098, 0.7725490196078432, 0. );
  }
  else {
    gl_FragColor = vec4( 1., 1., 1., 0. );
  }
}
