debug_print = print

function min(a,b)
    if a == nil or b == nil then
            warning("min a or b are nil returning 0")
            return 0
    end
    if a < b then
        return a
    end
    return b
end
function max(a,b)
    if a == nil or b == nil then
            warning("max a or b are nil returning 0")
            return 0
    end
    if a > b then
        return a
    end
    return b
end
function mid(x,y,z)
    x = x or 0
    y = y or 0
    z = z or 0
    return x > y and x or y > z and z or y
end
function __pico_angle(a)
  -- FIXME: why does this work?
  return (((a - math.pi) / (math.pi*2)) + 0.25) % 1.0
end
flr = math.floor
ceil = math.ceil
cos = function(x) return math.cos((x or 0)*(math.pi*2)) end
sin = function(x) return math.sin(-(x or 0)*(math.pi*2)) end
function atan2(y, x)
    return __pico_angle(math.atan(y, x))
end
sqrt = math.sqrt
abs = math.abs
sgn = function(x)
    if x < 0 then
        return -1
    else
        return 1
    end
end
band = function(x, y)
  x = math.floor(x)
  y = math.floor(y)
  return x & y
end
bor = function(x, y)
  x = math.floor(x)
  y = math.floor(y)
  return x | y
end
bxor = function(x, y)
  x = math.floor(x)
  y = math.floor(y)
  return x ~ y
end
bnot = function(x)
  x = math.floor(x)
  return ~x
end
shl = function(x, y)
  x = math.floor(x)
  y = math.floor(y)
  return x << y
end
shr = function(x, y)
  x = math.floor(x)
  y = math.floor(y)
  return x >> y
end

function warning(msg)
    log(debug.traceback("WARNING: "..msg,3))
end

function add(a,v)
    if a == nil then
        warning("add to nil")
        return
    end
    table.insert(a,v)
end
function del(a,dv)
    if a == nil then
        warning("del from nil")
        return
    end
    for i,v in ipairs(a) do
        if v==dv  then
        table.remove(a,i)
        end
    end
end
function foreach(a,f)
    if not a then
        warning("foreach got a nil value")
        return
    end
    for i,v in ipairs(a) do
        f(v)
    end
end
function count(a)
    return #a
end
function all(a)
    local i = 0
    local n = #a
    return function()
        i = i + 1
        if i <= n  then
            return a[i]
        end
    end
end
sub = string.sub

function tonum(data)
    if string.sub(data, 0, 2) == "0b" then
    local a,b=string.match(data,"(.*)%.(.*)$")
    if a == nil and b == nil then
       a=tonumber(string.sub(data, 3, #data), 2)
       return a
    end
    end
    
    return tonumber(data,10)
end
