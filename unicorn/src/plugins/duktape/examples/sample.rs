extern crate duktape;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use duktape::*;
use duktape::types::*;
use duktape::errors::*;

pub struct Point2D {
    x: i64,
    y: i64
}
impl Point2D {
    fn new(x: i64, y: i64) -> Point2D {
        return Point2D {
            x: x, y: y
        };
    }

    pub fn add(&self, _ctx: &mut Context, args: &[Value<'static>]) -> 
        DuktapeResult<Value<'static>>
    {
        println!("RUST ADD {:?}", args);

        let mut sum = 0.0;
        for arg in args.iter() {
            // TODO: Type checking.
            if let &Value::Number(n) = arg  {
                sum += n;
            }
        }
        Ok(Value::Number(sum))
    }

    pub fn min(&self, _ctx: &mut Context, args: &[Value<'static>]) -> 
        DuktapeResult<Value<'static>>
    {
        println!("RUST MIN {:?}", args);

        let mut sum = 0.0;
        for arg in args.iter() {
            // TODO: Type checking.
            if let &Value::Number(n) = arg {
                sum -= n;
            }
        }
        Ok(Value::Number(sum))
    }
}

impl Foo for Point2D {
    fn dispatch(&mut self, _ctx: &mut Context, args: &[Value<'static>]) -> DuktapeResult<Value<'static>> {
        println!("I'm here {:?}", args);
        println!("CONTEXT {:?}", _ctx.dump_context());
        
        if let Value::Number(function_idx) = args[0] {
            let function_idx = function_idx as i32;

            let args = &args[1..args.len()];
            if function_idx == 1 {
                return self.add(_ctx, args);
            } else if function_idx == 2 {
                return self.min(_ctx, args);
            }

        }

        Ok(Value::Number(0.))
    }
}

fn main() {
    let mut ctx = duktape::Context::new();
    let mut p = Arc::new(Mutex::new(Point2D::new(0, 0)));
    
   // ctx.eval_string("1+2".to_string());
   // println!("VALUE = {:?}", ctx.get_int(-1));

    ctx.register(0x1, "add", p.clone(), Some(2));
    ctx.register(0x2, "minus", p.clone(), Some(2));

    ctx.eval("function _init() { add(2.0, 4.0); }").unwrap();
    ctx.eval("_init();").unwrap();

    //assert_eq!(Value::Number(6.0), ctx.eval("add(2.0, 4.0)").unwrap());
    //assert_eq!(Value::Number(-6.0), ctx.eval("minus(2.0, 4.0)").unwrap());

    /*  ctx.eval("function add2(x, y) { return x+y; }").unwrap();
    assert_eq!(Ok(Value::Number(3.0)), ctx.call("add2", &[&2.0f64, &1.0f64]));

  ctx.eval("function id(x) { return x; }").unwrap();
    assert_eq!(Ok(Value::Null),  ctx.call("id", &[&Json::Null]));
    assert_eq!(Ok(Value::Bool(true)),  ctx.call("id", &[&true]));
    assert_eq!(Ok(Value::Bool(false)), ctx.call("id", &[&false]));
    assert_eq!(Ok(Value::Number(1.5)), ctx.call("id", &[&1.5f64]));
    assert_eq!(Ok(Value::String(Cow::Borrowed("é"))),
               ctx.call("id", &[&"é"]));*/

}