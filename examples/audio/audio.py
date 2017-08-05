#px8 / python cartridge
#version 1
#__python__

WAV_FILE = "./examples/assets/piano.wav"
SOUND_GUN_FILE = "./examples/assets/gun.wav"
SOUND_FIREWORKS_FILE = "./examples/assets/fireworks.wav"
KLYSTRACK_MUSIC = "./examples/assets/AmsterdamBoppe.kto"

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

class InteractiveNumber(object):
    def __init__(self, x, y, color):
        self.x = x
        self.y = y
        self.color = color
        self.value = 128
        self.text = 'Unknown'

        base_x_rect = self.x - 4
        base_y_rect = self.y - 4
        self.rect_minus = [base_x_rect, self.y, base_x_rect+2, self.y+2]
        self.rect_plus = [base_x_rect, base_y_rect, base_x_rect+2, base_y_rect+2]

    def update(self, x, y):
        rect_min_clicked = (self.rect_minus[0] <= x <= self.rect_minus[2] and
                            self.rect_minus[1] <= y <= self.rect_minus[3])
        if rect_min_clicked:
            self.value -= 10
            self.value = max(0, self.value)


        rect_plus_clicked = (self.rect_plus[0] <= x <= self.rect_plus[2] and
                             self.rect_plus[1] <= y <= self.rect_plus[3])
        if rect_plus_clicked:
            self.value += 10
            self.value = min(128, self.value)

        if rect_min_clicked or rect_plus_clicked:
            music_volume(self.value)

    def draw(self):
        rectfill(self.rect_minus[0], self.rect_minus[1], self.rect_minus[2], self.rect_minus[3], self.color)
        rectfill(self.rect_plus[0], self.rect_plus[1], self.rect_plus[2], self.rect_plus[3], self.color)


MENU = {
    'Volume': InteractiveNumber(18, 24, 7),
    'Play': Button(20, 20, 40, 28, 7, 'Play'),
    'Stop': Button(42, 20, 62, 28, 7, 'Stop'),
    'Pause': Button(64, 20, 84, 28, 7, 'Pause'),
    'Resume': Button(86, 20, 110, 28, 7, 'Resume'),
    'Gun': Button(20, 30, 40, 38, 7, 'Gun'),
    'Fireworks': Button(42, 30, 80, 38, 7, 'Fireworks'),
}

def _init():
    show_mouse()
    chiptune_play(0, KLYSTRACK_MUSIC, 0, 0)
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

    idx = 10
    for i in range(0, 16):
        is_playing = sound_isplaying(i)
        color = 7
        if is_playing:
            color = 8
        circfill(idx+i*5, 100, 1, color)
