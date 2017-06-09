#px8 / python cartridge
#version 1
#__python__

def _init():
    id_sound = sound_load("./examples/assets/cat.wav")
    print(id_sound)
    if id_sound != -1:
        sound_play(id_sound)

def _update():
    pass

def _draw():
    pass
