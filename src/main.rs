use rlisp::env;
use rlisp::eval;
use rlisp::object::Object;

use linefeed::{Interface, ReadResult};
use std::cell::RefCell;
use std::rc::Rc;

const PROMPT: &str = "lisp-rs> ";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = Interface::new(PROMPT).unwrap();
    reader.set_prompt(PROMPT).unwrap();
    let mut env = Rc::new(RefCell::new(env::Env::new()));

    while let ReadResult::Input(input) =
        reader.read_line().unwrap()
    {
        if input.eq("exit") {
            break;
        }

        let val = eval::eval(input.as_ref(), &mut env)?;
        match val {
            Object::Void => {}
            Object::Integer(n) => println!("{}", n),
            Object::Bool(b) => println!("{}", b),
            Object::Symbol(s) => println!("{}", s),
            _ => println!("{:?}", val),
        };
    }

    println!("Goodbye!");
    Ok(())
}
