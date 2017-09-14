<p align="center">
  <img src="docs/px8.png">
</p>

<p align="center">
  <a href="https://travis-ci.org/Gigoteur/PX8">
      <img src="https://travis-ci.org/Gigoteur/PX8.svg?branch=master" alt="Travis for PX8">
  </a>
  <a href="https://crates.io/crates/px8">
      <img src="http://meritbadge.herokuapp.com/px8" alt="PX8 Crate">
  </a>
</p>

# PX8

  * [Download](#download)
    + [Build](#build)
    + [Binaries](#binaries)
  * [Requirements](#requirements)
  * [Resolution](#resolution)
  * [Coordinate System](#coordinate-system)
  * [Keyboard Shortcut](#keyboard-shortcut)
  * [Run a cartridge](#run-a-cartridge)
  * [Edit a cartridge](#edit-a-cartridge)
  * [Display options](#display-options)
  * [How to create a new cartridge](#how-to-create-a-new-cartridge)
    + [Python](#python)
    + [Lua](#lua)
  * [Cartridge format](#cartridge-format)
  * [API documentation](#api-documentation)
  
  
PX8 is an Open Source Fantasy Console (128x128 pixels default resolution) in Rust, by using a cartridge that contains the code/gfx/music. The code could be in Python/Lua, or you could create directly everything in pure Rust by using it as a library.

Specifications:
 * 128x128 pixels default resolution
 * Predefined 16 colour palettes (pico-8, c64, etc)
 * Python 3 / Lua 5.3 support for the cartridge without tokens limit
 * Desktop/Mobile/Browser (Emscripten) support
 * Controls with dpad + 2 buttons (gamecontroller/joystick support)
 * Unlimited 8x8 sprites
 * Map 128x32 8-bit cels
 * Editor for the sprite / map
 * PX8 format to be able to use your favorite code editor for Python/Lua/Rust
 * Mutliple fonts support (pico-8, bbc, cbmII, appleII)
 * Audio support with an integrated ![Chiptune](https://github.com/kometbomb/klystron/)
 * Change the screen definition dynamically and the aspect ratio
 * Screenshot (PNG) / Video recording (GIF)
 * Pico-8 compatibility + cartridge (P8/P8.PNG) format support
 
It works on all platforms (Linux/OSX/Windows/Raspberry PI), in the browser (via Emscripten).

You can follow the development of the project [here](https://hallucino.itch.io/px8/devlog).

[![Donate](https://www.paypalobjects.com/en_US/i/btn/btn_donateCC_LG.gif)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=T9NLJWMVWRMVU&lc=FR&item_name=Gigoteur&currency_code=EUR&bn=PP%2dDonationsBF%3abtn_donateCC_LG%2egif%3aNonHosted)

![](http://i.imgur.com/tzoNZAa.gif)

Editor mode:

![](http://i.imgur.com/hh8OiR8.gif)

![](http://i.imgur.com/hD22SD8.gif)

![](http://i.imgur.com/m6d2dUL.gif)

Multiple resolution support:

![](http://i.imgur.com/NfdxTrT.gif)


More advanced examples:

* Advanced Micro Platformer - Starter Kit http://www.lexaloffle.com/bbs/?tid=28793 in Python
https://github.com/Gigoteur/PX8/tree/master/games/amp
![](http://i.imgur.com/fKvyRnP.gif)

* 2D Terrain Generation (http://www.lexaloffle.com/bbs/?uid=12213) in Python
https://github.com/Gigoteur/PX8/tree/master/games/terrain
![](http://i.imgur.com/ORElunz.gif)

* Voxel framework (http://www.lexaloffle.com/bbs/?tid=28308) in Python
https://github.com/Gigoteur/PX8/tree/master/examples/voxel
![](http://i.imgur.com/b1wE1cs.gif)

* BR: it is an example from duckduckontheloose (https://github.com/seleb/DuckDuckOnTheLoose)
https://github.com/Gigoteur/PX8/tree/master/games/BR
![](http://i.imgur.com/Xp3v1Lj.gif)

* Flappy bird clone(http://p1xl.com/files/flappy.p8):
https://github.com/Gigoteur/PX8/tree/master/games/flappy
![](http://i.imgur.com/Wg5jR9t.gif)

* Noise support:
![](http://i.imgur.com/um3gmWh.gif)

More [gifs](https://github.com/Gigoteur/PX8/wiki/Examples) ?

## Download

You can get directly the latest version via git:
```
git clone https://github.com/Gigoteur/PX8.git
cd PX8
```


### Build

**The first thing to do is to install Rust, so please go to [rustup](https://www.rustup.rs/) and follow all instructions.**

The build is the same for all platforms (Linux/OSX/Windows).

You must build PX8 with cargo directly in release mode to have the best perf. And you can choose to disable the following plugins for the cartridge:
  * cpython
  * px8_plugin_lua (rust-lua53 with modification)


For example to have all features:

```
cargo build --features="cpython px8_plugin_lua" --release 
```

### Binaries

Or you can get latest binaries for many platforms directly on [itch.io](https://hallucino.itch.io/px8) or you can build your own executable for free (see the BUILD instruction):
  * [Raspberry Pi 3](https://hallucino.itch.io/px8)
  * [Linux x64](https://hallucino.itch.io/px8)
  * [Windows x64](https://hallucino.itch.io/px8/purchase) and install [Python](https://www.python.org/)
  * [OSX](https://hallucino.itch.io/px8/purchase)

## Requirements

You will need multiple things:
  * SDL2
  * SDL2_mixer
  * python3

#### Linux

Packages for Debian/Ubuntu:
  * libsdl2-dev
  * libsdl2-mixer-dev
  * libpython3-dev

##### Raspberry Pi

Please enable the GL Driver to speed up the console (7 - Advanced Options -> Ac - GL Driver -> Yes) via:
```
sudo raspi-config
```

#### OSX

Install external dependencies via brew:
   * brew install python3
   * brew install sdl2
   * brew install sdl2_mixer
   
Right now you need to export the DYLD_FALLBACK_LIBRARY_PATH env variable for the python support, e.g:
   * export DYLD_FALLBACK_LIBRARY_PATH=/usr/local/Cellar/python3/3.5.1/Frameworks/Python.framework/Versions/3.5/lib



## Resolution

By default the resolution will 128x128 but you can change the default values by calling the [mode](https://github.com/Gigoteur/PX8/wiki/API-Documentation#mode) API function:
```
mode(width, height, [aspect_ratio])
```

Example:
```
mode(128, 128, 1.0)
mode(512, 128, 4.0)
```

### SDL + Opengl

You can force opengl with SDL via the '-o' option:

```
./target/release/px8 -o ./games/ski/ski.px8
```

### With Emscripten

You must follow the following [guide](https://users.rust-lang.org/t/compiling-to-the-web-with-rust-and-emscripten/7627) to install Emscripten. After that you can find some help [here](https://github.com/Gigoteur/PX8/wiki/emscripten).

You can see example of PX8 + Emscripten in the demos [repository](https://github.com/Gigoteur/PX8Demos/tree/master/emscripten/). You could also see live example from your browser:
  * Hello World [[Source Code]](https://github.com/Gigoteur/PX8Demos/tree/master/emscripten/Hello) [[Live Demo]](https://hallucino.github.io/demos/hello.html)
  * Hello World 2 [[Source Code]](https://github.com/Gigoteur/PX8Demos/tree/master/emscripten/Hello2) [[Live Demo]](https://hallucino.github.io/demos/hello2.html)
  * Lua Cartridge [[Source Code]](https://github.com/Gigoteur/PX8Demos/tree/master/emscripten/LuaCartridge) [[Live Demo]](https://hallucino.github.io/demos/lua_cartridge.html)
  * Cast [[Source Code]](https://github.com/Gigoteur/PX8Demos/tree/master/emscripten/Cast) [[Live Demo]](https://hallucino.github.io/demos/cast.html)

## Coordinate system

Each pixel can be access from 0 to 128 (or the new defined width/height) :
![](https://github.com/libgdx/libgdx/wiki/images/screenpixels.png)

## Keyboard Shortcut

Player 1:
  * cursors, Z,X / C,V / N,M

Player 2:
  * ESDF, LSHIFT,A / TAB,Q,E

System shortcut:
  * F2: Information debug (FPS, time execution (draw,update), palette name)
  * F3: Take a screenshot (png)
  * F4: Take a video (gif)
  * F5: Save the current cartridge's data (if opened with editor mode)
  * F6: Switch between editor/play mode
  * F7: Switch to the next available palette
  * P: Pause the console
  
## Run a cartridge

You should be able to run it directly by providing the path of the cartridge:

```
./target/release/px8 ./games/ski/ski.px8
```

### Demos

You could run the API demos:
```
./target/release/px8 -s 4 ./examples/api_demos.p8
```

or some fancy demos:
```
./target/release/px8 -s 4 ./examples/demos.p8
./target/release/px8 -s 4 ./examples/voxel/voxel.px8
./target/release/px8 -s 4 ./examples/pong/pong.px8
./target/release/px8 -s 4 ./games/ski/ski.px8
./target/release/px8 -s 4 ./games/amp/amp.px8
./target/release/px8 -s 4 ./games/terrain/terrain.px8
./target/release/px8 -s 4 ./games/BR/BR.px8
```

## Edit a cartridge

You can edit directly the GFX (Sprites + Map) with the 'F6' hotkey (to alternate between the run mode and the editor), or to open the cartridge by using the specific '-e' option:
```
./target/release/px8 -s 4 -e ./games/ski/ski.px8
```
and you can save the GFX with 'F5'.

## Display options

### Change the scale

With the '-s' option you can change the size of the console, so you can increase it (2/4/8/10).

You can also use the fullscreen option by using '-f' option.

### SDL + OpenGL

You can add the '-o' option to force SDL to use OpenGL

## Compatibility mode with PICO8

You could load a PICO8 cartridge file by using the '-m pico8' option to convert the Lua code.

## How to create a new cartridge

PX8 will call 3 functions, at startup or during the runtime:
  * _init : Called once on startup, mainly to initialize your variables
  * _update: Called once per visible frame, mainly to get keyboard input for example
  * _draw: Called once per visible frame, mainly to draw things on the screen :)

After that you can use the API to do your game. There is no limitation of what you can do in Python or Lua languages.

By default I don't do any modification in the Python or Lua interpreter, so you are free to create threads, load native files, etc


You will be able to find more technical documentation in the [wiki](https://github.com/Gigoteur/PX8/wiki)

### Python

The syntax of Python program is exactly the same that the Python 3.

You can create a classical Python program, all you need is to define the previous functions (_init, _update, _draw), and you can import any packages.


```py
def _init():
  px8_print("INIT")
  
def _update():
  px8_print("UPDATE")
  
def _draw():
  px8_print("DRAW")
```

### Lua

This is a modified version of Lua 5.3.4 that supports:
  * Compound-assignment operators (+=,-=,*=,/=,%=)


```lua
function _init()
  print("INIT")
end

function _update()
  print("UPDATE")
end

function _draw()
  print("DRAW")
end
```


## Cartridge format

Format | Read | Write
------------ | ------------- | -------------
P8 | :white_check_mark: | :white_check_mark: 
P8.PNG | :white_check_mark: | :red_circle:
PX8 | :white_check_mark: | :white_check_mark: 

## API documentation

API | Rust | Python | Lua
------------ | ------------- | ------------- | -------------
camera | :white_check_mark: | :white_check_mark: | :white_check_mark:
circ | :white_check_mark: | :white_check_mark: | :white_check_mark:
circfill | :white_check_mark: | :white_check_mark: | :white_check_mark:
clip | :white_check_mark: | :white_check_mark: | :white_check_mark:
cls | :white_check_mark: | :white_check_mark: | :white_check_mark:
color | :white_check_mark: | :white_check_mark: | :white_check_mark:
ellipse | :white_check_mark: | :white_check_mark: | :white_check_mark:
ellipsefill | :white_check_mark: | :white_check_mark: | :white_check_mark:
fget | :white_check_mark: | :white_check_mark: | :white_check_mark:
fset | :white_check_mark: | :white_check_mark: | :white_check_mark:
font | :white_check_mark: | :white_check_mark: | :white_check_mark:
line | :white_check_mark: | :white_check_mark: | :white_check_mark:
mode | :white_check_mark: | :white_check_mark: | :white_check_mark:
pal | :white_check_mark: | :white_check_mark: | :white_check_mark:
palt | :white_check_mark: | :white_check_mark: | :white_check_mark:
pget | :white_check_mark: | :white_check_mark: | :white_check_mark:
print | :white_check_mark: | :white_check_mark: | :white_check_mark:
pset | :white_check_mark: | :white_check_mark: | :white_check_mark:
noise | :white_check_mark: | :white_check_mark: | :white_check_mark:
noise_set_feed | :white_check_mark: | :white_check_mark: | :white_check_mark:
rect | :white_check_mark: | :white_check_mark: | :white_check_mark:
rectfill | :white_check_mark: | :white_check_mark: | :white_check_mark:
sget | :white_check_mark: | :white_check_mark: | :white_check_mark:
spr | :white_check_mark: | :white_check_mark: | :white_check_mark:
sset | :white_check_mark: | :white_check_mark: | :white_check_mark:
sspr | :white_check_mark: | :white_check_mark: | :white_check_mark:
trigon | :white_check_mark: | :white_check_mark: | :white_check_mark:
trigonfill | :red_circle: | :red_circle: | :red_circle:
btn | :white_check_mark: | :white_check_mark: | :white_check_mark:
btnp | :white_check_mark: | :white_check_mark: | :white_check_mark:
map | :white_check_mark: | :white_check_mark: | :white_check_mark:
mget | :white_check_mark: | :white_check_mark: | :white_check_mark:
mset | :white_check_mark: | :white_check_mark: | :white_check_mark:


More details here about each function with the arguments: [API](https://github.com/Gigoteur/PX8/wiki/API-Documentation)


The console is inspired from the awesome [Pico-8](http://www.lexaloffle.com/pico-8.php), so there is a [compatibility](https://github.com/Gigoteur/PX8/wiki/Pico-8-compatibility) mode (not 100%, and it is not the goal of the project) available with Pico-8 console and cartridges (P8/PNG).
  
