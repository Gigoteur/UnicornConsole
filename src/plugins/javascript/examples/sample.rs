extern crate px8_plugin_javascript;
use std::thread;
use std::time::Duration;

use px8_plugin_javascript::*;
use px8_plugin_javascript::types::*;
use px8_plugin_javascript::errors::*;

pub fn rust_add(_ctx: &mut Context, args: &[Value<'static>]) -> 
    DuktapeResult<Value<'static>>
{
    println!("RUST ADD {:?}", args);

    let mut sum = 0.0;
    for arg in args.iter() {
        // TODO: Type checking.
        if let &Value::Number(n) = arg {
            sum += n;
        }
    }
    Ok(Value::Number(sum))
}

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

    pub fn rust_add(&self, _ctx: &mut Context, args: &[Value<'static>]) -> 
        DuktapeResult<Value<'static>>
    {
        println!("RUST ADD {:?}", args);

        let mut sum = 0.0;
        for arg in args.iter() {
            // TODO: Type checking.
            if let &Value::Number(n) = arg {
                sum += n;
            }
        }
        Ok(Value::Number(sum))
    }
}

impl Foo for Point2D {
    fn add(&self, _ctx: &mut Context, args: &[Value<'static>]) -> DuktapeResult<Value<'static>> {
        println!("I'm here {:?}", args);
        self.rust_add(_ctx, args)
    }
}

fn main() {
    let mut ctx = px8_plugin_javascript::Context::new();
    let mut p = Point2D::new(0, 0);
    
    ctx.eval_string("1+2".to_string());
    println!("VALUE = {:?}", ctx.get_int(-1));

    ctx.register("add", p, rust_add, Some(2));
    assert_eq!(Value::Number(5.0), ctx.eval("add(2.0, 3.0)").unwrap());

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