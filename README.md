[![Build Status](https://travis-ci.org/Gigoteur/PX8.svg?branch=master)](https://travis-ci.org/Gigoteur/PX8)

# PX8

PX8 is an open source fantasy console (128x128 pixels) in Rust. The main features are:
 * Python 3 / Lua 5.X support for the cartridge
 * Edition of the cartridge data
 * PX8 format to use your favorite code editor
 * Screenshot (PNG)
 * Record video (GIF format)
 * Mainly run @60 FPS
 
It should work on all platform (Linux/OSX/Windows), and on tiny hardware like Raspberry Pi (with opengl enabled).

The console is inspired from [Pico-8](http://www.lexaloffle.com/pico-8.php), so you can play Pico-8 cartridges (P8/P8.PNG).

![](http://i.imgur.com/T3yB1mh.gif)
![](http://i.imgur.com/Vosz9qf.gif)
![](http://i.imgur.com/9fbKRMn.gif)

- [PX8](#px8)
  * [Build](#build)
  * [Run a cartridge](#run-a-cartridge)
  * [Edit a cartridge](#edit-a-cartridge)
  * [Keyboard Shortcut](#keyboard-shortcut)
  * [Create a new cartridge](#create-a-new-cartridge)
    + [Python](#python)
    + [Lua](#lua)
  * [Cartridge format](#cartridge-format)
  * [API documentation](#api-documentation)
  * [PX8 Format documentation](#px8-format-documentation)
  * [Compatible API with Pico-8](#compatible-api-with-pico-8)
    + [Audio](#audio)
    + [Cart Data](#cart-data)
    + [Graphics](#graphics)
    + [Input](#input)
    + [Map](#map)
    + [Math](#math)
    + [Memory](#memory)
    + [Peek/Poke](#peek-poke)
  * [Pico-8 compatibility](#pico-8-compatibility)
  * [Screenshots and Records](#screenshots-and-records)

## Build

You could build PX8 with cargo directly, in release mode for example:

```
cargo build --release 
```

## Run a cartridge

You should be able to run it directly by providing the path of the cartridge:

```
./target/release/px8 -s 4 ./games/ski/ski.px8
```

The '-s' option is the scale, so you can increase it, or in fullscreen by using '-f' option.

## Edit a cartridge

You can edit the cartridge by using the specific '-e' option:
```
./target/release/px8 -s 4 -e ./games/ski/ski.px8
```

## Keyboard Shortcut

 * F2: FPS debug
 * F3: Take a screenshot
 * F4: Take a video
 * F5: Save the current cartridge
 * F6: Switch between editor/play mode

## Create a new cartridge

PX8 could call 3 functions:
  * _init : Called once on cartridge startup, mainly to initialize your variables
  * _update: Called once per visible frame, mainly to get keyboard input for example
  * _draw: Called once per visible frame, mainly to draw things on the screen :)

### Python

```
def _init():
  print("INIT")
  
def _update():
  px8_print("UPDATE")
  
def _draw():
  px8_print("DRAW")
```

### Lua

```

```

## Cartridge format

Format | Read | Write
------------ | ------------- | -------------
P8 | :white_check_mark: | :white_check_mark: 
P8.PNG | :white_check_mark: | :red_circle:
PX8 | :white_check_mark: | :white_check_mark: 

## API documentation

See [API](https://github.com/Gigoteur/PX8/wiki/API-Documentation)

## PX8 Format documentation

## Compatible API with Pico-8

### Audio

API | Python | Lua
------------ | ------------- | -------------
sfx | :red_circle: | :red_circle:
music | :red_circle: | :red_circle:

### Cart Data

### Graphics

API | Python | Lua
------------ | ------------- | -------------
camera | :white_check_mark: | :white_check_mark:
circ | :white_check_mark: | :white_check_mark:
circfill | :white_check_mark: | :white_check_mark:
clip | :red_circle: | :red_circle:
cls | :white_check_mark: | :white_check_mark:
color | :white_check_mark: | :white_check_mark:
cursor | :red_circle: | :red_circle:
fget | :red_circle: | :red_circle:
flip | :red_circle: | :red_circle:
fset | :red_circle: | :red_circle:
line | :white_check_mark: | :white_check_mark:
print | :white_check_mark: (px8_print) | :white_check_mark:
pal | :white_check_mark: | :white_check_mark:
palt | :white_check_mark: | :white_check_mark:
pget | :white_check_mark: | :white_check_mark:
print | :white_check_mark: | :white_check_mark:
pset | :white_check_mark: | :white_check_mark:
rect | :white_check_mark: | :white_check_mark:
rectfill | :white_check_mark: | :white_check_mark:
sget | :white_check_mark: | :white_check_mark:
spr | :white_check_mark: | :white_check_mark:
sspr | :white_check_mark: | :white_check_mark:

### Input

API | Python | Lua
------------ | ------------- | -------------
btn | :white_check_mark: | :white_check_mark:
btnp | :white_check_mark: | :white_check_mark:

### Map

API | Python | Lua
------------ | ------------- | -------------
map | :white_check_mark: (spr_map) | :white_check_mark:
mget | :red_circle: | :white_check_mark:
mset | :red_circle: | :white_check_mark:

### Math

### Memory

### Peek Poke

API | Python | Lua
------------ | ------------- | -------------
stat | :red_circle: | :white_check_mark:


## Pico-8 compatibility

The version of LUA in Pico-8 has some [differences](https://gist.github.com/josefnpat/bfe4aaa5bbb44f572cd0) with the original one.

Lua features | Compatibility
------------ | ------------- 
Compound assignment operators | :white_check_mark:
Single line shorthand for if then else operator | :red_circle:
Not Equal To | :red_circle:

GFX: :white_check_mark: 

MUSIC: :red_circle:

## Screenshots and Records

![](http://i.imgur.com/1Cykf86.gif)
![](http://i.imgur.com/ySLiMqp.gif)
