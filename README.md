<p align="center">
  <img src="unicorn-docs/unicorn.png">
</p>

<p align="center">
  <a href="https://travis-ci.org/Gigoteur/UnicornConsole">
      <img src="https://travis-ci.org/Gigoteur/UnicornConsole.svg?branch=master" alt="Travis for UnicornConsole">
  </a>
  <a href="LICENSE">
      <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="LICENSE">
  </a>
</p>

# Unicorn Console 

Unicorn Console is a quick and dirty engine that let you do what you want with a resolution of 400x240 pixels. The main engine is not dependant of a specific GFX library so you can use it where you want:
  * [unicorn](https://github.com/Gigoteur/UnicornConsole/tree/master/unicorn): Main engine source code
  * [unicorn-devkit](https://github.com/Gigoteur/UnicornConsole/tree/master/unicorn-devkit): SDL2 version
  * [unicorn-libretro](https://github.com/Gigoteur/UnicornConsole/tree/master/unicorn-libretro): [libretro](http://www.libretro.com/index.php/api/) API version
  * [unicorn-web](https://github.com/Gigoteur/UnicornConsole/tree/master/unicorn-web): Webassembly version
  * [unicorn-android](https://github.com/Gigoteur/UnicornConsole/tree/master/unicorn-android): Android version
  
  
TOC:
  * [Features](#features)
  * [Requirements](#requirements)
  * [Download](#download)
    + [Build](#build)
  * [Create](#create)

## Features

  * Display: 400x240 pixels, 32 bits color
  * Palette: predefined palettes/extend existing one
  * Sprite: 3200 8x8 sprites
  * Dynamic sprite: create/save sprites with all size
  * Map: 400x60 cells, 3200x480 pixels
  * Code: Rust/Javascript/Python/Lua
  * Sound: chiptune support via [klystron](http://kometbomb.github.io/klystrack/) engine 
  * Editor: GFX/SOUND/CODE editor
  

## Download
### Build

Cargo feature:
  * cpython: enable python support
  * unicorn_plugin_lua: enable lua support
  * duktape: enable duktape (javascript) support
  * libksnd: use the native version of klystron for the sound

You can choose to build the main UI to play/edit games:
```
cd unicorn-devkit
cargo build --release
```

and run it with the default embedded game:
```
./target/release/uc-devkit
```

or load an existing one:
```
./target/release/uc-devkit ../unicorn/games/floppybird/floppybird.uni
```

You can also choose to build the libretro version:
```
cd unicorn-libretro
cargo build --release
```

And load the shared library with retroarch:
```
retroarch -L target/release/libunicorn_libretro.so ../unicorn/examples/api_demos.uni
```






