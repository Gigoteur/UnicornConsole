pub mod lua_plugin;
pub mod python_plugin;
pub mod rpython_plugin;
pub mod rhai_plugin;
pub mod wasm_plugin;

/*
####################################### Unicorn Console API (Lua) ############################
#                       #    LUA    #     CPython   #    RPython    #   RHAI    #    WASM    #
# Input 
btn                     #     X     #       X       #               #           #            #
btnp                    #     X     #       X       #               #           #     X      #
mouse_x                 #           #       X       #               #           #            #
mouse_y                 #           #       X       #               #           #            #
mouse_state             #           #       X       #               #           #            #
mouse_statep            #           #       X       #               #           #            #
mouse_left_state        #           #       X       #               #           #            #
mouse_left_statep       #           #       X       #               #           #            #

# Palette
set_color_palette       #           #       X       #               #           #            #
reset_palette           #           #       X       #               #           #            #
reset_palette           #           #       X       #               #           #            #
switch_palette          #           #       X       #               #           #            #

# Math
atan2                   #     X     #       X       #               #           #            #
cos                     #     X     #       X       #               #           #            #
sin                     #     X     #       X       #               #           #            #
flr                     #     X     #       X       #               #           #            #
rnd                     #     X     #       X       #               #           #            #
srand                   #     X     #       X       #               #           #            #
mid                     #     X     #       X       #               #           #            #
bxor                    #     X     #       X       #               #           #            #

# System
time                    #     X     # unicorn_time  #               #           #            #
mtime                   #           # unicorn_mtime #               #           #            #
utime                   #           # unicorn_utime #               #           #            #
    
# Map                   
map                     #     X     #    mapdraw    #               #           #            #
mget                    #     X     #       X       #               #           #            #
mset                    #     X     #       X       #               #           #            #

# GFX
mode_width              #     X     #       X       #               #           #     X      #
mode_height             #     X     #       X       #               #           #     X      #
camera                  #     X     #       X       #               #           #            #
circ                    #     X     #       X       #               #           #     X      #
circfill                #     X     #       X       #               #           #            #
clip                    #     X     #       X       #               #           #            #
cls                     #     X     #       X       #               #           #     X      #
color                   #     X     #       X       #               #           #            #
ellipse                 #     X     #       X       #               #           #            #
ellipsefill             #     X     #       X       #               #           #            #
fget                    #     X     #       X       #               #           #            #
fillp                   #     X     #       X       #               #           #            #
fset                    #     X     #       X       #               #           #            #
fset_all                #     X     #       X       #               #           #            #
font                    #     X     #       X       #               #           #            #
line                    #     X     #       X       #               #           #            #
pal                     #     X     #       X       #               #           #            #
palt                    #     X     #       X       #               #           #            #
pget                    #     X     #       X       #               #           #            #
polygon                 #           #       X       #               #           #            #
print                   #     X     # unicorn_print #               #           #     X      #
pset                    #     X     #       X       #               #           #            #
rect                    #     X     #       X       #               #           #            #
rectfill                #     X     #       X       #               #           #            #
sget                    #     X     #       X       #               #           #            #
spr                     #     X     #       X       #               #           #            #
sset                    #     X     #       X       #               #           #            #
sspr                    #     X     #       X       #               #           #            #
trigon                  #     X     #       X       #               #           #            #

# Music
play_note               #     X     #       X       #               #           #            #
tigger_note             #     X     #       X       #               #           #            #
*/