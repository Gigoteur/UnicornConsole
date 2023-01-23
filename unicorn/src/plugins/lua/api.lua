function dget(index)
    return 0
end

function dset(index, value)
    return 0
end

function rnd(x)
  if x == nil then
    x = 1
  end

  if type(x) == "number" then
    x = math.floor(x)
    return userdata:rnd(x)
  end

  if type(x) == "table" then
    return x[math.random(#x)]
  end
end

function srand(x)
    if x == nil then
        x = 1
    end
    x = math.floor(x)
    return userdata:srand(x)
end

function time()
    return userdata:time()
end


function mtime()
    return userdata:mtime()
end


function utime()
    return userdata:utime()
end

function btn(i, p)
    if i == nil then
        i = 0
    end
    if p == nil then
        p = 0
    end
    return userdata:btn(i, p)
end

function btnp(i, p)
    if i == nil then
        i = 0
    end
    if p == nil then
        p = 0
    end
    return userdata:btnp(i, p)
end

function map(cel_x, cel_y, sx, sy, cel_w, cel_h, layer)
    cel_x = math.floor(cel_x)
    cel_y = math.floor(cel_y)
    sx = math.floor(sx)
    sy = math.floor(sy)
    cel_w = math.floor(cel_w)
    cel_h = math.floor(cel_h)
    if layer == nil then
        layer = 0
    end

    userdata:mapdraw(cel_x, cel_y, sx, sy, cel_w, cel_h, layer)
end

function mapdraw(cel_x, cel_y, sx, sy, cel_w, cel_h, layer)
    map(cel_x, cel_y, sx, sy, cel_w, cel_h, layer)
end

function mget(x, y)
    x = math.floor(x)
    y = math.floor(y)
    return userdata:mget(x, y)
end

function mset(x, y, v)
    x = math.floor(x)
    y = math.floor(y)
    v = math.floor(v)
    userdata:mset(x, y, v)
end

function mode_width()
    return userdata:mode_width()
end

function mode_height()
    return userdata:mode_height()
end

function camera(x, y)
    if x == nil then
        x = 0
    end
    if y == nil then
        y = 0
    end
    userdata:camera(x, y)
end

function circ(x, y, r, col)
    if col == nil then
        col = -1
    end
    userdata:circ(x, y, r, col)
end

function circfill(x, y, r, col)
    if col == nil then
        col = -1
    end
    userdata:circfill(x, y, r, col)
end

function clip(x, y, w, h)
    if x == nil then
        x = -1
    end
    if y == nil then
        y = -1
    end
    if w == nil then
        w = -1
    end
    if h == nil then
        h = -1
    end

    userdata:clip(x, y, w, h)
end

function cls(col)
    if col == nil then
        col = -1
    end
    userdata:cls(col)
end

function color(value)
    userdata:color(value)
end

function ellipse(x, y, rx, ry, col)
    if col == nil then
        col = -1
    end
    userdata:ellipse(x, y, rx, ry, col)
end

function ellipsefill(x, y, rx, ry, col)
    if col == nil then
        col = -1
    end
    userdata:ellipsefill(x, y, rx, ry, col)
end

function fillp(pat)
    transparent = false

    if pat == nil then
        pat = 0
    end

    if (pat % 1) ~= 0  then
        transparent = true
    end

    pat = math.floor(pat)

    userdata:fillp(pat, transparent)
end

function fget(idx, flag)                       
    if flag == nil then
        return userdata:fget_all(idx)
    end
    return userdata:fget(idx, flag)
end


function fset(idx, flag, value)                       
    if value == nil then
        return userdata:fset_all(idx, flag)
    end

    return userdata:fset(idx, flag, value)
end

function font(name)
    if name == nil then
        name = "pico8"
    end
    userdata:font(name)
end

function line(x0, y0, x1, y1, col)
    if col == nil then
        col = -1
    end
    userdata:line(x0, y0, x1, y1, col)
end

function pal(c0, c1, p)
    if c0 == nil then
        c0 = -1
    end
    if c1 == nil then
        c1 = -1
    end
    if p == nil then
        p = -1
    end
    userdata:pal(c0, c1, p)
end

function palt(c, t)
    if c == nil then
        c = -1
    end
    if t == nil then
        t = -1
    end
    userdata:palt(c, t)
end

function pget(x, y)
    return userdata:pget(x, y)
end


function pset(x, y, col)
    if col == nil then
        col = -1
    end
    userdata:pset(x, y, col)
end


function print(str, x, y, col)
    if x == nil then
        x = -1
    end
    if y == nil then
        y = -1
    end
    if col == nil then
        col = -1
    end
    userdata:print(str, x, y, col)
end

function rect(x0, y0, x1, y1, col)
    if col == nil then
        col = -1
    end
    userdata:rect(x0, y0, x1, y1, col)
end

function rectfill(x0, y0, x1, y1, col)
    if col == nil then
        col = -1
    end
    userdata:rectfill(x0, y0, x1, y1, col)
end

function sget(x, y)
    x = math.floor(x)
    y = math.floor(y)
    return userdata:sget(x, y)
end

function spr(n, x, y, w, h, flip_x, flip_y, angle, zoom, dynamic)
    n = math.floor(n)

    if x == nil then
        x = 0
    end
    if y == nil then 
        y = 0
    end

    x = math.floor(x)
    y = math.floor(y)

    if w == nil then
        w = 1
    end
    if h == nil then
        h = 1
    end
    if flip_x == nil then
        flip_x = false
    end
    if flip_y == nil then
        flip_y = false
    end
    if angle == nil then
        angle = 0.0
    end
    if zoom == nil then
        zoom = 1.0
    end
    if dynamic == nil then
        dynamic = false
    end

    userdata:spr(n, x, y, w, h, flip_x, flip_y, angle, zoom, dynamic)
end

function sspr(sx, sy, sw, sh, dx, dy, dw, dh, flip_x, flip_y)
    sx = math.floor(sx)
    sy = math.floor(sy)
    sw = math.floor(sw)
    sh = math.floor(sh)
    dx = math.floor(dx)
    dy = math.floor(dy)

    if dw == nil then
        dw = sw
    end

    if dh == nil then
        dh = sh
    end

    if flip_x == nil then
        flip_x = false
    end
    if flip_y == nil then
        flip_y = false
    end         

    userdata:sspr(sx, sy, sw, sh, dx, dy, dw, dh, flip_x, flip_y)
end

function sset(x, y, color)
    x = math.floor(x)
    y = math.floor(y)
    if color == nil then
        color = -1
    end
    color = math.floor(color)

     userdata:sset(x, y, color)
end

function music(n, fadems, channelmask)
end

function sfx(n, channel, offset, length)
end


function play_note(note_idx, instrument_idx, channel)
    userdata:play_note(note_idx, instrument_idx, channel)
end

function trigger_note(note_idx, instrument_idx)
    userdata:trigger_note(note_idx, instrument_idx)
end

function stat(val)
    return ""
end

function poke(addr, value)
    debug_print("POKE NOT IMPLEMENTED", addr, value)
end