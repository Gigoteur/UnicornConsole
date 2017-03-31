import random
from datetime import datetime

width=128
height=128

drawmode=0
drawmodes=["lines", "textured", "per-pixel"]

terrain=[0] * (width)
per_pixel_terrain=[[]] * (width)

global_randomness=30
initial_height=60

def regenerate(randomness):
    random.seed(datetime.now())
    generate_terrain(randomness)
    copy_terrain_to_per_pixel_terrain()

def generate_height_at_midpoint(left,right,randomness):
    if left < 128 and right < 128:
        terrain[int(flr((left+right)/2))] = (terrain[int(left)]+ terrain[int(right)])/2 +(rnd(1)*randomness-(randomness/2))

def generate_terrain(randomness):
    for i in range(0, width):
        terrain[i]=initial_height

    step=flr(width/2)
    while step >= 1:
        segmentstart=1
        while segmentstart<=width:
            left=segmentstart
            right=left+step
            if right>=width:
                right-=width
            generate_height_at_midpoint(left,right,randomness)
            segmentstart+=step

        randomness/=2
        step/=2

def copy_terrain_to_per_pixel_terrain():
    c = 1
    for x in range(0, width):
        ground_thickness=flr(rnd(3))
        per_pixel_terrain[x] = [0] * height
        height_here=terrain[x]

        for y in range(0, int(terrain[x])):
            if y>height_here-1:
                c=3
            elif y>height_here-(2+ground_thickness):
                c=11
            elif y>height_here-(3+ground_thickness):
                c=3
            elif y>height_here-(5+ground_thickness):
                c=1
            elif y>height_here-(7+ground_thickness):
                c=2
            elif flr(rnd(2))==0:
                c=2
            else:
                c=4

            per_pixel_terrain[x][y]=c

def _init():
    regenerate(global_randomness)
    #sspr(0, 0, 1, 32, 0, 64, 1, 32)

def _update():
    global global_randomness, drawmode
    if btnp(5):
        regenerate(global_randomness)

    if btnp(1):
        drawmode=(drawmode+1) % len(drawmodes)
    elif btnp(0):
        drawmode=drawmode-1
        if drawmode == -1:
            drawmode = 2

    if btnp(3):
        global_randomness=max(0,global_randomness-5)
        regenerate(global_randomness)
    elif btnp(2):
        global_randomness=min(200,global_randomness+5)
        regenerate(global_randomness)

def draw_with_lines():
    for i in range(0, width):
        line(i,128,i,128-terrain[i],3)

def draw_textured():
    for i in range(0, width):
        sspr(i%32,0,1,32,i,128-terrain[i])
        line(i,128,i,128-(terrain[i]-32),1)

def draw_per_pixel_terrain():
    for x in range(0, width):
        for y in range(0, height):
            if per_pixel_terrain[x][y] != 0:
                pset(x, (height)-y, per_pixel_terrain[x][y])

def _draw2():
    pass

def _draw():
    rectfill(0,0,128,128,12)
    if drawmode==0:
        draw_with_lines()
    elif drawmode==1:
        draw_textured()
    elif drawmode==2:
        draw_per_pixel_terrain()
