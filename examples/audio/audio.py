#px8 / python cartridge
#version 1
#__python__

WAV_FILE = "./examples/assets/piano.wav"
SOUND_GUN_FILE = "./examples/assets/gun.wav"
SOUND_FIREWORKS_FILE = "./examples/assets/fireworks.wav"

class Button(object):
    def __init__(self, x1, y1, x2, y2, color, text, highlight=False):
        self.x1 = x1
        self.y1 = y1
        self.x2 = x2
        self.y2 = y2
        self.color = color
        self.text = text
        self.clicked = True if highlight else False

    def update(self, x, y):
        self.clicked = (self.x1 <= x <= self.x2 and
                        self.y1 <= y <= self.y2)

    def draw(self):
        rectfill(self.x1, self.y1, self.x2, self.y2, self.color)
        i = 3 if self.clicked else 1
        px8_print(self.text, self.x1 + 1, self.y1, i)

    def is_click(self):
        return self.clicked

MENU = {
    'Play': Button(20, 20, 40, 28, 7, 'Play'),
    'Stop': Button(42, 20, 62, 28, 7, 'Stop'),
    'Pause': Button(64, 20, 84, 28, 7, 'Pause'),
    'Resume': Button(86, 20, 110, 28, 7, 'Resume'),
    'Gun': Button(20, 30, 40, 38, 7, 'Gun'),
    'Fireworks': Button(42, 30, 80, 38, 7, 'Fireworks'),
}

def _init():
    show_mouse()
    music_load(WAV_FILE)
    sound_load(SOUND_GUN_FILE)
    sound_load(SOUND_FIREWORKS_FILE)

def _update():
    if mouse_state():
        mousex, mousey = mouse_x(), mouse_y()

        for item in MENU.values():
            item.update(mousex, mousey)
            if item.text =='Play' and item.is_click():
                print("Play")
                music_play(WAV_FILE)
            elif item.text =='Stop' and item.is_click():
                print("Stop")
                music_stop()
            elif item.text =='Pause' and item.is_click():
                print("Pause")
                music_pause()
            elif item.text =='Resume' and item.is_click():
                print("Resume")
                music_resume()
            elif item.text =='Gun' and item.is_click():
                print("Play gun sound")
                sound_play(SOUND_GUN_FILE)
            if item.text =='Fireworks' and item.is_click():
                print("Play fireworks sound")
                sound_play(SOUND_FIREWORKS_FILE)

def _draw():
    cls()

    for item in MENU.values():
        item.draw()

