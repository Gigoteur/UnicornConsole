--voxel framework
--pico-8 doodle by trasevol_dog

--this was the start of my
--"your personal archipel"
--project for procjam!
--it's got 3d island generation
--check it out there:
-- https://trasevol-dog.itch.io/personal-archipel

--rotate with 
--take a look at the spritesheet
--and modify it


cama1=0
va1=0

t=0
function _update()
 
 if btn(0) then va1-=0.01 end
 if btn(1) then va1+=0.01 end

 cama1+=va1
 
 va1*=0.9

end


function _draw()
 cls()
 
 local ocx=cos(cama1)
 local osx=-sin(cama1)
 local ocy=cos(cama1+0.25)
 local osy=-sin(cama1+0.25)
 
 if cama1%1>0.5 then
  ssx=15
  sox=-1
 else
  ssx=0
  sox=1
 end
   
 if cama1%1>0.25 and cama1%1<0.75 then
  ssy=15
  soy=-1
 else
  ssy=0
  soy=1
 end

 for l=0,19 do
  local ly=96-l*4
  local lx=64
  
  local x,y=lx,ly
  
  local sx=ssx
  for ix=0,15 do  
  local sy=ssy
  for iy=0,15 do

   local c=sget(sx+(l%8)*16,sy+flr(l/8)*16)
   
   if c~=0 then
   
    --local a=atan2(sx-8,sy-8)-cama1
    --local l=sqrt(sqr(sx-8)+sqr(sy-8))
    local xx=3.99*((sx-7.5)*ocx+(sy-7.5)*ocy)
    local yy=1.2*((sx-7.5)*osx+(sy-7.5)*osy)
   
    xx+=x
    yy+=y
   
    rectfill(xx-2,yy-2,xx+1,yy+1,c)
   
   end
  
  sy+=soy
  end
  sx+=sox
  end
  
 end
 
-- print(stat(0),0,0,7)
-- print(stat(1),0,8,7)
-- print(cama1,0,16,6)
end



function sqr(a) return a*a end
