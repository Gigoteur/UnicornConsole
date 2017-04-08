[![Build Status](https://travis-ci.org/Gigoteur/PX8.svg?branch=master)](https://travis-ci.org/Gigoteur/PX8)
[![](http://meritbadge.herokuapp.com/px8)](https://crates.io/crates/px8)

# PX8

PX8 is an Open Source Fantasy Console (128x128 pixels) in Rust, by using a cartridge that contains the code/gfx/music. The code could be in Python/Lua, or you could create directly everything in pure Rust by using it as a library.

It is still in development, but it is usable and the main features are:
 * 128x128 pixels default resolution
 * Predefined palettes (pico-8, c64, etc), or use any RGB colors
 * Python 3 / Lua 5.X support for the cartridge without tokens limit
 * Desktop/Mobile/Browser (Emscripten) support
 * Controls with dpad + 2 buttons (gamecontroller/joystick support)
 * Unlimited sprites (8x8)
 * Map support (128x32)
 * Edition of the cartridge data
 * PX8 format to be able to use your favorite code editor for Python/Lua/Rust
 * Change the screen definition (128x128, 256x256, WIDTHxHEIGHT)
 * Screenshot (PNG) / Video recording (GIF)
 * Pico-8 compatibility + cartridge (P8/P8.PNG) format support
 
It works on all platforms (Linux/OSX/Windows), in the browser (via Emscripten), and on tiny hardware like Raspberry Pi 2/3.

The console is inspired from the awesome [Pico-8](http://www.lexaloffle.com/pico-8.php), so there is a [compatibility](https://github.com/Gigoteur/PX8/wiki/Pico-8-compatibility) mode (not 100%) available with Pico-8 console and cartridges (P8/PNG).

- [PX8](#px8)
  * [Demos](#demos)
  * [Download](#download)
    + [Binaries](#binaries)
  * [Requirements](#requirements)
  * [Build](#build)
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
  
## Demos
**The time for each frame is slow (10ms) in the GIF, and doesn't correspond to the speed of the game.**

![](http://i.imgur.com/tzoNZAa.gif)

![](http://i.imgur.com/lFB2UPw.gif)

![](https://j.gifs.com/0gMZ87.gif)

![](https://j.gifs.com/xGyM1n.gif)

![](https://j.gifs.com/vgw08V.gif)

![](https://j.gifs.com/k5gVw5.gif)

Editor mode:

![](http://i.imgur.com/ZxNgWrt.gif)

More advanced examples:

* Advanced Micro Platformer - Starter Kit http://www.lexaloffle.com/bbs/?tid=28793 in Python
https://github.com/Gigoteur/PX8/tree/master/games/amp
![](http://i.imgur.com/fKvyRnP.gif)

* 2D Terrain Generation (http://www.lexaloffle.com/bbs/?uid=12213) in Python
https://github.com/Gigoteur/PX8/tree/master/games/terrain
![](http://i.imgur.com/ORElunz.gif)

## Download

You can get directly the latest version via git:
```
git clone https://github.com/Gigoteur/PX8.git
cd PX8
```

### Binaries

Or you can get binaries for multiples platforms directly on [itch.io](https://hallucino.itch.io/px8):
  * Raspberry Pi (available)
  * Windows (Work in progress)
  * Linux (Work in progress)
  * Mac (Work in progress)


## Requirements

You will need multiple things:
  * SDL2
  * python3
  * libreadline

#### Linux

Packages:
  * libsdl2-dev
  * libreadline-dev
  * libpython3-dev

##### Raspberry Pi

Please enable the GL Driver (7 - Advanced Options -> Ac - GL Driver -> Yes) via:
```
sudo raspi-config
```

#### OSX

## Build

You could build PX8 with cargo directly, in release mode for example, with the support of Python and Lua.

```
cargo build --features="cpython lua" --release 
```

## Resolution

By default the resolution will 128x128 but you can change the default values by specifying the values of the env variables PX8_SCREEN_WIDTH + PX8_SCREEN_HEIGHT during the compulation:
```
PX8_SCREEN_WIDTH=256 PX8_SCREEN_HEIGHT=256
```

Example:
```
PX8_SCREEN_WIDTH=256 PX8_SCREEN_HEIGHT=256 cargo build --release
```

### With GFX-RS

If you want to use the gfx_rs renderer (WIP):

```
cargo build --features="gfx_rs_renderer" --release 
```

### SDL + Opengl

You can force opengl with SDL via the '-o' option:

```
./target/release/px8 -o ./games/ski/ski.px8
```

### With Emscripten

You must follow the following [guide](https://users.rust-lang.org/t/compiling-to-the-web-with-rust-and-emscripten/7627) to install Emscripten.

You can see example of PX8 + Emscripten in the demos [repository](https://github.com/Gigoteur/PX8Demos/tree/master/emscripten/). You could also see live example from your browser:
  * Hello World [[Source Code]](https://github.com/Gigoteur/PX8Demos/tree/master/emscripten/Hello) [[Live Demo]](https://hallucino.github.io/demos/hello.html)
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
  * F2: FPS debug
  * F3: Take a screenshot
  * F4: Take a video
  * F5: Save the current cartridge
  * F6: Switch between editor/play mode
  * F7: Switch to the next available palette
  
## Run a cartridge

You should be able to run it directly by providing the path of the cartridge:

```
./target/release/px8 ./games/ski/ski.px8
```

### Demos

You could run the API demos:
```
./target/release/px8 -s 4 ./demos/api_demos.py 
```

or some fancy demos:
```
./target/release/px8 -s 4 ./demos/demos.py
```

## Edit a cartridge

You can edit the GFX in the cartridge by using the specific '-e' option, and alternate between the run mode and the editor with 'F6':
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

### Python

You can create a classical Python program, all you need is to define the previous functions (_init, _update, _draw), and you can import any packages.


```
def _init():
  px8_print("INIT")
  
def _update():
  px8_print("UPDATE")
  
def _draw():
  px8_print("DRAW")
```

### Lua

```
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
cursor | :red_circle: | :red_circle: | :red_circle:
ellipse | :white_check_mark: | :white_check_mark: | :white_check_mark:
ellipsefill | :white_check_mark: | :white_check_mark: | :white_check_mark:
fget | :white_check_mark: | :white_check_mark: | :white_check_mark:
flip | :red_circle: | :red_circle: | :red_circle:
fset | :white_check_mark: | :white_check_mark: | :white_check_mark:
line | :white_check_mark: | :white_check_mark: | :white_check_mark:
pal | :white_check_mark: | :white_check_mark: | :white_check_mark:
palt | :white_check_mark: | :white_check_mark: | :white_check_mark:
pget | :white_check_mark: | :white_check_mark: | :white_check_mark:
print | :white_check_mark: | :white_check_mark: | :white_check_mark:
pset | :white_check_mark: | :white_check_mark: | :white_check_mark:
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
