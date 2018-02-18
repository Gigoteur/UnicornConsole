extern crate unicorn_web;
extern crate unicorn;

use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::rc::Rc;

#[macro_use]
extern crate stdweb;

use stdweb::web::{
    self
};

fn main() {
    stdweb::initialize();

    let canvas = web::document().get_element_by_id( "viewport" ).unwrap();
    let uc = Rc::new( RefCell::new( unicorn_web::UnicornWeb::new( &canvas ) ) );

    uc.borrow_mut().state.setup();
    uc.borrow_mut().state.init();

    uc.borrow_mut().state.toggle_debug();

    let data = include_bytes!("../../../unicorn/sys/unicorn.uni");
    let data_final: Vec<u8> = unicorn::unicorn::array_to_vec(data);

    uc.borrow_mut().state.load_cartridge_raw("unicorn.uni", data_final, true);

    unicorn_web::hide( "loading" );
    unicorn_web::hide( "error" );

    unicorn_web::show( "viewport" );

    unicorn_web::support_input( uc.clone() );

    web::window().request_animation_frame( move |_| {
        unicorn_web::main_loop( uc );
    });

    stdweb::event_loop();
}