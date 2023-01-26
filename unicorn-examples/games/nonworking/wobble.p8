pico-8 cartridge // http://www.pico-8.com
version 36
__lua__
-- wobblepaint v1.6
-- by zep

-- info: lexaloffle.com/bbs/?pid=wobblepaint

-- tab 5: load & draw snippets!

-- uncomment to save/load to a
-- different cartridge:
-- data_cart = "wobdat2.p8"

--shortcut keys
kk={
 '1','2', -- change color
 'w','r', -- change item
 's','f', -- undo, redo
 'e','d', -- adjust brush size
 '\09'    -- tab: toggle menu
 }

-- where to save data
data_offset  = 0x400
max_data_len = 0x3f00

do_wobble=true -- interface
use_devkeys = true -- mouse
debug_wob_encode=false
tray_col=2
tray_brush=1
brush_col = 14
cop = 0 -- 1 drawing 2 dragmenu
recording_gif=0

lmb=0
lx,ly=0,0
menu_y=-40 -- start compact

ci=1
lib={{}}
scn=lib[ci]  -- current scn
scn.back_col=1

function _init()

 poke(0x5f2d, 1)
 
 cpat={[0]=0,0,0,0}
 cpal={}
 for i=0,3 do
  cpal[i]=peek(0x214+i)
 end
 
 -- size,shape,dotted,noise
 -- grab from preset dat
 
 cbp={[0]=0,1,2,3} -- brush preset
 csize,cshape,cdotted,cnoise
  ={},{},{},{}

 for i=0,3 do
  local dat=bp_data[cbp[i]]
  csize[i]=dat[1]
  cshape[i]=dat[2]
  cdotted[i]=dat[3]
  cnoise[i]=dat[4]
 end
 
 init_undo()
 
 lib=load_library()
 
 -- skip to first unused
 local ii=1
 while (lib[ii] and 
  #lib[ii] > 0 and ii<99)
 do ii+=1 end
 go_item(ii)

--[[
 if (ii>1) then 
  show_msg(40,
   "loaded "..(ii-1).." items")
 end
]]
 
 -- create menu item 
 toggle_devkeys()
 toggle_devkeys()
end


function toggle_devkeys()
	menuitem(1)
	use_devkeys=not use_devkeys
	
	menuitem(1,
	 use_devkeys and
	  "turn off mouse" or
	  "turn on mouse",
	 toggle_devkeys)
end


function _update()

 local ml =3 -- min len
 local ml2=8 -- (2^2+2^2)
 
 if (use_devkeys) then
  process_devkeys()
 else
  process_btns()
 end
 update_system()
  
 mb=stat(34)
 mx=mid(0,stat(32),127)
 my0=mid(0,stat(33),127)
 
 -- everything in canvas space
 my=get_canvas_y(my0)
 
 -- button press
 -- note: adding a button also
 -- counts as a click (need for
 -- setting bcakground color)
 if (mb>0 and lmb<mb) then

  -- don't draw if menu
  local i,j=menu_item(mx,my0)
  
  if (i>=0) then
   cop=0 -- maybe become 2
   process_button(mx,my0,mb)
   lmb=mb
   return
  else
   
   -- pick up colour  
   if (mb>1) then
     -- adjust for menu position and
     -- canvas offset (menu_y/2)
     cpal[tray_col]=
      pget(mx,my+(menu_y/2+26))
    return
   end
   
  end
  
  cop=1 -- drawing
  
  lx=mx
  ly=my
  
  crv={} -- global
  add(scn,crv)
  
  crv.col=brush_col()
  crv.pat=brush_pat()
  crv.size=brush_size()
  crv.shape=brush_shape()
  crv.dotted=brush_dotted()
  crv.noise=brush_noise()
  
  crv[1]=0 -- dummy
  
  -- safety
  
  mx=mid(0,flr(mx),127)
  my=mid(0,flr(my),127)
  
  add(crv,mx)
  add(crv,my)
  
  
  redo_stack[ci]={} -- clear redo
  
  ml=-1
  
 end
 
 if (cop==1) then
	 
	 local dx=mx-lx
	 local dy=my-ly
	 local dd=dx^2 + dy^2
	 local maxout=64
	 while (dd>ml2 and lmb>0 and
	 maxout>0) do
	 
	  maxout-=1 -- safety
	  
	  -- jump to a spr40 pixel
	  -- -> readjust mxy/lxy
	  -- match drawing logic!
	  local a=atan2(dx,dy)
	  a = flr(a*16+0.5)/16
	  
	  lx+=flr(0.5+cos(a)*3)
	  ly+=flr(0.5+sin(a)*3)
   
   -- safety
   lx=mid(0,flr(lx),127)
   ly=mid(0,flr(ly),127)
   
   add(crv,lx)
	  add(crv,ly)
	  
	  dx=mx-lx
		 dy=my-ly
		 dd=dx^2 + dy^2
		 
	 end
	 
	end
	
	if (cop==2) then
	 menu_y=menu_y0+(my0-drag_y0)
	 menu_y=mid(0,menu_y,-40)
	end
 
 lmb=mb
 
 -- no cursor operation
 if (mb==0) cop=0
 
 lmx=stat(32)
 lmy=stat(33)
 
end

local function r1()
 if (not do_wobble) return 0
 --return -.1+rnd(1.2)
 return rnd(1)-0.5
 
end

function get_canvas_y(y)
 local yy= y-(40+menu_y)/2
 yy = mid(0,yy-6,127)
 --printh(yy)
 return yy
end
 
msg=""
msg_t=0
wob_t=0

function _draw()
 cls(scn.back_col)
 
 --spr(0,0,0,16,16)
 --local q = flr(time()*6)
 local q = wob_t\1
 
 -- draw scene centered
 --camera(0,get_canvas_y(0))
 camera(0,-(menu_y+52)/2)
 wob_draw(scn,0,0,wob_t\1)
 wob_t+=0.2
 camera()
 
 -- menu
 
 draw_menu(0,menu_y,q)
 
 -- cursor
 
 mx=mid(0,stat(32),127) --+r1()
 my=mid(0,stat(33),127) --+r1()
 
 if (stat(34)>1) then
  -- picking up color;
  -- don't draw cursor
 elseif (recording_gif>0) then
  recording_gif-=1
  if (recording_gif==0) then
   extcmd("video")
   menu_y=last_menu_y
  end
 elseif (menu_item(mx,my)>=0) then
  -- menu pointer

  if (cop==0) then
  draw_menu_tip(menu_item(mx,my))
  end
  
  -- mouse cursor
  spr(30+(min(1,mb)),mx-2,my)

 else
	 
	 -- draw brush cursor
	 
	 local bcol=brush_col()%16
	 
	 if (bcol==pget(mx,my)
	     and not drawing) then
	  bcol+=2 end
	  
	 if (stat(34)==0 and
	  mx<127 and my<127
	 ) then
		 
		 local r=brush_size()
		 local s=brush_shape()
		 if (r==0) then
		 
		  pset(mx,my,bcol)
		 --[[
		  line(mx-3,my,mx-2,my,bcol)
		  line(mx+3,my,mx+2,my,bcol)
		  line(mx,my-3,mx,my-2,bcol)
		  line(mx,my+3,mx,my+2,bcol)
		 ]]
		 elseif (s==2) then
		  line(mx-r/2,my,mx+r/2,my,bcol)
		 elseif (s==3) then
		  line(mx,my-r/2,mx,my+r/2,bcol)
		 elseif (r==1) then
		  rectfill(mx,my,mx+1,my+1,bcol)
		 else
		  r-=1
		  if (s<2) then
		   circ(mx,my,r,bcol) 
		  elseif (s==4) then
		   rect(mx-r,my-r,mx+r,my+r,bcol)
		  else
		   line(mx-r,my,mx-2,my,bcol)
			  line(mx+r,my,mx+2,my,bcol)
			  line(mx,my-r,mx,my-2,bcol)
			  line(mx,my+r,mx,my+2,bcol)
		  end
		 end

	 end
 end

 
 -- system message
 if (msg_t>0) then
  msg_t = max(0, msg_t-1)
  
  if (msg2) then
	  local sx=10
	  local sy=64-10
	  
	  rectfill(sx,sy,sx+107,sy+20,0)
	  rect(sx+1,sy+1,sx+106,sy+19,14)
	  
	  print("🐱",sx+6,sy+8,6)
	  print(msg,sx+18,sy+4,7)
	  print(msg2,sx+18,sy+11,14)
	 else
   -- lil message
   local w=#msg*4+8
   local sx=64-w/2
   local sy=116--menu_y+52
   
   rectfill(sx,sy,sx+w,sy+10,0)
	  rect(sx+1,sy+1,sx+w-1,sy+9,14)
	  print(msg,65-#msg*2,sy+3,7)
   
	 end
 end
 

 -- test encode/decode
 
 if (btn(🅾️) and debug_wob_encode) then
 
 len,lenb=wob_save(scn,0x6400)
 
 if (#scn>0 and scn[#scn]) then
  print(((#scn[#scn]-1)/2)..
    " segments",2,112,6)
 end
 
 print(len.." bytes "..lenb.." bits",2,120,7)
 
 -- draw histogram
 if (debug_wob_encode) then
	 for i=-8,7 do
	  line(i+10,100,i+10,100-count_da[i],7)
	  pset(i+10,100,i+8)
	 end
 
	 scn2 = wob_load(0x6400)
	 
	 --len2=scn_to_dat(scn2,0x6440)
	 camera(-64,get_canvas_y(0))
	 draw_scn(scn2,q)
	 camera()
	 
	 -- stress test
	 -- if bug, get feedback loop
	 --scn=scn2 
	 
 end -- debug
 
 end
 
end

-- includes col2
function brush_col()
 return cpal[tray_col]%256
end


function brush_pat()
 return cpat[tray_col]
end

function brush_size()
 return csize[tray_brush]
end

function brush_shape()
 return cshape[tray_brush]
end

function brush_dotted()
 return cdotted[tray_brush]
end

function brush_noise()
 return cnoise[tray_brush]
end

function brush_preset()
 return cbp[tray_brush]
end

-->8
-- menu

-- rounded rectfill
function rrectfill(x0,y0,x1,y1,c)
 rectfill(x0,y0+1,x1,y1-1,c)
 line(x0+1,y0,x1-1,y0,c)
 line(x0+1,y1,x1-1,y1,c)
end

function menu_item(x,y)
 
 if (y>menu_y+52) return -1,-1
 -- drawing
 if (cop==1) return -1,-1
 if (menu_y<=-50) return -1,-1

 y-=menu_y
 
 --if (x>=87) x-=4
 if (x>=40 and x<44) then
  x-=2
 end
 if (x>=44) x-=4
 if (x>=80 and x<84) then
  x-=2
 end
 if (x>=84) x-=4
 
 
 return 
  mid(0,flr(x/10),11),
  mid(0,flr(y/10),4)

end


function save_gif()
 recording_gif=62 -- v1.5 (was 30) gives exactly 60 frames
 wob_t=0 -- v1.5: fixed uneven frame lengths


 last_menu_y=menu_y
 menu_y = -52 -- hide
 
 _draw()flip()
 extcmd("rec")

end

-- assume no joystick input
function process_devkeys()
 if (not use_devkeys) return

 -- mousewheel
 local dd=stat(36)
 if (dd!=0) then
  resize_brush(dd)
 end

 while (stat(30)) do
  local c=stat(31)
  
 
 
  if (c=="\218" or -- pre-emptive ctrl
      c=="\215" or -- cut
      c=="\194"   -- copy
      ) then
   
   memset(data_offset,0,max_data_len)
   local len=wob_save(lib[ci],data_offset)
   printh(
    "wobdat=\""..
    mem_to_str(data_offset,len).."\"",
    "@clip")
   
   if (c=="\194") then
	   show_msg(30,"copied") 
	  end
	  
	  -- doubles as "clear"
	  if (c=="\215") then
	   show_msg(30,"cut")
	   scn={
	    ["back_col"]=scn.back_col
	   }
	  end
	 
  end
  
  -- ctrl-z, ctrl-y
  if (c=="\217") undo()
  if (c=="\216") redo()
  
  -- paste
  if (c=="\213") then
   
   local str=stat(4)
   
   if (sub(str,1,8)=="wobdat=\"")
   then
   	str=sub(str,9)
   	str=sub(str,1,#str-1)
   end
   
   -- test if is valid hex string
   if (tonum("0x"..sub(str,1,1))) then
   
   	str_to_mem(str,0x4300)
    lib[ci]=wob_load(0x4300)
    redo_stack[ci]={} -- clear redo
    go_item(ci)
   
    show_msg(30,"pasted")
   
   else
    show_msg(30,"paste failed","ctrl-c to copy")
   
   end
   
   
  end
    
  
  -- can't use! reserved!
  -- but maybe in binary export? hrm.
  -- ** pico-8 save will compete
  -- with cstore()! **
  --if (c=="\210") printh("save")
  
  
  -- same as gfx editor
  if (c==kk[1]) then 
   cpal[tray_col]=
   band(cpal[tray_col],0xf0)+
   (cpal[tray_col]-1)%16
  end
  
  if (c==kk[2]) then 
   cpal[tray_col]=
   band(cpal[tray_col],0xf0)+
   (cpal[tray_col]+1)%16
  end
  
  if (c==kk[3]) go_item(ci-1)
  if (c==kk[4]) go_item(ci+1)
  
  if (c==kk[5]) undo()
  if (c==kk[6]) redo()
  
  if (c==kk[7]) resize_brush( 1)
  if (c==kk[8]) resize_brush(-1)
  
  -- tab
  if (c==kk[9]) then
   toggle_menu_visible()
  end
  
 end

end

function show_msg(tt, m, m2)
 msg_t=tt
 msg=m
 msg2=m2
end

function process_btns()
 
 -- shortcuts when not over menu
 if (msy > menu_y+52 and btn(🅾️)) 
 then
	
	 if (btnp(⬅️)) undo() show_msg(30,"undo")
	 if (btnp(➡️)) redo() show_msg(30,"redo")
	 if (btnp(⬆️)) resize_brush(1) show_msg(30,"size:"..brush_size())
  if (btnp(⬇️)) resize_brush(-1) show_msg(30,"size:"..brush_size())
  if (btnp(❎)) toggle_menu_visible()
 
 end
 
end



function toggle_menu_visible()
 if (menu_y == -52) then
  menu_y=last_menu_y
 else
  last_menu_y=menu_y
  menu_y=-52
 end
end
  
function resize_brush(dv)
 csize[tray_brush] =
  mid(0,csize[tray_brush]+dv,31)
end



function process_button(x,y,mb)

 i,j=menu_item(x,y)
 
 if (i<4 and mb>1) return
 if (i>7 and mb>1) return
 
 -- can drag from anywere:
 -- only if stay still on click
 if (x==lmx and y==lmy and x>42 or y < menu_y+32)
 then
  cop=2 -- dragging
	 drag_y0=y
	 menu_y0=menu_y
 end 

 
 --===== bottom row ======--
 
 if (j==4) then 
 
  if (i==0) then
   cop=2 -- dragging
   drag_y0=y
   menu_y0=menu_y
  end

 
  if(i==1) save_gif()
  if(i==2) undo()
  if(i==3) redo()
		
  if (i>=4 and i<8) then
   tray_col = i-4
  end
  
  if (i>=8 and i<12) then
   tray_brush = i-8
  end
  
 end
 
 --===== j 0..2 ======--
 
 if (j==3 and i < 4) then
  if (i==0) go_item(ci-1)
  if (i==2) go_item(ci+1)
  
  
  
  -- save single to clip
  -- use ctrl-c instead
--[[
  if (i==1) then
   memset(data_offset,0,max_data_len)
   local len=wob_save(lib[ci],data_offset)
   printh(
    "wobdat=\""..
    mem_to_str(data_offset,len).."\"",
    "@clip")
   msg=("copied "..len.." bytes")
   msg2="to clipboard"
   msg_t=60
  end
]]

  --save library
  if (i==3) then
   local len=save_library(lib)

--  also copy whole library to clip
--  printh(mem_to_str(data_offset,len),
--   "@clip")

		 if (len > 0) then
    show_msg(60,
     "saved "..len.." bytes",
     tostr(flr(len/(max_data_len/100))).."% of storage used"
    )  
   end
  end
  
 end
 
 -- select pattern 
 if (j<3 and i>=0 and i<4) then
  
 local cp=cpat[tray_col]

 -- toggle variation
 if (cp == read_pat(i+j*4,0))
 then
  preset_pat[i+j*4] = 
   read_pat(i+j*4,1)
 elseif (cp == read_pat(i+j*4,1)) then
  preset_pat[i+j*4] = 
   read_pat(i+j*4,0)
 end
 
 cpat[tray_col] = 
   preset_pat[i+j*4]
   
 end
 
 -- select color
 if (j<4 and i>=4 and i<8) then
  local c=i-4 + j*4
  if (mb==1) then
   cpal[tray_col] = c+c*16
  elseif (mb==2) then
   cpal[tray_col] %= 16
   cpal[tray_col] += c*16
  elseif (mb==3) then
   scn.back_col = c
  end
  
 end
 
 -- select brush preset (1..)
 if (j<4 and i>=8 and i<12) then

  local w=(i-8)+j*4
  -- copy preset data
  local dat=bp_data[w]
  local b=tray_brush
  cbp[b]=w
  csize[b]=dat[1]
  cshape[b]=dat[2]
  cdotted[b]=dat[3]
  cnoise[b]=dat[4]
 end
 
end

function draw_menu(x,y,q)

srand(q)

rectfill(x,y,x+127,y+10*5,0)
camera(0,-y)


local mmx,mmy=menu_item(
  stat(32),stat(33))
hover= (mmx==i and mmy==j)

 
 local mmx,mmy=menu_item(
  stat(32),stat(33))
  
 for j=0,4 do
  for i=0,11 do
   sx=i*10+r1()
   sy=j*10+r1()
   if (i>=4)sx+=4
   if (i>=8)sx+=4
   
   -- debug
   --rectfill(sx+1,sy+1,sx+8,sy+8,i+j)
   
   if (mmx==i and mmy==j) then
    rrectfill(sx+1,sy+1,sx+9,sy+9,1)
   end

  end

 --return
end


 local pt=preset_pat
 
 
 
 -- fill patterns
 
 for y=0,2 do
 for x=0,3 do
  local sx=1+x*10 +r1()
  local sy=1+y*10 +r1()
  local c=brush_col()
  local fill=pt[x+y*4]
  if (c%16==flr(c/16)) then
   fill+=0.5
  end
  fillp(fill)
  
  rectfill(sx,sy,sx+7,sy+7,
   pt[x+y*4] == brush_pat()
    and 7 or 6
  )
 end
 end
 fillp()
 
 -- navigator
 for x=0,3 do
  local sx=x*10+1 +r1()
  local sy=31 +r1()
  
  if (x==1) then
   local str=tostr(ci)
   if (#str==1) str="0"..str
   print(str,sx+1+r1(),sy+2+r1(),7)
  else
   
   spr(20+x,sx,sy)
   
  end
 end
 
 -- colors
 
 for y=0,3 do
 for x=0,3 do
  local sx=x*10+45 +r1()
  local sy=y*10 +r1()
  local c=x+y*4
  
  circfill(sx+4,sy+4,3,c)
  --rectfill(sx,sy,sx+7,sy+7,c)
 end
 end
 
 -- brush presets
 
 for y=0,3 do
 for x=0,3 do
  local sx=89+x*10 +r1()
  local sy= 1+y*10 +r1()
  local c=x+y*4
  pal(7,6)
  spr(x+y*4,sx,sy)
  pal()
 end
 end

 -- draw bottom row
 
 camera(0,-4*10+1-y)
 
 -- buttons
 
 for i=0,3 do
  spr(16+i,1+i*10+r1(),2+r1())
 end
 
 
 -- tray colors
 
 for i=0,3 do
  ss= (i == tray_col )
    and 0 or -1
  
  local c=cpal[i]
  local fill=cpat[i]
  if (c%16==flr(c/16)) then
   fill+=0.5
  end
  fillp(fill)
  
  if (i != tray_col)
  then
  	circfill(49+i*10+r1(),
           6+r1(),
           2,cpal[i])
  else
 	 rrectfill(45+i*10-ss +r1(),
  		3-ss +r1(),
    53+i*10+ss +r1(),
    9+ss +r1(),
    cpal[i])
  end
  fillp()
 
  -- blue outline if black
  if cpal[i]==0 then
   circ(50+i*10+r1(),
        6+r1(),4+ss,1)
  end
  
  
 end
 
 -- tray brushes
 for i=0,3 do
 
  sel= i == tray_brush
  local bgcol=0
  local fgcol=7--brush_col()
  if (sel) bgcol=scn.back_col
  
  if (bgcol==fgcol and bgcol!=0) fgcol+=1
  
  if (bgcol != 0) then
   rrectfill(89+i*10 +r1(),
    3 +r1(),
    97+i*10 +r1(),
    9 +r1(),
    bgcol)
  end
		pal(7,fgcol)
		
  spr(cbp[i],
      88+i*10+5-4 +r1(),
         6-4 +r1())
    
  pal()
  
--  circfill(87+i*10+5,y+6,
--     i,brush_col())
 end
 
 
 camera()

end

function draw_menu_tip(i,j)
	--rectfill(0,12,127,20,0)
	
	-- cols, crush are obvious
 
 if (j < 3 or j > 4) return
 if (i >=4) return
 if (msg_t>0) return
 
	local strs=
	{
		{nil,nil,nil,"save all"},
		{nil,"save gif","undo","redo"}
	}
	
	local s=strs[j-2][i+1]
	local x=i*10 + 2
	local y=menu_y+53
	--if (j ==3) y-=30
	
	if (s) then
	rectfill(x-2,y-1,x+#s*4,y+5,0)
	print(s,x,y,7)
	end
	
end





--------- undo -------------

function init_undo()
	
	redo_stack={}
	for i=1,99 do
	 add(redo_stack,{})
	end

end

function undo()
 if (#scn == 0) return
 add(redo_stack[ci],scn[#scn])
 scn[#scn]=nil
end

function redo()
 if (#redo_stack[ci] == 0) return
 add(scn,redo_stack[ci][#redo_stack[ci]])
 redo_stack[ci][#redo_stack[ci]]=nil
end



-->8
--system


-- rewrite stat to allow mouse
-- controlled by keyboard

_stat = stat
msx=64
msy=64
msdx=0
msdy=0

msb=0
function stat(x)

 if (use_devkeys) return _stat(x)

 -- replacements for mouse / kbd
 if (x==32) return flr(msx)
 if (x==33) return flr(msy)
 if (x==34) return msb
 
 if (x==30) return false
 if (x==31) return ""

 return _stat(x)

end

function update_system()
 if (use_devkeys) then
  msx=stat(32)
  msy=stat(33)
  return
 end
 local accel=0.8
 msb=0
 if (not btn(🅾️)) then
	 if (btn(⬅️)) msdx -= accel
	 if (btn(➡️)) msdx += accel
	 if (btn(⬆️)) msdy -= accel
	 if (btn(⬇️)) msdy += accel
 end
 
 if (not btn(🅾️) or msy<menu_y+52) then
  if (btn(❎) and not block_x) msb+=1
 end
 
 msx+=msdx
 msy+=msdy
 msdx*=0.7
 msdy*=0.7
 
 if (msx<0 or msx>=128) msdx=0
 if (msy<0 or msy>=128) msdy=0
 msx=mid(0,msx,127)
 msy=mid(0,msy,127)
 
 
 if (cop==0 and msy<menu_y+52) then
  if (btn(🅾️)) msb+=2
 end

 -- stop accidentally drawing
 -- when use ❎+🅾️ shortcut
 if (msy>menu_y+52) then
  if (btn(🅾️)) block_x=true
 end
 
 if (not btn(❎) and not btn(🅾️))
 then block_x=false end
 
end


function load_library()

 local lib={{["back_col"]=1}}
 
 reload(data_offset,data_offset,max_data_len,
	  data_cart)
 
 local src=data_offset

 local i=0
 while (peek2(src)>0) do
	 i+=1
	 lib[i]=wob_load(src)
	 src+=peek2(src)
 end

 return lib
end

function save_library(lib)

 -- mirror palette values
 for i=0,3 do
  poke(0x214+i,cpal[i])
 end

-- write all
local dest=data_offset

memset(data_offset,0,0x4300-data_offset)

for i=1,#lib do
 local len=wob_save(lib[i],dest)
 dest += len
end

-- end data marker
-- (read as item size 0)
poke2(dest,0)
dest+=2


if ((dest-data_offset) > 
    max_data_len) then
    
 show_msg(90,
  "** too much data! **",
  (dest-data_offset).." / "..max_data_len.." bytes")   
 
 return 0
end

if (data_cart) then
 cstore(data_offset,data_offset,max_data_len,data_cart)
else
 cstore() -- includes cpal
end

 return dest-data_offset
end

function mem_to_str(src,len)
 local str=""
 for i=0,len-1 do
  str=str..
   sub(tostr(peek(src+i),1),
    5,6)
 end
 return str
end




function go_item(i)

 ci=mid(1,i,99)
 if (not lib[ci] or #lib[ci]==0) then
  lib[ci]={}
  if (lib[ci-1]) then
   lib[ci].back_col=lib[ci-1].back_col
  else
   -- first item
   lib[ci].back_col=1
  end
 end
 scn=lib[ci]
  
end
-->8
-- encode

--[[

 █ format
 
 scene:
 
	 16 bits: data length in bytes
	          // can use poke2
	  4 bits: background color
	  n bits: curves
	  
	 // in library, terminate
	 // scene list with a scene
	 // of data length:0
	
 curve header:
 
 // 3 bits overhead
 // grouped by co-varying attrs
 
 1 change col/size
  4 col  0..15
  5 size 0..31
  
 1 change extra
  3 shape
  4 dotted (thirds)
  3 noise  (thirds)
 
 1 pat
  16 pat
   4 col2 

 7 start_x
 7 start_y
 1 has_segments (not just a dot)

 segment:
  1:
    0:continue
    1:turn
    
    1 sgn (0 == right)
    1..8 string of 0 end in 1
    // -8..-1,1..7 means turn
    // 8 means end of curve
]]

function wob_save(scn,dest)

 local dest0=dest
 local x0=0
 local y0=0
 
 local bit=1 
 local byte=0
 
 local lcol     = 0
 local lshape   = 0
 local lpat     = 0
 
 
 count_da={}
 for i=-8,7 do
  count_da[i]=0
 end
 draw_da=0

 short=0
 long=0
 
 function putbit(bval)
	 if (bval) byte+=bit 
	 poke(dest, byte) bit*=2
	 if (bit==256) then
	  bit=1 byte=0
	  dest += 1			
	 end
	end
	
	function putval(val, bits)
	 if (bits == 0) return
	 for i=0,bits-1 do
	  putbit(band(val,shl(1,i))!=0)
	 end
	end
	
	-- 2 byte header: len
	-- makes it easy to read and
	-- skip over appended sections
	dest += 2
	
	-- back color
	putval(scn.back_col,4)
	
 for j=1,#scn do
  
  local crv=scn[j]
 	
 	-- curve header
 	
 	-- vals for detecting change
 	-- (any val ok as long as
 	-- each state is unique)

 	local col,shape =
 	 crv.size+(crv.col%16)*16,
 	 crv.shape+crv.dotted*8+crv.noise*8
 	
 	if (col!=lcol) then
   putval(1,1)
	 	putval(crv.col%16,4)
	  putval(crv.size,5)
  else
   putval(0,1)
  end
  
  if (shape!=lshape) then
   putval(1,1)
	 	putval(crv.shape,3)
	 	putval(crv.dotted,4)
	 	putval(crv.noise,3)
  else
   putval(0,1)
  end
  
  -- "using pattern"
  
  if (crv.pat != 0) then
    putval(1,1)
    -- always write pat / col2
		 	putval(crv.pat,16)
		 	putval(flr(crv.col/16),4)
  else
   putval(0,1)
  end

  lcol,lshape = col,shape
  
  -- 7 start x,y
  putval(crv[2],7)
  putval(crv[3],7)
  
  if (#crv < 4) then
   putval(0,1) -- no segs
  else
   putval(1,1) -- has segs

	  local a=0
	  local la=0
	   
	  -- segments
	  for i=4,#crv,2 do
	  
	   --printh("seg dest pos: "..dest.." bit "..bit)
	  
	   local dx=crv[i+0]-crv[i-2]
	   local dy=crv[i+1]-crv[i-1]
	   
	   -- categorize
	   
	   a=flr(atan2(dx,dy)*16+0.5)%16
	  
	   -- write delta
	   -- (waste a few bits for simplicity)
	   
	   local da=a-la
	   while (da>7)do da-=16 end
	   while (da<-8)do da+=16 end
	   
	   -- debug stats
	   if (debug_wob_encode) then
		   count_da[da]+=1
		   pset(draw_da%32,
		    60 + flr(draw_da/32)*2,
		    da+8) draw_da+=1
	   end
	   -- favor straight (1 bit)
	   -- tried 2-bit cluster at start, but little diff
	   
	   if (da==0) then
	    putval(1,1)
	   else
	    putval(0,1)
	    putval(da<0 and 1 or 0,1)
	    for i=1,abs(da)-1 do
	     putval(0,1)
	    end
	    putval(1,1)
	   end
	   
	   la=a
	  end
	  
	   -- end-of-segs marker
	   -- 1+1+8
    putval(0,10)

  end
  
  
 end

 local len=dest-dest0+1
 poke2(dest0,len)
	return len,len*8-(8-bit)

end
-->8
--preset data

-- size,shape,dotted,noise

bp_data={
[0]=
{0,4,0,0},
{1,4,0,0},
{3,0,0,0},
{7,0,0,0},

{0,1,6,0}, -- circ dot
{1,0,6,0}, -- fillcirc dot
{2,4,6,0}, -- dotted squares
{3,4,1,3}, -- squarejunk

{3,5,12,1},-- cute stars
{5,5,1,0}, -- tinsel
{3,1,3,2}, -- bubbles
{2,0,1,2}, -- splats

{2,2,0,0}, -- hline (lettering)
{2,3,0,0}, -- vline
{4,6,1,0}, -- track 
{3,1,4,0}, -- tidy chain
}

preset_pat={}
function read_pat(i,j)

 local sx=64+ flr(i/2)*8 +j*4
 local sy=8 + (i%2)   *4
 b=0x0
 for y=0,3 do
  for x=0,3 do
   b=shl(b,1)
   if (sget(sx+x,sy+y)==0) then
    b=bor(b,0x1)
   end
  end
 end
 
 return b
end

for i=0,11 do
 preset_pat[i]=read_pat(i,0)
end

-->8
-- wob_load / wob_draw 

--[[
 wob_draw
 sx,sy screen x,y (default:0,0)
 q wobble seed 
   (default use t() for 6fps)
]]
function wob_draw(scn,sx,sy,q)

 sx=sx or 0
 sy=sy or 0
 q=q or flr(time()*6)
 
 local funcs={[0]=
  circfill,circ,
  function(x,y,r,c)
   line(x-r,y,x+r,y,c) end,
	 function(x,y,r,c)
	  line(x,y-r,x,y+r,c) end,
  function(x,y,r,c)
   rectfill(x-r,y-r,x+r,y+r,c)
   end,
  function(x,y,r,c) -- star
		 a=nrnd(1)
		 for j=0,4 do
		  line(x,y,x+cos(a+j/5)*r,
		       y+sin(a+j/5)*r, c)
		 end
		end,
	 function(x,y,r,c,i) -- spin
		 local dx=cos(i*0x0.08)*r
		 local dy=sin(i*0x0.08)*r
		 line(x-dx,y-dy,x+dx,y+dy,c)
		end
 }
 
 local rv
 function nrnd(m)
  rv=rotl(rv,3)
  rv*=0x2518.493b -- mashed
  return (rv%m)
 end

 -- seed wobble by time
 srand(q)
 
 local xx=rnd(1)-.5
 local yy=rnd(1)-.5
 
 for j=1,#scn do
  local crv=scn[j]
  
  local r,col,x0,y0=
  	crv.size,crv.col,
	  sx+crv[2] + xx,
	  sy+crv[3] + yy
	
  local x1,y1=x0,y0
  
  local shape,dotted,noise,pat=
	  crv.shape,
	  crv.dotted/3,
	  crv.noise/3,
	  crv.pat
	 
  if(dotted>0)dotted=max(1,flr(dotted*(1+r)/2))
  
  -- set pattern
  if((col/0x11)%1==0)pat+=0.5  
  fillp(pat)
  
  local sfunc=funcs[shape]
  
  -- random generator for noise
  -- seeded by curve number j
  rv=0x37f9.2407*j
  
  for i=2,#crv-1,2 do
   
   x0=x1 y0=y1
   x1=sx+crv[i]   +xx
   y1=sy+crv[i+1] +yy
   
   -- jump to another rnd
   -- offset closeby 
   -- (prevents crinkles)
   xx=xx*7/8+(rnd(1)-.5)/2
   yy=yy*7/8+(rnd(1)-.5)/2
   
   if (dotted>0) then
   
    -- one every nth point
    
    if ((i-2)/2)%dotted==0 then
     
     if (noise==0) then
      sfunc(x1,y1,r,col,i/2)
     else
     
     local mag=(r+2)*noise*2
     local smag=(r+1)*noise
     local r0=r-r*noise
     
     sfunc(
      x1 + nrnd(mag) - mag/2,
      y1 + nrnd(mag) - mag/2,
      r0 + nrnd(smag),
      col,i/2)
     end
    end
   elseif (shape==2) then
    -- wide brush (lettering)
    for i=flr(-r/2),flr(r+.5)/2 do
		   line(x0+i,y0,x1+i,y1,col)
		  end
   elseif (shape==3) then
    -- tall brush
    for i=flr(-r/2),flr(r+.5)/2 do
		   line(x0,y0+i,x1,y1+i,col)
		  end
   elseif (r<2) then
		  -- common
		  line(x0,y0,x1,y1,col)
		  if (r==1) then 
		  line(x0+1,y0,x1+1,y1,col)
		  line(x0,y0+1,x1,y1+1,col)
		  line(x0+1,y0+1,x1+1,y1+1,col)
		  end
		 else
		  -- cheap hack:
		  -- draw at control point
		  -- and at midpoint.
		  sfunc(x0,y0,r-1,col)
		  sfunc((x0+x1)/2,(y0+y1)/2,
		   r-1,col)
		 end   
  end
 end
 
 fillp()
 
end


-- decode

function wob_load(src)

 local src0=src
 src-=1 
 local bit,b=256,0
 local scn={}
 
 local function getval(bits)
  
  local val=0
  for i=0,bits-1 do
   --get next bit from stream
   if (bit==256) then
    bit=1
    src+=1
    byte=peek(src)
   end
   if band(byte,bit)>0 then
    val+=2^i
   end
   bit*=2
  end
  return val
 end
 
 local dat_len = getval(16)
 local lsize,lcol
 
 -- back color
 scn.back_col=getval(4)
 
 -- read state
 local col,size,shape,dotted,
       noise,pat=
       0,0,0,0,0,0
 
 -- read until out of data
 -- each item is >= 3 bytes
 while (src<src0+dat_len-3) do
 
  -- curve header (3 sections)
  -- 1. 
  local crv=add(scn,{0})
  
 -- {0} --dummy

  if (getval(1)==1) then
   col,size=getval(4),getval(5)
  end
  
  if (getval(1)==1) then
   shape,dotted,noise=
   getval(3),getval(4),getval(3)
  end

  -- set state
  crv.col,crv.size,crv.shape,
  crv.dotted,crv.noise,crv.pat=
  col,size,shape,dotted,noise,0
  
  -- use pattern
  if (getval(1)>0) then
   crv.pat=getval(16)
   crv.col+=getval(4)*16
  end
  
  -- 7 start x,y
  add(crv,getval(7))
  add(crv,getval(7))
  local x0,y0,has_segs,a=
   crv[2],crv[3],getval(1),0
  
  -- read segments
  
  while (has_segs>0) do
   local v=0
   
   if (getval(1)<1) then
    -- read non-zero da
    local neg=getval(1)
    v=1
    while(getval(1)<1 and v<8)
    do v+=1 end
    if (neg>0) v*=-1
   end
   
   if (v==8) then
    -- end of segment
    has_segs=0
   else
    -- add segment
    a+=v
    x0+=flr(.5+cos(a/16)*3)
    y0+=flr(.5+sin(a/16)*3)
    add(crv,x0)
    add(crv,y0)
   end
  end
 end

 return scn
end


-- for loading from string
-- copied from clipboard
function str_to_mem(str,dest)
 for i=1,#str,2 do
  poke(dest,
   tonum("0x"..sub(str,i,i+1)))
  dest+=1
 end
end


__gfx__
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000077700000000000070000000000000000000000000000007070000070070000700700000000000000000000000000000000000
00000000000000000007770000777770000700000777000000000000000000770000700000777700707000007770000000000000000000000000007000770000
00000000000770000077777007777777000000000070000000770770007770770077777007777000070077000700770000000000000070000007007007007770
00007000000770000077777007777777000070000000700000770770007770000007770000077700000700700007777000077700000070000000700707007007
00000000000000000077777007777777000000000007770000000000007770000007070000777770000700700007777000000000000070000770070000777007
00000000000000000007770000777770000007000000700000000000000000000000000000700700000077000000770000000000000000000007000000000770
00000000000000000000000000077700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000028ccee7700000000000000007070777777000077070070770700000077777777070070770070000000000000
00000000000007000007000000000000000007000000000000700000007777707070000007700770777000070777000777777777000777700070000000000000
07070700077777700077000007007000000077000000000000770000007000707070777700777700070070777770700077777777070070770070707000707070
00000000070007700777770007007700000777000000000000777000007000707070000070077007000077770070000077777777000777707077777070777770
0070707007000770007700700077777000777700000000000077770000777770770077777000700000007777000077777070770007077070e7777770e7777770
00000000070007700007007000007700000777000000000000777000007000707700777707000007707007070770700707077700000077770e7777e00e7777e0
07070700077777700000000000007000000077000000000000770000007777007700000000700070777000070770700770700077070770700077770000777700
000000000000000000000000000000000000070000000000007000000000000077000000000707000700707700007777070700770000777700eeee0000eeee00
__label__
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000700000000000000000000000000000000000000000eeeeeee0000000000000000000000000000000000001111111100007770000
0000000000000000700000077000000700700000000000000000000000ccc0000eeeeeeeee000777000000000000000000000000000011117771110077777000
070707000000777777000077777000070077000000000002220000000ccccc000eeeeeeeee007777700000000000000000000000000011177777110777777700
000000000000700077000007700700007777700000000022222000000ccccc000eeeeeeeee007777700000000000000000000077000011177777110777777700
007070700000700077000000700700000077000000000022222000000ccccc000eeeeeeeee007777700000000000700000000077000011177777110777777700
0000000000007000770000000000000000700000000000222220000000ccc0000eeeeeeeee000777000000000000000000000000000011117771110077777000
070707000000777777000000000000000000000000000002220000000000000000eeeeeee0000000000000000000000000000000000001111111100007770000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111a11111111111111111111111111111111111111111111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111a11111111111111111111111111111111111111111111111111111111111111111111111a11111a11111111
11111111111111111111111111111111111111111a111a11111111111111111111111111111111111111111111111111111111111111111111a111a111111111
1111111111111111111111111111111111111aa11a1aa1111111111111111111111111111111111111111111111111111111111111111111111a11a111111111
111111111111111111111111111111111111111aaaa111111111111111111111111111111111111111111111111111111111111111111111111a1a1111111111
1111111111111111111111111111111111111111a1a111111111111111111111111111111111111111111111111111111111111111111111111aaa1111111111
1111111111111111111111111111111111111111a11a111111111111111111111111111111111111111111111111111111111111111111111aa1a1aa11111111
111111111111111111111111111111111111111a1111a111111111111111111111111111111111111111111111111111111111111111111aa111a111aa111111
11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111a11111111111
11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111a11111111111
11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111a11111111111
11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111
11111111111a11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111
11111111111a11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111
1111111a111a11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111
11111111aaa111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111
1111111111aaaaa11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111
11111111aa1a11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111
1111111a111a11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111
111111111111a1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111
111111111111a1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111
1111111111111111111111111111111111111111111111111111111111111111111111111111a1111a1111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111111111111111111111111111111111111111a11a11111111111111111111111111111111111111111111111
111111111111111111111111111111111111111111111111111111111111111111111111111111a1a11111111111111111111111111111111111111111111111
111111111111111111111111111111111111111111111ee11111111111111111111111111111111a111111111111111111111111111111111111111111111111
111111111111111111111111111111111111111111111eee11111111111111111111111111111aaaaa1111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111111eeee111111111111111111111111111aa11a11aa11111111111111111111111111111111111111111111
11111111111111111111111111111111111111111111eeee1111111111111111111111111111111a111111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111111eeee1111111111111111111111111111111a111111111111111111111111111111111111111111111111
1111111111111111111111111111111111111111111eeeee11111111111eee11111111111111111a111111111111111111111111111111111111111111111111
1111111111111111111111111111111111111111111ee1ee1111111111e111e11111111111111111111111111111111111111111111111111111111111111111
1111111111111111111111111111111111111111111ee1ee1111111111e111e11111111111111111111111111111111111111111111111111111111111111111
111111111111111111111111111111111111111111eee1ee1111111111e111e11111111111111111111111111111111111111111111111111111111111111111
111111111111111111111111111111111111111111ee11ee11111111111eee111111111111111111111111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111eee11ee11111111111111111111111111111111111111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111ee111ee11111111111111111111111111111111111111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111ee111ee11111111111111111111111111111111111111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111ee111ee11111111111111111111111111111111111111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111ee111ee11111111111111111111111111111111111111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111ee111ee11111111111111111111111111111111111111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111ee111ee11111111111111111111111111111111111111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111ee11eee11111111111111111111111111111111111111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111ee11ee111111111111111111111111111111111111111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111eee1ee111111111111111111111111111111111111111111111111111111111111111111111111111111111
111111111111111111111111111111111111111111ee1eee11111111111111111111111111111111111111111111111111111111111111111111111111111111
111111111111111111111111111111111111111111ee11ee111111111111111111111111111111111111111111111111111ee111111111111111111111111111
111111111111111111111111111111111111111111eee1eee1111111111111111111111111111111111111111111111111eeee11111111111111111111111111
1111111111111111111111111111111111111111111ee11ee111111111111111111111111111111111111111111111111eeeeee1111111111111111111111111
1111111111111111111111111111111111111111111eee1eee111111111111111111111eee11111111111111111111111ee11eeee11111111111111111111111
11111111111111111111111111111111111111111111eee1eee11111111111111111111eeeee111111111111111111111ee111eee11111111111111111111111
111111111111111111111111111111111111111111111eee1eee111111111111111111eeeeeee1111111111111111111eee1111eee1111111111111111111111
1111111111111111111111111111111111111111111111eee1eeee1111111111111111ee111ee111111111111111111eee111111ee1111111111111111111111
11111111111111111111111111111111111111111111111ee11eee1111111111111111ee111ee11111111111111111eee1111111ee1111111111111111111111
11111111111111111111111111111111111111111111111eee11ee1111111111111111ee111eee1111111111111111ee11111111eee111111111111111111111
111111111111111111111111111111111111111111111111ee11ee1111111111111111ee1111ee111111111111111eee111111111ee111111111111111111111
111111111111111111111111111111111111111111111111ee11eeee1111111111111eee1111ee111111111111111ee1111111111ee111111111111111111111
111111111111111111111111111111111111111111111111eee11eeeee1111111111eee11111eee1111111111111eee1111111111eee11111111111111111111
1111111111111111111111111111111111111111111111111ee1111eeeeee111111eee1111111eeeeeeeeeeeeeeeee111111111111ee11111111111111111111
1111111111111111111111111111111111111111111111111eee11111eeeeeeeeeeeee1111111eeeeeeeeeeeeeeeee111111111111ee11111111111111111111
11111111111111111111111111111111111111111111111111ee11111111eeeeeeeee1111111111111111111111111111111111111eee1111111111111111111
11111111111111111111111111111111111111111111111111eee1111111111111eee11111111111111111111111111111111111111ee1111111111111111111
11111111111111111111111111111111111111111111111eeeeee11111111111111ee11111111111111111111111111111111111111ee1111111111111111111
11111111111111111111111111111111111111111111111eeee1111111111111111ee11111111111111111111111111111111111111ee1111111111111111111
1111111111111111111111111111111111111111111111eee111111111111111111ee1111111111111111111111111111111ee11111ee1111111111111111111
1111111111111111111111111111111111111111111111ee1111111111111111111ee111111111ee11111111111111111111ee11111ee1111111111111111111
11111111111111111111111b1111111111111111111111ee1111111111111111111eee11111111ee11111111111111111111ee11111ee1111111111111111111
111111111111111111111b111b11111111111111111111ee11111111111111111111ee11111111ee11111111111111111111ee11111ee1111111111111111111
1111111111111111111b111b111b111111111111111111ee11111111111111111111eee1111111ee11111111111111111111ee11111ee1111111111111111111
111111111111111111111b111b111b1111111111111111ee111111111111111111111eee111111ee11111111111111111111111111eee1111111111111111111
1111111111111111111b111b111b111111111111111111ee1111111111111111111111ee1111111111111111111111111111111111ee11111111111111111111
11111111111111111b111b111b111b1111111111111111ee1111111eee111111111111eee111111111111111111111111111111111ee11111111111111111111
1111111111111111111b111b111b111111111111111111ee1111111eeeee11111111111ee11111111111111111111111111111111eee11111111111111111111
11111111111111111b111b111b111b1111111111111111ee111111eeeeee11111111111eee1111111111111111111111111111111ee111111111111111111111
111111111111111b111b111b111b111111111111111111ee111111ee11eee11111111111eeee11111111111111111111111111eeeee111111111111111111111
1111111111111b111b111b111b1111111111111111111bee111111ee111ee111111111111eeeeee11111111111111111111eeeeeee1111111111111111111111
111111111111111b111b111b1111111111111111111b11eb11111eee111eeee111111111111eeeeeeeeeeeeeeeeeeeeeeeeeeee1111111111111111111111111
1111111111111b111b111b111b11111111111b111b111bee1b111ee11111eeee11111111111111eeeeeeeeeeeeeeeeeeeeeee111111111111111111111111111
111111111111111b111b111b1111111b111b111b111b11eb111b1ee1111111eee111111111111111111111111111111111ee1111111111111111111111111111
1111111111111b111b111b1111111b111b111b111b111beeeb111ee11111111eeee1111111111111111111111111111111ee1111111111111111111111111111
11111111111b111b111b1111111b111b111b111b111b111bee1beee111111111eeee111111111111111111111111111111ee1111111111111111111111111111
1111111111111b111b111b111b111b111b111b111b111b1eebeeebe11b11111111ee111111111111111111111111111111ee1111111111111111111111111111
11111111111b111b111b111b111b111b111b111b111b111b1eebe11b111b111111eee11111111111111111111111111111ee1111111111111111111111111111
111111111b111b111b111b111b111b111b111b111b111b111b111b111b111111111ee11111111111111111111111111111ee1111111111111111111111111111
11111111111b111b111b111b111b111b111b111b111b111b111b111b111b1111111ee1111111111111111eeeeee1111111ee1111111111111111111111111111
111111111b111b111b111b111b111b111b111b111b111b111b111b111b111b1111eee11111eeeeeeeeeeeeeeeee111111bee1b11111111111111111111111111
1111111b111b111b111b111b111b111b111b111b111b111b111b111b111b111111ee111b11eeeeeeeeeeee111eee111b11eb11111111111b1111111111111111
111111111b111b111b111b111b111b111b111b111b111b111b111b111b111b1111ee1b111bee11111111111111ee1b111bee1b1111111b111b11111111111111
1111111b111b111b111b111b111b111b111b111b111b111b111b111b111b111111ebe11b1eeb11111111111111eb111b11eb111b111b111b111b111111111111
111111111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b1eeb111be11b1111111b111beeeb111bee1b111b111b111b111b1111111111
1111111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111be11b1eeb11111111111b111bee1b1eeb111b111b111b111b111111111111
111111111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b1eeb111be11b1111111b111b1eebeeebe11b111b111b111b111b1111111111
1111111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111beeebeeeb1111111b111b111b1eebeeeb111b111b111b111b111111111111
111111111b111b111b111b1111111b111b111b111b111b111b111b111b111b111b11ebeeeb111b111b111b111b111b111b111b111b111b111b111b1111111b11
11111111111b111b11111111111b111b111b111b111b111b111b111b111b111b111b11eb111b111b111b111b111b111b111b111b111b111b111b111b111b111b
111111111b111b11111111111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b11
11111111111111111111111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b
1111111111111111111111111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b11
11111111111111111111111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b
111111111111111111111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b11
1111111111111111111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b
111111111111111111111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b11
1111111111111111111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b111b
111111111111111111111b111b111b111b1111111b111b111b111b11111111111b111b111b111b111b111b111b111b11111111111b111b111b111b111b111b11
1111111111111111111b111b111b111b111b111b111b111b111b11111111111b111b111b111b111b111b111b111b11111111111b111b111b111b111b111b111b
111111111111111111111b111b111b11111111111b111b111b111111111111111b111b111b111b11111111111b111111111111111b111b111b111b111b111b11
11111111111111111111111b111b111111111111111b111b1111111111111111111b111b111b1111111111111111111111111111111b111b1111111b111b1111
1111111111111111111111111b111111111111111111111111111111111111111b111b111b111111111111111111111111111111111111111111111111111111
1111111111111111111111111111111111111111111111111111111111111111111b111111111111111111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111

__sfx__
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
__music__
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000
00 00000000

