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

    use contexts::Contexts;

    use core::info::Info;

    use gfx::Screen;

    use crate::core::AudioCommandBuffer;
    use crate::core::AudioSyncCommand;

    pub struct ExtraData {
        /* External objects to get access to Unicorn data ! */
        pub contexts: Arc<Mutex<Contexts>>,
        pub info: Arc<Mutex<Info>>,
        pub screen: Arc<Mutex<Screen>>,
        pub audio: Arc<Mutex<AudioCommandBuffer>>,
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
        show_mouse              #               #               #
*/    

            methods.add_method("play_note", |_lua_ctx, game_state, (note_idx, instrument_idx, channel):(u32, u32, u8)| {
                game_state.audio.lock().unwrap().push(AudioSyncCommand::PressedKey {note_index: note_idx as usize, instrument_index: instrument_idx as usize, channel: channel as usize});
                Ok(0)
            });

            methods.add_method("trigger_note", |_lua_ctx, game_state, (note_idx, instrument_idx):(u32, u32)| {
                game_state.audio.lock().unwrap().push(AudioSyncCommand::TriggerNote {note_index: note_idx as usize, instrument_index: instrument_idx as usize});
                Ok(0)
            });

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

            methods.add_method("srand", |_lua_ctx, game_state, x:u64| {
                ChaCha8Rng::seed_from_u64(x.into());
                Ok(1)
            });

            methods.add_method("time", |_lua_ctx, game_state, ()| {
                let value = game_state.info.lock().unwrap().time();
                Ok(value)
            });

            methods.add_method("mtime", |_lua_ctx, game_state, ()| {
                let value = game_state.info.lock().unwrap().mtime();
                Ok(value)
            });

            methods.add_method("utime", |_lua_ctx, game_state, ()| {
                let value = game_state.info.lock().unwrap().utime();
                Ok(value)
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
        sspr                    #     X         #               #
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
    
            methods.add_method("fillp", |_lua_ctx, game_state, (pat, transparent):(u32, bool)| {
                game_state.screen
               .lock()
               .unwrap()
               .fillp(pat, transparent);
               
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

            methods.add_method("pal", |_lua_ctx, game_state, (c0, c1, _pal_idx):(i32, i32, i32)| {
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

            methods.add_method("sspr", |_lua_ctx, game_state, (sx, sy, sw, sh, dx, dy, dw, dh, flip_x, flip_y):(u32, u32, u32, u32, i32, i32, u32, u32, bool, bool)| {
                game_state.screen
               .lock()
               .unwrap()
               .sspr(sx, sy, sw, sh, dx, dy, dw, dh, flip_x, flip_y);

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
    }

    impl LuaPlugin {
        pub fn new() -> LuaPlugin {
            LuaPlugin {
                lua: Lua::new(),
                loaded_code: false,
            }
        }

        pub fn load(&mut self,
                    contexts: Arc<Mutex<Contexts>>,
                    info: Arc<Mutex<Info>>,
                    screen: Arc<Mutex<Screen>>,
                    audio: Arc<Mutex<AudioCommandBuffer>>) {
            info!("[PLUGIN][LUA] Init plugin");
            
            self._load_pico8_functions();

            self.lua.context(|lua| {
                let globals = lua.globals();
                let userdata = lua.create_userdata(ExtraData{
                    contexts:contexts.clone(), 
                    info:info.clone(),
                    screen:screen.clone(),
                    audio:audio.clone()}).unwrap();
                
                globals.set("userdata", userdata.clone()).unwrap();
                let data = include_str!("lua/api.lua").to_string();
                lua.load(&data).exec().unwrap();

            });
        }

        fn _load_pico8_functions(&mut self) {
            info!("[PLUGIN][LUA] Load Pico8 functions");

            self.lua.context(|lua| {
                let data = include_str!("lua/pico8.lua").to_string();
                lua.load(&data).exec().unwrap();
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
    use crate::core::AudioCommandBuffer;

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
                    _screen: Arc<Mutex<Screen>>,
                    _audio: Arc<Mutex<AudioCommandBuffer>>) {
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
