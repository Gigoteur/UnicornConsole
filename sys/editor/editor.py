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

        self.on_current_sprite_x = 0
        self.on_current_sprite_y = 0
        self.on_current_sprite = False

def point_in_rect(x, y, coord):
    return (coord[0] <= x <= coord[2] and
            coord[1] <= y <= coord[3])

class Widget(object):
    def __init__(self, name, x, y, data):
        self.name = name
        self.x1 = x
        self.y1 = y
        self.data = data
        self.clicked = False

        self.x2 = x
        self.y2 = self.y1 + len(data)
        if self.data:
            self.x2 = self.x1 + len(data[0])

    def is_click(self):
        return self.clicked

    def reset(self):
        self.clicked = False

    def update(self, x, y):
        self.clicked = (self.x1 <= x <= self.x2 and
                        self.y1 <= y <= self.y2)

    def draw(self):
        for y, row in enumerate(self.data):
            for idx, pixel in enumerate(row):
                pset(self.x1+idx, self.y1+y, pixel)

class SpritesMap(object):
    def __init__(self, state):
        self.state = state

        self.buttons = [96, 79, 115, 87]
        self.buttons_map = [Button(96, 79, 100, 87, 2, "1", True),
                            Button(101, 79, 105, 87, 2, "2"),
                            Button(106, 79, 110, 87, 2, "3"),
                            Button(111, 79, 115, 87, 2, "4")]

    def update(self):
        self.state.on_current_sprite = False
        self.state.mouse_state = mouse_state()
        self.state.mouse_x = mouse_x()
        self.state.mouse_y = mouse_y()

        if self.state.mouse_state == 1:
            if point_in_rect(self.state.mouse_x, self.state.mouse_y, self.buttons):
                for btn_idx, button in enumerate(self.buttons_map):
                    button.update(self.state.mouse_x, self.state.mouse_y)
                    if button.is_click():
                        self.state.idx_map = btn_idx
                        self.state.current_sprite = 64 * self.state.idx_map

            if self.state.mouse_y >= self.state.idx_sprites_batch and self.state.mouse_y < 120:
                y = math.floor((self.state.mouse_y - self.state.idx_sprites_batch) / 8)
                x = math.floor(self.state.mouse_x / 8)
                self.state.current_sprite = (x + y * 16) + 64 * self.state.idx_map
                self.state.x_zoom_sprite = (self.state.current_sprite % 16) * 8
                self.state.y_zoom_sprite = math.floor(self.state.current_sprite / 16) * 8

        if point_in_rect(self.state.mouse_x, self.state.mouse_y, [self.state.idx_x_zoom_sprite,
                                                                     self.state.idx_y_zoom_sprite,
                                                                     self.state.idx_x_zoom_sprite+8*8,
                                                                     self.state.idx_y_zoom_sprite+8*8]):
            idx_x = math.floor((self.state.mouse_x - self.state.idx_x_zoom_sprite) / 8)
            idx_y = math.floor((self.state.mouse_y - self.state.idx_y_zoom_sprite) / 8)
            self.state.on_current_sprite_x = self.state.x_zoom_sprite + idx_x
            self.state.on_current_sprite_y = self.state.y_zoom_sprite + idx_y
            self.state.on_current_sprite = True

    def draw_sprite_map(self):
        current_sprite_x = 0
        current_sprite_y = 0

        idx = self.state.idx_map * (4 * 16)
        y = self.state.idx_sprites_batch
        for j in range(0, 4):
            x = 0
            for _ in range(0, 16):
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

    def draw_sprite_flags(self):
        idx = 0
        for i in [1, 2, 4, 8, 16, 32, 64, 128]:
            flag = fget(self.state.current_sprite, i)
            color = 0
            if flag:
                color = 7

            circfill(80 + idx, 74, 2, color)

            idx += 6

    def draw(self):
        self.draw_sprite_map()
        self.draw_sprite_flags()
        self.draw_button()
        self.draw_information()

    def draw_button(self):
        for button in self.buttons_map:
            button.draw()

    def draw_information(self):
        if self.state.on_current_sprite:
            px8_print("x %d y %d" % (self.state.on_current_sprite_x, self.state.on_current_sprite_y), 0, 120, 5)

class PalettePicker(object):
    def __init__(self, state):
        self.state = state
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

            if point_in_rect(self.state.mouse_x, self.state.mouse_y, [self.state.idx_x_zoom_sprite,
                                                                         self.state.idx_y_zoom_sprite,
                                                                         self.state.idx_x_zoom_sprite+8*8,
                                                                         self.state.idx_y_zoom_sprite+8*8]):
                idx_x = math.floor((self.state.mouse_x - self.state.idx_x_zoom_sprite) / 8)
                idx_y = math.floor((self.state.mouse_y - self.state.idx_y_zoom_sprite) / 8)

                sset(self.state.x_zoom_sprite + idx_x, self.state.y_zoom_sprite + idx_y, self.get_current_color())

            if point_in_rect(_mouse_x, _mouse_y, [self.idx_x, self.idx_y, self.idx_x+4*8, self.idx_y+4*8]):
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

class SpriteEditor(object):
    def __init__(self, state):
        self.state = state
        self.pp = PalettePicker(state)

    def update(self):
        self.pp.update()

    def draw(self):
        self.pp.draw()

        px8_print(str(self.state.current_sprite), 80, 64, 7)

        sspr(self.state.x_zoom_sprite,
             self.state.y_zoom_sprite,
             8,
             8,
             self.state.idx_x_zoom_sprite,
             self.state.idx_y_zoom_sprite,
             8*8, 8*8)

class MapEditor(object):
    def __init__(self, state):
        self.state = state

        self.coord = [0, 8, 128, 78]
        self.offset_x = 0
        self.offset_y = 0
        self.available_zooms = [1, 1/2, 1/4]
        self.idx_zoom = 0
        self.zoom = self.available_zooms[self.idx_zoom]
        self.size_sprite = 8 * self.zoom
        self.select_field = [0, 8]

        self._cache = [0] * (128*32)

        for y in range(0, 32):
            for x in range(0, 128):
                self._cache[x + y * 128] = mget(x, y)

    def update(self):
        if btnp(0):
            self.offset_x -= 8
            self.offset_x = max(0, self.offset_x)
        if btnp(1):
            self.offset_x += 8
            self.offset_x = min(flr((128 - 16*self.zoom) * self.zoom), self.offset_x)
        if btnp(2):
            self.offset_y -= 8
            self.offset_y = max(0, self.offset_y)
        if btnp(3):
            self.offset_y += 8
            self.offset_y = min(flr((32 - 8*self.zoom) * self.zoom), self.offset_y)

        if btnp(4):
            self.idx_zoom = (self.idx_zoom + 1) % len(self.available_zooms)
            self.zoom = self.available_zooms[self.idx_zoom]
            self.size_sprite = 8 * self.zoom

        if point_in_rect(self.state.mouse_x, self.state.mouse_y, self.coord):
            self.select_field = [self.state.mouse_x - self.state.mouse_x % 8 * self.zoom,
                                 self.state.mouse_y - self.state.mouse_y % 8 * self.zoom]

            if self.state.mouse_state == 1:
                new_x = (self.select_field[0] + self.offset_x * self.size_sprite) / 8
                new_y = (self.select_field[1] + self.offset_y * self.size_sprite - 8) / 8
                idx = flr(new_x + new_y * 128)
                self._cache[idx] = self.state.current_sprite
                mset(new_x, new_y, self.state.current_sprite)

    def draw(self):
        rectfill(self.coord[0], self.coord[1], self.coord[2], self.coord[3], 0)
        self.draw_with_zoom()
        self.draw_select_field()
        px8_print("%d %d" % (self.offset_x, self.offset_y), 0, 120, 5)

    def draw_with_zoom(self):
        zoom = self.zoom

        idx_y = 0
        for y in range(self.offset_y, self.offset_y + flr(8/zoom)):
            idx_x = 0
            for x in range(self.offset_x, self.offset_x + flr(16/zoom)):
                offset = x + y * 128
                sprite_number = self._cache[offset]
                if sprite_number != 0:
                    sprite_x = (sprite_number%16) * 8
                    sprite_y = flr(sprite_number / 16) * 8

                    dx = idx_x * (8*zoom)
                    dy = idx_y * (8*zoom) + 9
                    sspr(sprite_x, sprite_y, 8, 8, dx, dy, flr(zoom*8), flr(zoom*8))

                idx_x += 1
            idx_y += 1

    def draw_select_field(self):
        rect(self.select_field[0],
             self.select_field[1],
             self.select_field[0] + 8 * self.zoom,
             self.select_field[1] + 8 * self.zoom,
             7)


class ToolsEditor(object):
    def __init__(self, state):
        self.state = state
        self.buffer_copy = []

        self.widgets = [
            Widget("ERASE", 0, 80, [
                [5, 6, 5, 5, 5, 5, 6, 5],
                [5, 5, 6, 5, 5, 6, 5, 5],
                [5, 5, 5, 6, 6, 5, 5, 5],
                [5, 5, 5, 6, 6, 5, 5, 5],
                [5, 5, 6, 5, 5, 6, 5, 5],
                [5, 6, 5, 5, 5, 5, 6, 5],
            ]),
            Widget("COPY", 8, 80, [
                [5, 5, 5, 6, 6, 6, 6, 6],
                [5, 6, 6, 6, 5, 5, 5, 6],
                [5, 6, 5, 6, 5, 5, 5, 6],
                [5, 6, 5, 6, 5, 5, 5, 6],
                [5, 6, 5, 6, 5, 5, 5, 6],
                [5, 6, 6, 6, 6, 6, 6, 6],
            ]),
            Widget("PASTE", 16, 80, [
                [5, 6, 6, 6, 6, 5, 5, 5],
                [5, 6, 5, 5, 6, 5, 5, 5],
                [5, 6, 5, 5, 6, 5, 5, 5],
                [5, 6, 5, 5, 6, 5, 5, 5],
                [5, 6, 5, 5, 6, 5, 5, 5],
                [5, 6, 6, 6, 6, 5, 5, 5],
            ])
        ]

    def update(self):
        if self.state.mouse_state == 1:
            for widget in self.widgets:
                widget.update(self.state.mouse_x, self.state.mouse_y)

        for widget in self.widgets:
            if widget.is_click():
                if widget.name == "ERASE":
                    for x in range(0, 8):
                        for y in range(0, 8):
                            sset(self.state.x_zoom_sprite + x, self.state.y_zoom_sprite + y, 0)
                if widget.name == "COPY":
                    if not self.buffer_copy:
                        self.buffer_copy = [0] * (8*8)

                    for x in range(0, 8):
                        for y in range(0, 8):
                            self.buffer_copy[x+y*8] = sget(self.state.x_zoom_sprite + x, self.state.y_zoom_sprite + y)

                if widget.name == "PASTE":
                    for x in range(0, 8):
                        for y in range(0, 8):
                            sset(self.state.x_zoom_sprite + x, self.state.y_zoom_sprite + y, self.buffer_copy[x+y*8])

            widget.reset()

    def draw(self):
        for widget in self.widgets:
            widget.draw()


class Editor(object):
    def __init__(self):
        self.state = State()

        self.windows = [SpriteEditor(self.state), MapEditor(self.state)]
        self.current_window = self.windows[0]
        self.tools = ToolsEditor(self.state)
        self.sm = SpritesMap(self.state)

        self.widgets = [
            Widget("SPRITE EDITOR", 110, 1, [
                [6, 8, 8, 8, 8, 8, 8, 6],
                [8, 6, 6, 6, 6, 6, 6, 8],
                [8, 6, 8, 8, 8, 8, 6, 8],
                [8, 6, 8, 8, 8, 8, 6, 8],
                [8, 6, 6, 6, 6, 6, 6, 8],
                [6, 8, 8, 8, 8, 8, 8, 6],
            ]),
            Widget("MAP EDITOR", 119, 1, [
                [8, 8, 8, 8, 8, 8, 8, 8],
                [8, 6, 6, 6, 6, 6, 6, 8],
                [8, 6, 8, 8, 8, 8, 6, 8],
                [8, 6, 8, 8, 8, 8, 6, 8],
                [8, 6, 6, 6, 6, 6, 6, 8],
                [8, 8, 8, 8, 8, 8, 8, 8],
            ])
        ]

    def draw_contour(self):
        rectfill(0, 0, 128, 8, 8)
        rectfill(0, 120, 128, 128, 8)

        rectfill(0, 75, 128, 87, 5)
        rectfill(0, 9, 8, 77, 5)
        rectfill(75, 9, 128, 76, 5)

    def update(self):
        self.sm.update()
        self.tools.update()
        self.current_window.update()
        if self.state.mouse_state == 1:
            for widget in self.widgets:
                widget.update(self.state.mouse_x, self.state.mouse_y)

            for widget in self.widgets:
                if widget.is_click():
                    if widget.name == "MAP EDITOR":
                        self.current_window = self.windows[1]
                    elif widget.name == "SPRITE EDITOR":
                        self.current_window = self.windows[0]

    def draw(self):
        cls()
        self.draw_contour()

        self.sm.draw()
        self.tools.draw()
        self.current_window.draw()
        for widget in self.widgets:
            widget.draw()

E = Editor()

def _init():
    cls()

def _update():
    E.update()

def _draw():
    E.draw()


