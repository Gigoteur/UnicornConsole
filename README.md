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

## Edition

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

## Cartridge format

Format | Read | Write
------------ | ------------- | -------------
P8 | :white_check_mark: | :white_check_mark: 
P8.PNG | :white_check_mark: | :red_circle:
PX8 | :white_check_mark: | :white_check_mark: 

## API documentation

## PX8 Format documentation

## Demos

## Compatible API

API | Python | Lua
------------ | ------------- | -------------
rect | :white_check_mark: | :white_check_mark:
rectfill | :white_check_mark: | :white_check_mark:
circ | :white_check_mark: | :white_check_mark:
circfill | :white_check_mark: | :white_check_mark:
spr | :white_check_mark: | :white_check_mark:
map | :white_check_mark: | :white_check_mark:
rect | :white_check_mark: | :white_check_mark:
rect | :white_check_mark: | :white_check_mark:
rect | :white_check_mark: | :white_check_mark:
sfx | :red_circle: | :red_circle:
music | :red_circle: | :red_circle:

## Pico-8 compatibility

The version of LUA in Pico-8 has some [differences](https://gist.github.com/josefnpat/bfe4aaa5bbb44f572cd0) with the original one.

Lua features | Compatibility
------------ | ------------- 
Compound assignment operators | :white_check_mark:
Single line shorthand for if then else operator | :red_circle:
Not Equal To | :red_circle:

Lua API | Compatibility
------------ | ------------- 

GFX: :white_check_mark: 

MUSIC: :red_circle:

