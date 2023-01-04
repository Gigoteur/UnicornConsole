#[cfg(feature = "rlua")]
pub mod plugin {
    use log::{error, info, debug};

    use anyhow::{Result, anyhow};

    use std::sync::{Arc, Mutex};

    use rand;
    use rand::Rng;
    use rand::prelude::*;
    use rand_chacha::ChaCha8Rng;

    use rlua::{Lua, UserData, UserDataMethods};
    use rlua::prelude::LuaError;

    use contexts::Contexts;

    use core::info::Info;

    use gfx::Screen;

    use crate::contexts;
    
    pub struct ExtraData {
        /* External objects to get access to Unicorn data ! */
        pub contexts: Arc<Mutex<Contexts>>,
        pub info: Arc<Mutex<Info>>,
        pub screen: Arc<Mutex<Screen>>,
    }

    impl UserData for ExtraData {
        fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
/*
        # Input                 #               #               #
        btn                     #     X         #               #
        btnp                    #     X         #               #
        mouse_x                 #               #               #
        mouse_y                 #               #               #
        mouse_state             #               #               #
        mouse_statep            #               #               #
        # Palette               #               #               #
        palette                 #               #               #
        palette_hexa            #               #               #
        palette_reset           #               #               #
        palette_switch          #               #               #
        # Math                  #               #               #
        atan2                   #     X         #               #
        cos                     #     X         #               #
        sin                     #     X         #               #
        flr                     #     X         #               #
        rnd                     #     X         #               #
        srand                   #     X         #               #
        mid                     #     X         #               #
        bxor                    #     X         #               #
        # Memory                #               #               #
        memcpy                  #               #               #
        # System                #               #               #
        time                    #     X         #               #
        time_sec                #               #               #
        show_mouse              #               #               #
*/    

            methods.add_method("btn", |_lua_ctx, game_state, (i, player):(u8, u8)| {
               let value = game_state.contexts.lock().unwrap().input_context.btn(player, i);
               Ok(value)
            });

            methods.add_method("btnp", |_lua_ctx, game_state, (i, player):(u8, u8)| {
                let value = game_state.contexts.lock().unwrap().input_context.btnp(player, i);
                Ok(value)
             });

             methods.add_method("rnd", |_lua_ctx, game_state, x:u32| {
                let value: f64;

                if x == 0 {
                    value = rand::thread_rng().gen_range(0.0..1.0);
                } else {
                    value = rand::thread_rng().gen_range(0.0.. x as f64);
                }

                Ok(value)
            });

            methods.add_method("srand", |_lua_ctx, game_state, x:u8| {
                ChaCha8Rng::seed_from_u64(x.into());
                Ok(1)
            });


/* 
        # Map                   #               #               #
        mapdraw                 #     X         #               #
        mget                    #     X         #               #
        mset                    #     X         #               #
*/
            methods.add_method("mapdraw", |_lua_ctx, game_state, (cel_x, cel_y, sx, sy, cel_w, cel_h, layer):(u32, u32, i32, i32, u32, u32, u8)| {
                game_state.screen.lock().unwrap().mapdraw(cel_x, cel_y, sx, sy, cel_w, cel_h, layer);
                Ok(0)
            });

            methods.add_method("mget", |_lua_ctx, game_state, (x, y):(i32, i32)| {
                let value = game_state.screen.lock().unwrap().mget(x, y);
                Ok(value)
            });

            methods.add_method("mset", |_lua_ctx, game_state, (x, y, v):(i32, i32, u32)| {
                game_state.screen.lock().unwrap().mset(x, y, v);
                Ok(0)
            });

/*
        # GFX                   #    Lua        #    New name (if conflicted with keywords language)   #
        mode_width              #     X         #               #
        mode_height             #     X         #               #
        camera                  #     X         #               #
        circ                    #     X         #               #
        circfill                #     X         #               #
        clip                    #     X         #               #
        cls                     #     X         #               #
        color                   #     X         #               #
        ellipse                 #     X         #               #
        ellipsefill             #     X         #               #
        fget                    #     X         #               #
        fset                    #     X         #               #
        font                    #     X         #               #
        line                    #     X         #               #
        pal                     #     X         #               #
        palt                    #     X         #               #
        pget                    #     X         #               #
        polygon                 #               #               #
        print                   #     X         #               #
        pset                    #     X         #               #
        rect                    #     X         #               #
        rectfill                #     X         #               #
        sget                    #     X         #               #
        spr                     #     X         #               #
        sset                    #     X         #               #
        sspr                    #               #               #
        sspr_rotazoom           #               #               #
        trigon                  #     X         #               #
*/
            methods.add_method("mode_width", |_lua_ctx, game_state, ()| {
                Ok(game_state.screen
                    .lock()
                    .unwrap()
                    .mode_width())
            });

            methods.add_method("mode_height", |_lua_ctx, game_state, ()| {
                Ok(game_state.screen
                    .lock()
                    .unwrap()
                    .mode_height())
            });

            methods.add_method("camera", |_lua_ctx, game_state, (x, y):(i32, i32)| {
                game_state.screen
               .lock()
               .unwrap()
               .camera(x, y);
               
               Ok(())
            });

            methods.add_method("circ", |_lua_ctx, game_state, (x, y, r, col):(i32, i32, i32, i32)| {
                game_state.screen
               .lock()
               .unwrap()
               .circ(x, y, r, col);
               
               Ok(())
            });

            methods.add_method("circfill", |_lua_ctx, game_state, (x, y, r, col):(i32, i32, i32, i32)| {
                game_state.screen
               .lock()
               .unwrap()
               .circfill(x, y, r, col);
               
               Ok(())
            });


            methods.add_method("clip", |_lua_ctx, game_state, (x, y, w, h):(i32, i32, i32, i32)| {
                game_state.screen
               .lock()
               .unwrap()
               .clip(x, y, w, h);
               
               Ok(())
            });

            methods.add_method("cls", |_lua_ctx, game_state, col:i8| {
                game_state.screen
               .lock()
               .unwrap()
               .cls(col);
               
               Ok(())
            });


            methods.add_method("color", |_lua_ctx, game_state, col:i32| {
                game_state.screen
               .lock()
               .unwrap()
               .color(col);
               
               Ok(())
            });

            methods.add_method("ellipse", |_lua_ctx, game_state, (x, y, rx, ry, col):(i32, i32, i32, i32, i32)| {
                game_state.screen
               .lock()
               .unwrap()
               .ellipse(x, y, rx, ry, col);
               
               Ok(())
            });

            methods.add_method("ellipsefill", |_lua_ctx, game_state, (x, y, rx, ry, col):(i32, i32, i32, i32, i32)| {
                game_state.screen
               .lock()
               .unwrap()
               .ellipsefill(x, y, rx, ry, col);
               
               Ok(())
            });
    
            methods.add_method("fget", |_lua_ctx, game_state, (idx, flag):(u32, u8)| {
                let value = game_state.screen
               .lock()
               .unwrap()
               .fget(idx, flag);
               
               Ok(value)
            });

            methods.add_method("fget_all", |_lua_ctx, game_state, (idx, flag):(u32, u8)| {
                let value = game_state.screen
               .lock()
               .unwrap()
               .fget_all(idx);
               
               Ok(value)
            });

            methods.add_method("fset", |_lua_ctx, game_state, (idx, flag, value):(u32, u8, bool)| {
                game_state.screen
               .lock()
               .unwrap()
               .fset(idx, flag, value);
               
               Ok(())
            });

            methods.add_method("fset_all", |_lua_ctx, game_state, (idx, flag):(u32, u8)| {
                game_state.screen
               .lock()
               .unwrap()
               .fset_all(idx, flag);
               
               Ok(())
            });

            methods.add_method("font", |_lua_ctx, game_state, name:String| {
                game_state.screen
               .lock()
               .unwrap()
               .font(&name);
               
               Ok(())
            });

            methods.add_method("line", |_lua_ctx, game_state, (x0, y0, x1, y1, col):(i32, i32, i32, i32, i32)| {
                game_state.screen
               .lock()
               .unwrap()
               .line(x0, y0, x1, y1, col);
               
               Ok(())
            });

            methods.add_method("pal", |_lua_ctx, game_state, (c0, c1, pal_idx):(i32, i32, i32)| {
                game_state.screen
               .lock()
               .unwrap()
               .pal(c0, c1);
               
               Ok(())
            });

            methods.add_method("palt", |_lua_ctx, game_state, (c, t):(i32, bool)| {
                game_state.screen
               .lock()
               .unwrap()
               .palt(c, t);
               
               Ok(())
            });

            methods.add_method("pset", |_lua_ctx, game_state, (x, y, col):(i32, i32, i32)| {
                game_state.screen
               .lock()
               .unwrap()
               .pset(x, y, col);
               
               Ok(())
            });

            methods.add_method("pget", |_lua_ctx, game_state, (x, y):(u32, u32)| {
                let value = game_state.screen
               .lock()
               .unwrap()
               .pget(x, y);
               
               Ok(value)
            });

            methods.add_method("print", |_lua_ctx, game_state, (str_data, x, y, col):(String, i32, i32, i32)| {
                game_state.screen
               .lock()
               .unwrap()
               .print(str_data, x, y, col);
               
               Ok(())
            });
    
            methods.add_method("rect", |_lua_ctx, game_state, (x0, y0, x1, y1, col):(i32, i32, i32, i32, i32)| {
                game_state.screen
               .lock()
               .unwrap()
               .rect(x0, y0, x1, y1, col);
               
               Ok(())
            });

            methods.add_method("rectfill", |_lua_ctx, game_state, (x0, y0, x1, y1, col):(i32, i32, i32, i32, i32)| {
                game_state.screen
               .lock()
               .unwrap()
               .rectfill(x0, y0, x1, y1, col);
               
               Ok(())
            });

            methods.add_method("sget", |_lua_ctx, game_state, (x, y):(i32, i32)| {
                let value = game_state.screen
               .lock()
               .unwrap()
               .sget(x, y);
               
               Ok(value)
            });

            methods.add_method("spr", |_lua_ctx, game_state, (n, x, y, w, h, flip_x, flip_y, angle, zoom, dynamic):(u32, i32, i32, i32, i32, bool, bool, f64, f64, bool)| {
                game_state.screen
               .lock()
               .unwrap()
               .spr(n, x, y, w, h, flip_x, flip_y, angle, zoom, dynamic);

               Ok(())
            });

            methods.add_method("sset", |_lua_ctx, game_state, (x, y, col):(u32 , u32, i32)| {
                game_state.screen
               .lock()
               .unwrap()
               .sset(x, y, col);
               
               Ok(())
            });
        }
    }

    pub struct LuaPlugin {
        lua: Lua,
        loaded_code: bool,
        info: Vec<Arc<Mutex<Info>>>,
    }

    impl LuaPlugin {
        pub fn new() -> LuaPlugin {
            LuaPlugin {
                lua: Lua::new(),
                loaded_code: false,
                info: Vec::new(),
            }
        }

        pub fn load(&mut self,
                    contexts: Arc<Mutex<Contexts>>,
                    info: Arc<Mutex<Info>>,
                    screen: Arc<Mutex<Screen>>) {
            info!("[PLUGIN][LUA] Init plugin");
            
            self._load_pico8_functions();
            self._load_utilities_functions();

            self.lua.context(|lua| {
                let globals = lua.globals();
                let userdata = lua.create_userdata(ExtraData{
                    contexts:contexts.clone(), 
                    info:info.clone(),
                    screen:screen.clone()}).unwrap();
                
                globals.set("userdata", userdata.clone()).unwrap();

                lua.load(
                    r#"
                        function rnd(x)
                          if x == nil then
                            x = 1
                          end
                          x = math.floor(x)
                          return userdata:rnd(x)
                        end

                        function srand(x)
                            if x == nil then
                                x = 1
                            end
                            x = math.floor(x)
                            return userdata:srand(x)
                        end

                        function btn(i, p)
                            if p == nil then
                                p = 0
                            end
                            return userdata:btn(i, p)
                        end
                        
                        function btnp(i, p)
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
                    "#,
                )
                .exec()
                .unwrap();
    
            });
        }

        fn _load_pico8_functions(&mut self) {
            info!("[PLUGIN][LUA] Load Pico8 functions");

            self.lua.context(|lua| {
                lua.load(
                    r#"
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
                "#,
            )
            .exec()
            .unwrap();
            });
        }

        fn _load_utilities_functions(&mut self) {
            info!("[PLUGIN][LUA] Load Utilities functions");

            self.lua.context(|lua| {
                lua.load(
                    r#"
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
                atan2 = function(y,x) return __pico_angle(math.atan2(y,x)) end
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
                "#,
            )
            .exec()
            .unwrap();
            });
        }


        pub fn init(&mut self) -> Result<()> {
            info!("[PLUGIN] LUA INIT");
            
            if !self.loaded_code {
               return Err(anyhow!("[PLUGIN][LUA] [init] impossible to load the code"))
            }

          let _res = match self.lua.context(|lua_ctx| {
                lua_ctx
                .load(
                    r#"
                    _init()
                    "#,
                )
                .set_name("call init method")?
                .exec()
            }) {
                Ok(_) => return Ok(()),
                Err(err) => return Err(anyhow!("[PLUGIN][LUA] [init]: {}", err)),
            };
        }

        pub fn draw(&mut self) -> Result<()> {
            if self.loaded_code {
                let _res = match self.lua.context(|lua_ctx| {
                    lua_ctx.load(
                        r#"
                            _draw()
                        "#,
                    )
                    .set_name("call draw method")?
                    .exec()}) {
                            Ok(_) => return Ok(()),
                            Err(err) => return Err(anyhow!("[PLUGIN][LUA] [draw]: {}", err)),
                    };
            }
            Err(anyhow!("[PLUGIN][LUA] [draw]: code is not loaded !"))
        }

        pub fn update(&mut self) -> Result<()> {
            if self.loaded_code {
                let _res = match self.lua.context(|lua_ctx| {
                    lua_ctx.load(
                        r#"
                            _update()
                        "#,
                    )
                    .set_name("call update method")?
                    .exec()}) {
                            Ok(_) => return Ok(()),
                            Err(err) => return Err(anyhow!("[PLUGIN][LUA] [update]: {}", err)),
                    };
            }
            Err(anyhow!("[PLUGIN][LUA] [draw]: code is not loaded !"))
        }

        pub fn load_code(&mut self, data: String) -> bool {
            info!("[PLUGIN][LUA] [load_code] {:?}", data.len());

            debug!("[PLUGIN][LUA] [load_code] {:?}", data);

            let _res = match self.lua.context(|lua_ctx| {
                        lua_ctx.load(&data).exec()}) {
                            Ok(_) => self.loaded_code = true,
                            Err(err) => {
                                error!("{:?}", err);
                                self.loaded_code = false;
                            }

            };

            self.loaded_code
        }
    }

    

    
}


#[cfg(not(feature = "rlua"))]
pub mod plugin {
    use log::{error};

    use std::sync::{Arc, Mutex};
    use anyhow::{Result, anyhow};

    use contexts::Contexts;

    use core::info::Info;

    use gfx::Screen;

    #[derive(Debug)]
    pub struct LuaPlugin {}

    impl LuaPlugin {
        pub fn new() -> LuaPlugin {
            LuaPlugin {}
        }

        // Keep the compatibility
        pub fn load(&mut self,
                    _contexts: Arc<Mutex<Contexts>>,
                    _info: Arc<Mutex<Info>>,
                    _screen: Arc<Mutex<Screen>>) {
            error!("LUA plugin disabled");
        }
        pub fn load_code(&mut self, _data: String) -> bool {
            false
        }
        pub fn init(&mut self) -> Result<()> {
            Err(anyhow!("[PLUGIN][LUA] [init] lua is not compiled"))
        }
        pub fn draw(&mut self) -> Result<()> {
            Err(anyhow!("[PLUGIN][LUA] [draw] pytluahon is not compiled"))
        }
        pub fn update(&mut self) -> Result<()> {
            Err(anyhow!("[PLUGIN][LUA] [update] lua is not compiled"))
        }
    }
}
