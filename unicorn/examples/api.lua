cls() -- clear screen
rect(0,0,127,127,1)

-- play music from pattern 02
-- (loop back flag set in pat 13)
music(0)

-- play sound 0 on channel 3
sfx(0, 3)

-- draw palette
x=3
rectfill(1,1,7,7,5)
for i=0,15 do
    print(i,x,2,i)
    x = x + 6 + flr(i/10)*4
end

-- gfx shapes
camera(-10,0) -- shift draws
rectfill(10,10,18,20,8)
rect(20,10,28,20,9)
circfill(35,15,5,10)
circ(45,15,5,11)
line(52,10,58,20,12)
pset(60,10,13)
pset(62,10,pget(60,10)+1)
clip(72,10,8,10)
rectfill(0,0,127,127,6)
clip()

-- sprites

rectfill(60,24,85,33,15)
spr(1,10,25)
spr(1,20,25,1,1,true)
pal(15,12) -- remap 15 to 12
sspr(8,0,8,8, 30,25,24,8)
pal(15,0) -- draw solid black
pal(1,8)
sspr(8,0,8,8, 60,25,24,8)
pal() -- reset palette mapping

-- map

mset(3,3,mget(1,3)+2)
map(0,3,10,35,4,3,1)
fset(6,1,true) --pink flower
-- only cels with flag 2 set
map(0,3,50,35,4,3,2)

-- set cursor position and color
cursor(0,90)
color(7)

-- math
cursor(0,70)
x = cos(0.125) + mid(1,2,3)
x = (x%1) + abs(-1)
print("x: "..x)


-- collections and strings
a={}
add(a, 11) add(a, 12)
add(a, 13) add(a, 14)
del(a, 13)
str="a: "
for i in all(a) do
    str=str..i.." "
end
print(str)

-- foreach and anon functions
total=0
foreach(a,
    function(i)
        total=total+i
    end
)
print("total: "..total)


-- callbacks and input
-- pico-8 automatically loops
-- and calls _update() and
-- _draw() once per frame if
-- they exist

t=0
function _update()
    t=t+1
end

function _draw()

    -- show current value of t
    rectfill(0,90,30,96,5)
    print("t: "..t,1,91,7)

    -- show state of buttons

    print("buttons: ", 1,101,7)

    for p=0,7 do
        for i=0,5 do
            col=5
            if(btn(i,p)) then col=8+i end
            rectfill(40+i*10,100+p*3,
                48+i*10, 101+p*3, col)
        end
    end

end