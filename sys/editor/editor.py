import random
import math

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

class State(object):
    def __init__(self):
        self.mouse_x = 0
        self.mouse_y = 0

        self.idx_sprites_batch = 88
        self.current_sprite = 0

        self.x_zoom_sprite = 0
        self.y_zoom_sprite = 0

        self.idx_x_zoom_sprite = 10
        self.idx_y_zoom_sprite = 10

        self.idx_map = 0

def pointInRectangle(x, y, coord):
    return (coord[0] <= x <= coord[2] and
            coord[1] <= y <= coord[3])

class SpritesMap(object):
    def __init__(self, pp):
        self.pp = pp
        self.state = State()
        self.buttons = [96, 79, 115, 87]
        self.buttons_map = [Button(96, 79, 100, 87, 2, "1", True),
                            Button(101, 79, 105, 87, 2, "2"),
                            Button(106, 79, 110, 87, 2, "3"),
                            Button(111, 79, 115, 87, 2, "4")]

    def update(self):
        self.state.mouse_state = mouse_state()
        if self.state.mouse_state == 1:
            self.state.mouse_x = mouse_x()
            self.state.mouse_y = mouse_y()

            if pointInRectangle(self.state.mouse_x, self.state.mouse_y, self.buttons):
                for btn_idx, button in enumerate(self.buttons_map):
                    button.update(self.state.mouse_x, self.state.mouse_y)
                    if button.is_click():
                        self.state.idx_map = btn_idx

            if pointInRectangle(self.state.mouse_x, self.state.mouse_y, [self.state.idx_x_zoom_sprite,
                                                                         self.state.idx_y_zoom_sprite,
                                                                         self.state.idx_x_zoom_sprite+8*8,
                                                                         self.state.idx_y_zoom_sprite+8*8]):
                idx_x = math.floor((self.state.mouse_x - self.state.idx_x_zoom_sprite) / 8)
                idx_y = math.floor((self.state.mouse_y - self.state.idx_y_zoom_sprite) / 8)

                sset(self.state.x_zoom_sprite + idx_x, self.state.y_zoom_sprite + idx_y, self.pp.get_current_color())


        if self.state.mouse_y >= self.state.idx_sprites_batch and self.state.mouse_y < 120:
                y = math.floor((self.state.mouse_y - self.state.idx_sprites_batch) / 8)
                x = math.floor(self.state.mouse_x / 8)
                self.state.current_sprite = (x + y * 16) + 64 * self.state.idx_map
                self.state.x_zoom_sprite = (self.state.current_sprite % 16) * 8
                self.state.y_zoom_sprite = math.floor(self.state.current_sprite / 16) * 8

    def draw_zoom_sprite(self):
        sspr(self.state.x_zoom_sprite,
             self.state.y_zoom_sprite,
             8,
             8,
             self.state.idx_x_zoom_sprite,
             self.state.idx_y_zoom_sprite,
             8*8, 8*8)

    def draw_map(self):
        current_sprite_x = 0
        current_sprite_y = 0

        idx = self.state.idx_map * (4 * 16)
        y = self.state.idx_sprites_batch
        for j in range(0, 4):
            x = 0
            for i in range(0, 16):
                spr(idx, x, y)
                if idx == self.state.current_sprite:
                    current_sprite_x = x
                    current_sprite_y = y
                x += 8
                idx += 1
            y += 8

        current_sprite_x -= 1
        rect(current_sprite_x, current_sprite_y, current_sprite_x+8, current_sprite_y+8, 7)

        current_sprite_x -= 1
        current_sprite_y -= 1
        rect(current_sprite_x, current_sprite_y, current_sprite_x+10, current_sprite_y+10, 0)

    def draw_sprite_info(self):
        px8_print(str(self.state.current_sprite), 80, 64, 7)
        idx = 0
        for i in [1, 2, 4, 8, 16, 32, 64, 128]:
            flag = fget(self.state.current_sprite, i)
            color = 0
            if flag:
                color = 7

            circfill(80 + idx, 74, 1, color)

            idx += 4

    def draw(self):
        self.draw_zoom_sprite()
        self.draw_map()
        self.draw_sprite_info()

    def draw_button(self):
        for button in self.buttons_map:
            button.draw()

class PalettePicker(object):
    def __init__(self):
        self.idx_x = 80
        self.idx_y = 16
        self.current_color = 0
        self.current_selection_x = 0
        self.current_selection_y = 0

    def get_current_color(self):
        return self.current_color

    def update(self):
        _mouse_state = mouse_state()
        if _mouse_state == 1:
            _mouse_x = mouse_x()
            _mouse_y = mouse_y()

            if pointInRectangle(_mouse_x, _mouse_y, [self.idx_x, self.idx_y, self.idx_x+4*8, self.idx_y+4*8]):
                idx_x = math.floor((_mouse_x - self.idx_x) / 8)
                idx_y = math.floor((_mouse_y - self.idx_y) / 8)

                self.current_color = idx_x+idx_y*4
                self.current_selection_x = idx_x
                self.current_selection_y = idx_y

    def draw(self):
        idx = 0
        x = self.idx_x
        y = self.idx_y

        for i in range(0, 16):
            pos_x = x + (8 * (idx % 4))

            pos_y = y
            rectfill(pos_x, pos_y, pos_x+7, pos_y+7, i)
            idx += 1

            if idx > 1 and idx % 4 == 0:
                y += 8

        current_selection_x = (self.idx_x + 8*self.current_selection_x) - 1
        current_selection_y = (self.idx_y + 8*self.current_selection_y) - 1
        rect(current_selection_x, current_selection_y, current_selection_x+9, current_selection_y+9, 7)

pp = PalettePicker()
sm = SpritesMap(pp)

def _init():
    cls()

def _update():
    global sm, pp

    sm.update()
    pp.update()

def _end():
    return False

def draw_mouse():
    pass

def draw_contour():
    rectfill(0, 0, 128, 8, 8)
    rectfill(0, 120, 128, 128, 8)

    rectfill(0, 75, 128, 87, 5)
    rectfill(0, 9, 8, 77, 5)
    rectfill(75, 9, 128, 76, 5)


def _draw():
    global sm
    global pp

    cls()

    draw_contour()

    sm.draw()

    draw_mouse()

    sm.draw_button()
    pp.draw()
