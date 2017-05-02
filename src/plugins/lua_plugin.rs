#[cfg(feature = "lua")]
pub mod plugin {
    use std::sync::{Arc, Mutex};

    use rand;
    use rand::Rng;

    use lua;
    use lua::ffi::lua_State;
    use lua::{State, Function, ThreadStatus};
    use libc::c_int;

    use config::Players;

    use px8::info::Info;

    use gfx::{SCREEN_WIDTH, SCREEN_HEIGHT};
    use gfx::Screen;

    pub struct ExtraData {
        /* External objects */
        pub players: Arc<Mutex<Players>>,
        pub screen: Arc<Mutex<Screen>>,
        pub info: Arc<Mutex<Info>>,
    }

    #[derive(Clone, Debug)]
    pub enum LuaPluginError {
        ThreadStatus(String),
        Other(String),
    }

    pub struct LuaPlugin {
        lua_state: Arc<Mutex<lua::State>>,
        loaded_code: bool,
    }

    impl LuaPlugin {
        pub fn new() -> LuaPlugin {
            LuaPlugin { lua_state: Arc::new(Mutex::new(lua::State::new())),
                        loaded_code: false
            }
        }

        #[allow(unused)]
        pub fn load(&mut self,
                    players: Arc<Mutex<Players>>,
                    info: Arc<Mutex<Info>>,
                    screen: Arc<Mutex<Screen>>) {
            info!("[PLUGIN][LUA] Init plugin");

            let extra = ExtraData {
              players: players.clone(),
              info: info.clone(),

              screen: screen.clone(),
            };

            let mut lua_state = self.lua_state.lock().unwrap();
            lua_state.open_libs();
            lua_state.set_extra(Some(Box::new(extra)));

            lua_state.new_table();
            lua_state.set_fns(&PX8LUA_LIB, 0);

            lua_state.push_value(-1);
            lua_state.set_global("PX8Lua");

            lua_state.new_metatable("PX8Lua");

            lua_state.push_value(-2);
            lua_state.set_field(-2, "__index");

            lua_state.pop(2);

            /* Create the PX8Lua object */
            lua_state.do_string("s = PX8Lua.new()");

            lua_state.do_string(r#"debug_print = print"#);

            lua_state.do_string(r#"camera = function(x, y)

              x = math.floor(x)
              y = math.floor(y)

              s:camera(x, y)
              end
              "#);

            lua_state.do_string(r#"btn = function(x, p)

              x = math.floor(x)

              if p == nil then
                p = 0
              end

              return s:btn(p, x) == 1
              end
              "#);
            lua_state.do_string(r#"btnp = function(x, p)

              x = math.floor(x)

              if p == nil then
                p = 0
              end

              return s:btnp(p, x) == 1
              end
              "#);


            lua_state.do_string(r#"rect = function(x0, y0, x1, y1, color)

              x0 = math.floor(x0)
              y0 = math.floor(y0)
              x1 = math.floor(x1)
              y1 = math.floor(y1)

              if color == nil then
                color = -1
              end

              color = math.floor(color)

              s:rect(x0, y0, x1, y1, color)
              end
              "#);
            lua_state.do_string(r#"rectfill = function(x0, y0, x1, y1, color)

              x0 = math.floor(x0)
              y0 = math.floor(y0)
              x1 = math.floor(x1)
              y1 = math.floor(y1)

              if color == nil then
                color = -1
              end

              color = math.floor(color)

              s:rectfill(x0, y0, x1, y1, color)
              end
              "#);
            lua_state.do_string(r#"circ = function(x, y, r, color)
              x = math.floor(x)
              y = math.floor(y)
              r = math.floor(r)

              if color == nil then
                color = -1
              end

              color = math.floor(color)

              s:circ(x, y, r, color)
              end
              "#);
            lua_state.do_string(r#"circfill = function(x, y, r, color)
              x = math.floor(x)
              y = math.floor(y)
              r = math.floor(r)

              if color == nil then
                color = -1
              end

              color = math.floor(color)

              s:circfill(x, y, r, color)
              end
              "#);
            lua_state.do_string(r#"clip = function(x, y, w, h)
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

              x = math.floor(x)
              y = math.floor(y)
              w = math.floor(w)
              h = math.floor(h)

              s:clip(x, y, w, h)
              end
              "#);
            lua_state.do_string(r#"ellipse = function(x, y, rx, ry, color)
              x = math.floor(x)
              y = math.floor(y)
              rx = math.floor(rx)
              ry = math.floor(ry)

              if color == nil then
                color = -1
              end

              color = math.floor(color)

              s:ellipse(x, y, rx, ry, color)
              end
              "#);
            lua_state.do_string(r#"ellipsefill = function(x, y, rx, ry, color)
              x = math.floor(x)
              y = math.floor(y)
              rx = math.floor(rx)
              ry = math.floor(ry)

              if color == nil then
                color = -1
              end

              color = math.floor(color)

              s:ellipsefill(x, y, rx, ry, color)
              end
              "#);
            lua_state.do_string(r#"fget = function(idx, flag)
              idx = math.floor(idx)
              flag = math.floor(flag)

              if flag == nil then
                return s:fget_all(idx)
              end

              return s:fget(idx, flag)

              end
              "#);
            lua_state.do_string(r#"fset = function(idx, flag, value)
              idx = math.floor(idx)
              flag = math.floor(flag)

              if value == nil then
                s:fset_all(idx, flag)
              else
                if value == true then
                    s:fset(idx, flag, 1)
                else
                    s:fset(idx, flag, 0)
                end
              end

              end
              "#);
            lua_state.do_string(r#"line = function(x0, y0, x1, y1, color)

              x0 = math.floor(x0)
              y0 = math.floor(y0)
              x1 = math.floor(x1)
              y1 = math.floor(y1)

              if color == nil then
                color = -1
              end

              color = math.floor(color)

              s:line(x0, y0, x1, y1, color)
              end
              "#);
            lua_state.do_string(r#"trigon = function(x1, y1, x2, y2, x3, y3, color)
              x1 = math.floor(x1)
              y1 = math.floor(y1)
              x2 = math.floor(x2)
              y2 = math.floor(y2)
              x3 = math.floor(x3)
              y3 = math.floor(y3)

              if color == nil then
                color = -1
              end

              color = math.floor(color)

              s:trigon(x1, y1, x2, y2, x3, y3, color)
              end
              "#);

            lua_state.do_string(r#"rnd = function(x)
              x = math.floor(x)
              return s:rnd(x)
              end
              "#);

            lua_state.do_string(r#"add = function(t, v)
              t[#t+1] = v
              end
              "#);

            lua_state.do_string(r#"cls = function()
              s:cls()
              end
              "#);

            lua_state.do_string(r#"palt = function(c, t)
              c = math.floor(c)

              if t == true then
                t = 1
              else
                t = 0
              end

              s:palt(c, t)
              end
              "#);

            lua_state.do_string(r#"pal = function(c0, c1, p)
              if c0 == nil then
                c0 = -1
              end

              if c1 == nil then
                c1 = -1
              end

              if p == nil then
                p = -1
              end

              s:pal(c0, c1, p)
              end
              "#);

            lua_state.do_string(r#"pset = function(x, y, color)
              x = math.floor(x)
              y = math.floor(y)

              if color == nil then
                color = -1
              end

              color = math.floor(color)

              s:pset(x, y, color)

              end
              "#);

            lua_state.do_string(r#"pget = function(x, y)
              x = math.floor(x)
              y = math.floor(y)

              return s:pget(x, y)
              end
              "#);

            lua_state.do_string(r#"sget = function(x, y)
              x = math.floor(x)
              y = math.floor(y)

              return s:sget(x, y)
              end
              "#);


            lua_state.do_string(r#"sset = function(x, y, color)
              x = math.floor(x)
              y = math.floor(y)

              if color == nil then
                color = -1
              end

              color = math.floor(color)

              s:sset(x, y, c)
              end
              "#);

            lua_state.do_string(r#"map = function(cel_x, cel_y, sx, sy, cel_w, cel_h, layer)

              cel_x = math.floor(cel_x)
              cel_y = math.floor(cel_y)
              sx = math.floor(sx)
              sy = math.floor(sy)
              cel_w = math.floor(cel_w)
              cel_h = math.floor(cel_h)


              if layer == nil then
                layer = 0
              end

              s:map(cel_x, cel_y, sx, sy, cel_w, cel_h, layer)
              end
              "#);

            lua_state.do_string(r#"mapdraw = function(cel_x, cel_y, sx, sy, cel_w, cel_h, layer)
                map(cel_x, cel_y, sx, sy, cel_w, cel_h, layer)
                end
                "#);

            lua_state.do_string(r#"mget = function(x, y)
              x = math.floor(x)
              y = math.floor(y)

              return s:mget(x, y)
              end
              "#);

            lua_state.do_string(r#"mset = function(x, y, v)
              x = math.floor(x)
              y = math.floor(y)
              v = math.floor(v)

              s:mset(x, y, v)
              end
              "#);

            lua_state.do_string(r#"spr = function(n, x, y, w, h, flip_x, flip_y)

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

              if flip_x == true then
                flip_x = 1
              else
                flip_x = 0
              end

              if flip_y == true then
                flip_y = 1
              else
                flip_y = 0
              end

              s:spr(n, x, y, w, h, flip_x, flip_y)
              end
              "#);

            lua_state.do_string(r#"sspr = function(sx, sy, sw, sh, dx, dy, dw, dh, flip_x, flip_y)
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

              if flip_x == true then
                flip_x = 1
              else
                flip_x = 0
              end

              if flip_y == true then
                flip_y = 1
              else
                flip_y = 0
              end

              s:sspr(sx, sy, sw, sh, dx, dy, dw, dh, flip_x, flip_y)
              end
              "#);

           lua_state.do_string(r#"print = function(str, x, y, col)

              if x == nil then
                x = -1
              end

              if y == nil then
                y = -1
              end

              if col == nil then
                col = -1
              end

              x = math.floor(x)
              y = math.floor(y)
              col = math.floor(col)

              s:print(str, x, y, col)

              end
              "#);

            lua_state.do_string(r#"time = function()
                v  = s:time()
                return v
              end
              "#);

            lua_state.do_string(r#"sfx = function(n, channel, offset)
              end
              "#);

            lua_state.do_string(r#"music = function(n, fade_len, channel_mask)
              end
              "#);

            lua_state.do_string(r#"flip = function()
              end
              "#);

            lua_state.do_string(r#"color = function(c)
                c = math.floor(c)
                s:color(c)
              end
              "#);

            lua_state.do_string(r#"peek = function(addr)
                return 0
              end
              "#);

            lua_state.do_string(r#"poke = function(addr, val)
              end
              "#);

            lua_state.do_string(r#"stat = function(x)
                v = s:stat(x)
                return v
              end
              "#);


            /* CARTDATA */
            lua_state.do_string(r#"cartdata = function(x)
              x = math.floor(x)
              s:cartdata(x)
              end
              "#);

            lua_state.do_string(r#"dget = function(x)
              x = math.floor(x)
              return s:dget(x)
              end
              "#);

            lua_state.do_string(r#"dset = function(x, y)
              x = math.floor(x)
              y = math.floor(y)

              s:dset(x, y)
              end
              "#);

            /* PICO8 compatible functions */

            lua_state.do_string(r#"
               function min(a,b)
                    if a == nil or b == nil then
                            warning('min a or b are nil returning 0')
                            return 0
                    end
                    if a < b then return a end
                    return b
                end

                function max(a,b)
                        if a == nil or b == nil then
                                warning('max a or b are nil returning 0')
                                return 0
                        end
                        if a > b then return a end
                        return b
                end

                function mid(x,y,z)
                        x = x or 0
                        y = y or 0
                        z = z or 0
                        return x > y and x or y > z and z or y
                end


              function add(a,v)
                if a == nil then
                  warning('add to nil')
                  return
                end
                table.insert(a,v)
              end

              function del(a,dv)
                if a == nil then
                  warning('del from nil')
                  return
                end
                for i,v in ipairs(a) do
                  if v==dv then
                    table.remove(a,i)
                  end
                end
              end

              function warning(msg)
                log(debug.traceback("WARNING: "..msg,3))
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
                if i <= n then return a[i] end
              end
            end
              "#);

            let value = lua_state.do_string(r#"

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

            sub = string.sub
            "#);

            error!("SMALL FUNCTIONS = {:?}", value);
        }

        pub fn init(&mut self) {
            if ! self.loaded_code {
                return;
            }

            let mut lua_state = self.lua_state.lock().unwrap();

            let value = lua_state.do_string("_init()");
            if value != ThreadStatus::Ok {
                error!("INIT = {:?}", value);
            } else {
                info!("INIT SUCCESS");
            }
        }

        pub fn draw(&mut self) -> bool {
            if ! self.loaded_code {
                return false;
            }

            let mut lua_state = self.lua_state.lock().unwrap();

            let value = lua_state.do_string("_draw()");
            if value != ThreadStatus::Ok {
                error!("[PLUGIN][LUA] DRAW = {:?}", value);
            }

            return true;
        }

        pub fn update(&mut self) -> bool {
            if ! self.loaded_code {
                return false;
            }

            let mut lua_state = self.lua_state.lock().unwrap();

            let value = lua_state.do_string("_update()");
            if value != ThreadStatus::Ok {
                let value = lua_state.do_string("_update60()");
                if value != ThreadStatus::Ok {
                    error!("UPDATE = {:?}", value);
                }
            }

            return true;

        }

        pub fn load_code(&mut self, data: String) -> bool {
            info!("LOAD CODE");
            let mut lua_state = self.lua_state.lock().unwrap();

            self.loaded_code = true;

            let value = lua_state.do_string(&data);
            if value != ThreadStatus::Ok {
                error!("LOAD CODE = {:?}", value);
                self.loaded_code = false;
            }

            self.loaded_code
        }
    }

    struct PX8Lua {
    }

    impl PX8Lua {
        fn new() -> PX8Lua {
            info!("PX8LUA NEW");
            return PX8Lua{};
        }

        #[allow(non_snake_case)]
        unsafe extern "C" fn lua_new(lua_context: *mut lua_State) -> c_int {
            let mut state = State::from_ptr(lua_context);

            // construct new userdata in lua space and initialize it
            *state.new_userdata_typed::<PX8Lua>() = PX8Lua::new();
            // set the userdata's metatable so we can call methods on it
            state.set_metatable_from_registry("PX8Lua");
            // return the userdata on top of the stack
            1
        }

        unsafe extern "C" fn lua_camera(lua_context: *mut lua_State) -> c_int {
            debug!("LUA CAMERA");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            screen.lock().unwrap().camera(x as i32, y as i32);

            1
        }

        unsafe extern "C" fn lua_color(lua_context: *mut lua_State) -> c_int {
            debug!("LUA COLOR");

            let mut state = State::from_ptr(lua_context);

            let value = state.check_integer(2);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            screen.lock().unwrap().color(value as i32);

            1
        }

        unsafe extern "C" fn lua_btn(lua_context: *mut lua_State) -> c_int {
            debug!("LUA BTN");

            let mut state = State::from_ptr(lua_context);
            let mut state2 = State::from_ptr(lua_context);

            let players = state2.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.players.clone()
            });

            let player = state.check_integer(2);

            let i = state.check_integer(3);

            let players_data = players.lock().unwrap();

            let value = players_data.get_value(player as u8, i as u8);

            state.push_integer(value as i64);

            1
        }


        unsafe extern "C" fn lua_btnp(lua_context: *mut lua_State) -> c_int {
            debug!("LUA BTN");

            let mut state = State::from_ptr(lua_context);
            let mut state2 = State::from_ptr(lua_context);

            let players = state2.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.players.clone()
            });

            let player = state.check_integer(2);
            let i = state.check_integer(3);

            let mut players_data = players.lock().unwrap();

            let value = players_data.get_value_quick(player as u8, i as u8);

            state.push_integer(value as i64);

            1
        }

        unsafe extern "C" fn lua_cls(lua_context: *mut lua_State) -> c_int {
            let mut state = State::from_ptr(lua_context);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            screen.lock().unwrap().cls();

            1
        }

        #[allow(non_snake_case)]
        unsafe extern "C" fn lua_rect(lua_context: *mut lua_State) -> c_int {
            debug!("LUA RECT");

            let mut state = State::from_ptr(lua_context);

            let x0 = state.check_integer(2);
            let y0 = state.check_integer(3);
            let x1 = state.check_integer(4);
            let y1 = state.check_integer(5);
            let col = state.check_integer(6);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            screen.lock().unwrap().rect(x0 as i32, y0 as i32, x1 as i32, y1 as i32, col as i32);

            1
        }

        #[allow(non_snake_case)]
        unsafe extern "C" fn lua_rectfill(lua_context: *mut lua_State) -> c_int {
            debug!("LUA RECTFILL");

            let mut state = State::from_ptr(lua_context);

            let x0 = state.check_integer(2);
            let y0 = state.check_integer(3);
            let x1 = state.check_integer(4);
            let y1 = state.check_integer(5);
            let col = state.check_integer(6);

            debug!("LUA RECTFILL x0:{:?} y0:{:?} x1:{:?} y1:{:?} col:{:?}", x0, y0, x1, y1, col);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });
            screen.lock().unwrap().rectfill(x0 as i32, y0 as i32, x1 as i32, y1 as i32, col as i32);

            1
        }

        #[allow(non_snake_case)]
        unsafe extern "C" fn lua_circ(lua_context: *mut lua_State) -> c_int {
            debug!("LUA CIRC");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);
            let r = state.check_integer(4);
            let col = state.check_integer(5);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            screen.lock().unwrap().circ(x as i32, y as i32, r as i32, col as i32);

            1
        }

        #[allow(non_snake_case)]
        unsafe extern "C" fn lua_circfill(lua_context: *mut lua_State) -> c_int {
            debug!("LUA CIRCFILL");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);
            let r = state.check_integer(4);
            let col = state.check_integer(5);

            debug!("LUA CIRCFILL {:?} {:?} {:?} {:?}", x, y, r, col);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            screen.lock().unwrap().circfill(x as i32, y as i32, r as i32, col as i32);

            1
        }

        #[allow(non_snake_case)]
        unsafe extern "C" fn lua_clip(lua_context: *mut lua_State) -> c_int {
            debug!("LUA CLIP");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);
            let w = state.check_integer(4);
            let h = state.check_integer(5);

            debug!("LUA CLIP {:?} {:?} {:?} {:?}", x, y, w, h);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            screen.lock().unwrap().clip(x as i32, y as i32, w as i32, h as i32);

            1
        }

        #[allow(non_snake_case)]
        unsafe extern "C" fn lua_ellipse(lua_context: *mut lua_State) -> c_int {
            debug!("LUA ELLIPSE");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);
            let rx = state.check_integer(4);
            let ry = state.check_integer(5);
            let col = state.check_integer(6);

            debug!("LUA ELLIPSE x:{:?} y:{:?} rx:{:?} ry:{:?} col:{:?}", x, y, rx, ry, col);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });
            screen.lock().unwrap().ellipse(x as i32, y as i32, rx as i32, ry as i32, col as i32);

            1
        }

        #[allow(non_snake_case)]
        unsafe extern "C" fn lua_ellipsefill(lua_context: *mut lua_State) -> c_int {
            debug!("LUA ELLIPSEFILL");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);
            let rx = state.check_integer(4);
            let ry = state.check_integer(5);
            let col = state.check_integer(6);

            debug!("LUA ELLIPSEFILL x:{:?} y:{:?} rx:{:?} ry:{:?} col:{:?}", x, y, rx, ry, col);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });
            screen.lock().unwrap().ellipsefill(x as i32, y as i32, rx as i32, ry as i32, col as i32);

            1
        }


        #[allow(non_snake_case)]
        unsafe extern "C" fn lua_trigon(lua_context: *mut lua_State) -> c_int {
            debug!("LUA TRIGON");

            let mut state = State::from_ptr(lua_context);

            let x1 = state.check_integer(2);
            let y1 = state.check_integer(3);
            let x2 = state.check_integer(4);
            let y2 = state.check_integer(5);
            let x3 = state.check_integer(6);
            let y3 = state.check_integer(7);
            let col = state.check_integer(8);

            debug!("LUA TRIGON x1:{:?} y1:{:?} x2:{:?} y2:{:?} x3:{:?} y3:{:?} col:{:?}", x1, y1, x2, y2, x3, y3, col);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });
            screen.lock().unwrap().trigon(x1 as i32, y1 as i32, x2 as i32, y2 as i32, x3 as i32, y3 as i32, col as i32);

            1
        }

        #[allow(non_snake_case)]
        unsafe extern "C" fn lua_palt(lua_context: *mut lua_State) -> c_int {
            debug!("LUA PALT");

            let mut state = State::from_ptr(lua_context);

            let c = state.check_integer(2);
            let t = state.check_integer(3);

            debug!("LUA PALT {:?} {:?}", c, t);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            screen.lock().unwrap().palt(c as i32, t == 1);

            1
        }


        unsafe extern "C" fn lua_pal(lua_context: *mut lua_State) -> c_int {
            debug!("LUA PAL");

            let mut state = State::from_ptr(lua_context);

            let c0 = state.check_integer(2);
            let c1 = state.check_integer(3);
//            let p = state.check_integer(4);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            screen.lock().unwrap().pal(c0 as i32, c1 as i32);

            1
        }

        unsafe extern "C" fn lua_pset(lua_context: *mut lua_State) -> c_int {
            debug!("LUA PSET");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);
            let col = state.check_integer(4);

            if x < 0 || y < 0 {
                return 1;
            }

            if x as usize >= SCREEN_HEIGHT || y as usize >= SCREEN_WIDTH {
                return 1;
            }

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            screen.lock().unwrap().pset(x as i32, y as i32, col as i32);

            1
        }

        unsafe extern "C" fn lua_pget(lua_context: *mut lua_State) -> c_int {
            debug!("LUA PGET");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);

            if x < 0 || y < 0 {
                return 1;
            }

            if x as usize >= SCREEN_HEIGHT || y as usize >= SCREEN_WIDTH {
                return 1;
            }

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            let value = screen.lock().unwrap().pget(x as u32, y as u32);

            state.push_integer(value as i64);

            1
        }


        unsafe extern "C" fn lua_sget(lua_context: *mut lua_State) -> c_int {
            debug!("LUA SGET");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);

            debug!("LUA SGET {:?} {:?}", x, y);

            if x < 0 || y < 0 {
                return 1;
            }

            if x as usize >= 128 || y as usize >= 128 {
                return 1;
            }

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            let value = screen.lock().unwrap().sget(x as u32, y as u32);

            state.push_integer(value as i64);

            1
        }

        unsafe extern "C" fn lua_sset(lua_context: *mut lua_State) -> c_int {
            debug!("LUA SSET");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);
            let col = state.check_integer(4);

            debug!("LUA SSET {:?} {:?} {:?}", x, y, col);

            if x < 0 || y < 0 {
                return 1;
            }

            if x as usize >= 128 || y as usize >= 128 {
                return 1;
            }

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            screen.lock().unwrap().sset(x as u32, y as u32, col as i32);

            1
        }

        unsafe extern "C" fn lua_line(lua_context: *mut lua_State) -> c_int {
            debug!("LUA LINE");

            let mut state = State::from_ptr(lua_context);

            let x0 = state.check_integer(2);
            let y0 = state.check_integer(3);
            let x1 = state.check_integer(4);
            let y1 = state.check_integer(5);
            let col = state.check_integer(6);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            screen.lock().unwrap().line(x0 as i32, y0 as i32, x1 as i32, y1 as i32, col as i32);

            1
        }

        unsafe extern "C" fn lua_fget(lua_context: *mut lua_State) -> c_int {
            debug!("LUA FGET");

            let mut state = State::from_ptr(lua_context);

            let idx = state.check_integer(2);
            let flag = state.check_integer(3);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            state.push_bool(screen.lock().unwrap().fget(idx as u32, flag as u8));

            1
        }

        unsafe extern "C" fn lua_fget_all(lua_context: *mut lua_State) -> c_int {
            debug!("LUA FGET ALL");

            let mut state = State::from_ptr(lua_context);

            let idx = state.check_integer(2);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            state.push_integer(screen.lock().unwrap().fget_all(idx as u32) as i64);

            1
        }

        unsafe extern "C" fn lua_fset(lua_context: *mut lua_State) -> c_int {
            debug!("LUA FSET");

            let mut state = State::from_ptr(lua_context);

            let idx = state.check_integer(2);
            let flag = state.check_integer(3);
            let value = state.check_integer(4);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            screen.lock().unwrap().fset(idx as u32, flag as u8, value == 1);

            1
        }

        unsafe extern "C" fn lua_fset_all(lua_context: *mut lua_State) -> c_int {
            debug!("LUA FSET ALL");

            let mut state = State::from_ptr(lua_context);

            let idx = state.check_integer(2);
            let flags = state.check_integer(3);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            screen.lock().unwrap().fset_all(idx as u32, flags as u8);

            1
        }

        unsafe extern "C" fn lua_rnd(lua_context: *mut lua_State) -> c_int {
            debug!("LUA RND");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);

            let value: f64;

            if x == 0 {
                value = rand::thread_rng().gen_range(0.0, 1.0);
            } else {
                value = rand::thread_rng().gen_range(0.0, x as f64);
            }

            state.push_number(value);

            1
        }

        // spr n x y [w h] [flip_x] [flip_y]
        unsafe extern "C" fn lua_spr(lua_context: *mut lua_State) -> c_int {
            debug!("LUA SPR");

            let mut state = State::from_ptr(lua_context);

            let n = state.check_integer(2);
            let x = state.check_integer(3);
            let y = state.check_integer(4);
            let w = state.check_integer(5);
            let h = state.check_integer(6);
            let flip_x = state.check_integer(7);
            let flip_y = state.check_integer(8);

            debug!("LUA SPR n:{:?} x:{:?} y:{:?} w:{:?} h:{:?} flip_x:{:?} flip_y:{:?}", n, x, y, w, h, flip_x, flip_y);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            screen.lock().unwrap().spr(n as u32, x as i32, y as i32, w as u32, h as u32, flip_x == 1, flip_y == 1);

            1
        }

        // sspr sx sy sw sh dx dy [dw dh] [flip_x] [flip_y]
        unsafe extern "C" fn lua_sspr(lua_context: *mut lua_State) -> c_int {
            debug!("LUA SSPR");

            let mut state = State::from_ptr(lua_context);

            let sx = state.check_integer(2);
            let sy = state.check_integer(3);
            let sw = state.check_integer(4);
            let sh = state.check_integer(5);
            let dx = state.check_integer(6);
            let dy = state.check_integer(7);
            let dw = state.check_integer(8);
            let dh = state.check_integer(9);
            let flip_x = state.check_integer(10);
            let flip_y = state.check_integer(11);

            debug!("LUA SSPR sx:{:?} sy:{:?} sw:{:?} sh:{:?} dx:{:?} dy:{:?} dw:{:?} dh:{:?} flip_x:{:?} flip_y:{:?}", sx, sy, sw, sh, dx ,dy, dw, dh, flip_x, flip_y);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            screen.lock().unwrap().sspr(sx as u32,
                                        sy as u32,
                                        sw as u32,
                                        sh as u32,
                                        dx as i32,
                                        dy as i32,
                                        dw as u32,
                                        dh as u32,
                                        flip_x == 1,
                                        flip_y == 1);

            1
        }

        // map cel_x cel_y sx sy cel_w cel_h [layer]
        unsafe extern "C" fn lua_map(lua_context: *mut lua_State) -> c_int {
            debug!("LUA MAP");

            let mut state = State::from_ptr(lua_context);

            let cel_x = state.check_integer(2);
            let cel_y = state.check_integer(3);
            let sx = state.check_integer(4);
            let sy = state.check_integer(5);
            let cel_w = state.check_integer(6);
            let cel_h = state.check_integer(7);
            let layer = state.check_integer(8);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            screen.lock().unwrap().map(cel_x as u32, cel_y as u32,
                                       sx as i32, sy as i32,
                                       cel_w as u32, cel_h as u32,
                                       layer as u8);


            1
        }

        unsafe extern "C" fn lua_mget(lua_context: *mut lua_State) -> c_int {
            debug!("LUA MGET");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            let value = screen.lock().unwrap().mget(x as i32, y as i32);

            state.push_integer(value as i64);

            1
        }


        unsafe extern "C" fn lua_mset(lua_context: *mut lua_State) -> c_int {
            debug!("LUA MSET");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);
            let v = state.check_integer(4);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            screen.lock().unwrap().mset(x as i32, y as i32, v as u32);

            1
        }

        unsafe extern "C" fn lua_print(lua_context: *mut lua_State) -> c_int {
            debug!("LUA PRINT");

            let mut state = State::from_ptr(lua_context);
            let mut state2 = State::from_ptr(lua_context);

            let str_data = state2.check_string(2);
            let x = state.check_integer(3);
            let y = state.check_integer(4);
            let col = state.check_integer(5);

            let screen = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.screen.clone()
            });

            screen.lock().unwrap().print(str_data.to_string(), x as i32, y as i32, col as i32);

            1
        }

        unsafe extern "C" fn lua_time(lua_context: *mut lua_State) -> c_int {
            debug!("LUA TIME");

            let mut state = State::from_ptr(lua_context);

            let info = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.info.clone()
            });

            info!("LUA TIME {:?}", info.lock().unwrap().real_time);

            state.push_number(info.lock().unwrap().real_time);

            1
        }

        unsafe extern "C" fn lua_stat(lua_context: *mut lua_State) -> c_int {
            debug!("LUA STAT");

            let mut state = State::from_ptr(lua_context);

            let value = state.check_integer(2);

            let players = state.with_extra(|extra| {
                let data = extra.as_ref().unwrap().downcast_ref::<ExtraData>().unwrap();
                data.players.clone()
            });

            let players_data = players.lock().unwrap();

            if value == 32 {
                state.push_integer(players_data.mouse.x as i64);
            } else if value == 33 {
                state.push_integer(players_data.mouse.y as i64);
            } else if value == 34 {
                state.push_integer(players_data.mouse.state as i64);
            } else {
                state.push_integer(0);
            }

            1
        }


        /***** CARTDATA *****/
        unsafe extern "C" fn lua_cartdata(_lua_context: *mut lua_State) -> c_int {
            debug!("LUA CARTDATA");

            /*
            let mut state = State::from_ptr(lua_context);
            let x = state.check_integer(2);
            */

            1
        }

        unsafe extern "C" fn lua_dget(_lua_context: *mut lua_State) -> c_int {
            debug!("LUA DGET");

            /*
            let mut state = State::from_ptr(lua_context);
            let x = state.check_integer(2);
            */

            1
        }


        unsafe extern "C" fn lua_dset(_lua_context: *mut lua_State) -> c_int {
            debug!("LUA DSET");

            /*
            let mut state = State::from_ptr(lua_context);
            let x = state.check_integer(2);
            */

            1
        }

    }

    pub const PX8LUA_LIB: [(&'static str, Function); 37] = [
        ("new", Some(PX8Lua::lua_new)),

        ("camera", Some(PX8Lua::lua_camera)),
        ("color", Some(PX8Lua::lua_color)),


        ("btn", Some(PX8Lua::lua_btn)),
        ("btnp", Some(PX8Lua::lua_btnp)),

        ("cls", Some(PX8Lua::lua_cls)),

        ("fget", Some(PX8Lua::lua_fget)),
        ("fget_all", Some(PX8Lua::lua_fget_all)),
        ("fset", Some(PX8Lua::lua_fset)),
        ("fset_all", Some(PX8Lua::lua_fset_all)),

        ("line", Some(PX8Lua::lua_line)),

        ("rect", Some(PX8Lua::lua_rect)),
        ("rectfill", Some(PX8Lua::lua_rectfill)),
        ("circ", Some(PX8Lua::lua_circ)),
        ("circfill", Some(PX8Lua::lua_circfill)),
        ("ellipse", Some(PX8Lua::lua_ellipse)),
        ("ellipsefill", Some(PX8Lua::lua_ellipsefill)),
        ("trigon", Some(PX8Lua::lua_trigon)),

        ("clip", Some(PX8Lua::lua_clip)),

        ("spr", Some(PX8Lua::lua_spr)),
        ("sspr", Some(PX8Lua::lua_sspr)),

        ("map", Some(PX8Lua::lua_map)),
        ("mget", Some(PX8Lua::lua_mget)),
        ("mset", Some(PX8Lua::lua_mset)),

        ("palt", Some(PX8Lua::lua_palt)),
        ("pal", Some(PX8Lua::lua_pal)),

        ("pget", Some(PX8Lua::lua_pget)),
        ("pset", Some(PX8Lua::lua_pset)),

        ("sget", Some(PX8Lua::lua_sget)),
        ("sset", Some(PX8Lua::lua_sset)),

        ("rnd", Some(PX8Lua::lua_rnd)),

        ("print", Some(PX8Lua::lua_print)),

        ("time", Some(PX8Lua::lua_time)),

        ("stat", Some(PX8Lua::lua_stat)),

        ("cartdata", Some(PX8Lua::lua_cartdata)),
        ("dget", Some(PX8Lua::lua_dget)),
        ("dset", Some(PX8Lua::lua_dset)),

    ];
}

#[cfg(not(feature = "lua"))]
pub mod plugin {
    use std::sync::{Arc, Mutex};

    use gfx::Sprite;
    use config::Players;

    use px8;
    use px8::info::Info;

    use gfx::{SCREEN_WIDTH, SCREEN_HEIGHT};
    use gfx::Screen;


    pub struct LuaPlugin {
    }

    impl LuaPlugin {
        pub fn new() -> LuaPlugin {
            LuaPlugin {}
        }

        // Keep the compatibility
        pub fn load(&mut self,
                    players: Arc<Mutex<Players>>,
                    info: Arc<Mutex<Info>>,
                    screen: Arc<Mutex<Screen>>) {
            panic!("LUA plugin disabled");
        }
        pub fn load_code(&mut self, data: String) -> bool { false }
        pub fn init(&mut self) {}
        pub fn draw(&mut self) -> bool { false }
        pub fn update(&mut self) -> bool { false }
    }
}
