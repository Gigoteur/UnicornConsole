#px8 / python cartridge
#version 1
#__python__

WAV_FILE = "./examples/assets/piano.wav"

def _init():
    sound_load(WAV_FILE)

def _update():
    if btnp(0):
        print("Play")
        sound_play(WAV_FILE)

    if btnp(1):
        print("Stop")
        sound_stop(WAV_FILE)

def _draw():
    pass
