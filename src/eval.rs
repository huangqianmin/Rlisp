use std::cell::RefCell;
use std::rc::Rc;

use crate::env::*;
use crate::eval;
use crate::object::*;
use crate::parser::*;

fn eval_binary_op(
    list: &Vec<Object>,
    env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
    if list.len() != 3 {
        return Err(format!(
            "Invalid number of arguments for binary operation"
        ));
    }

    let operation = &list[0];
    let left = eval_obj(&list[1], env).unwrap();
    let right = eval_obj(&list[2], env).unwrap();

    match operation {
        Object::BinaryOp(op) => match op.as_str() {
            "+" => match (&left, &right) {
                (Object::Integer(l), Object::Integer(r)) => {
                    Ok(Object::Integer(l + r))
                }
                (Object::Float(l), Object::Float(r)) => {
                    Ok(Object::Float(l + r))
                }
                (Object::Integer(l), Object::Float(r)) => {
                    Ok(Object::Float(*l as f64 + r))
                }
                (Object::Float(l), Object::Integer(r)) => {
                    Ok(Object::Float(l + *r as f64))
                }
                (Object::String(l), Object::String(r)) => {
                    Ok(Object::String(format!("{}{}", l, r)))
                }
                _ => Err(format!(
                    "Invalid types for + operator {} {}",
                    left, right
                )),
            },
            "-" => match (&left, &right) {
                (Object::Integer(l), Object::Integer(r)) => {
                    Ok(Object::Integer(l - r))
                }
                (Object::Float(l), Object::Float(r)) => {
                    Ok(Object::Float(l - r))
                }
                (Object::Integer(l), Object::Float(r)) => {
                    Ok(Object::Float(*l as f64 - r))
                }
                (Object::Float(l), Object::Integer(r)) => {
                    Ok(Object::Float(l - *r as f64))
                }
                _ => Err(format!(
                    "Invalid types for - operator {} {}",
                    left, right
                )),
            },
            "=" => match (&left, &right) {
                (Object::Integer(l), Object::Integer(r)) => {
                    return Ok(Object::Bool(l == r))
                }
                (Object::String(l), Object::String(r)) => {
                    return Ok(Object::Bool(l == r))
                }
                _ => Err(format!(
                    "Invalid types for = operator {} {}",
                    left, right
                )),
            },
            _ => todo!(),
        },
        _ => todo!(),
    }
}

fn eval_obj(
    obj: &Object,
    env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
    let current_obj = obj;
    let mut current_env = env;
    loop {
        match current_obj {
            Object::List(list) => {
                let head = &list[0];
                match head {
                    Object::BinaryOp(_op) => {
                        return eval_binary_op(
                            list,
                            &mut current_env,
                        );
                    }
                    _ => todo!(),
                }
            }
            Object::Integer(i) => {
                return Ok(Object::Integer(*i))
            }
            Object::Float(f) => return Ok(Object::Float(*f)),
            Object::String(s) => {
                return Ok(Object::String(s.clone()))
            }
            _ => todo!(),
        }
    }
}

pub fn eval(
    input: &str,
    env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
    let parsed_list = parse(input);
    if parsed_list.is_err() {
        return Err(format!("{}", parsed_list.err().unwrap()));
    }
    eval_obj(&parsed_list.unwrap(), env)
}

mod tests {
    use super::*;

    #[test]
    fn test_int_int_add() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(+ 1 2)", &mut env).unwrap();
        assert_eq!(result, Object::Integer(3));
    }

    #[test]
    fn test_int_float_add() {
        let mut env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(+ 1 3.0)", &mut env).unwrap();
        assert_eq!(result, Object::Float(4.0));
    }

    #[test]
    fn test_float_int_add() {
        let mut env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(+ 2.0 2)", &mut env).unwrap();
        assert_eq!(result, Object::Float(4.0));
    }

    #[test]
    fn test_float_float_add() {
        let mut env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(+ 1.0 2.0)", &mut env).unwrap();
        assert_eq!(result, Object::Float(3.0));
    }

    #[test]
    fn test_str_add() {
        let mut env = Rc::new(RefCell::new(Env::new()));

        let result =
            eval("(+ \"hello\" \"world\")", &mut env).unwrap();
        assert_eq!(
            result,
            Object::String("helloworld".to_string())
        );
    }

    #[test]
    fn test_int_int_sub() {
        let mut env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(- 1 2)", &mut env).unwrap();
        assert_eq!(result, Object::Integer(-1));
    }

    #[test]
    fn test_int_float_sub() {
        let mut env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(- 1.0 2.0)", &mut env).unwrap();
        assert_eq!(result, Object::Float(-1.0));
    }

    #[test]
    fn test_float_int_sub() {
        let mut env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(- 2.0 2)", &mut env).unwrap();
        assert_eq!(result, Object::Float(0.0))
    }

    #[test]
    fn test_str_whitespace_add() {
        let mut env = Rc::new(RefCell::new(Env::new()));

        let result =
            eval("(+ \"hello \" \"world\")", &mut env).unwrap();
        assert_eq!(
            result,
            Object::String("hello world".to_string())
        );
    }

    #[test]
    fn test_int_eq_false() {
        let mut env = Rc::new(RefCell::new(Env::new()));

        let reust = eval("(= 1 2)", &mut env).unwrap();
        assert_eq!(reust, Object::Bool(false));
    }

    #[test]
    fn test_int_eq_true() {
        let mut env = Rc::new(RefCell::new(Env::new()));

        let reust = eval("(= 100 100)", &mut env).unwrap();
        assert_eq!(reust, Object::Bool(true));
    }

    #[test]
    fn test_str_eq_false() {
        let mut env = Rc::new(RefCell::new(Env::new()));

        let reust =
            eval("(= \"hello\" \"world\")", &mut env).unwrap();
        assert_eq!(reust, Object::Bool(false));
    }

    #[test]
    fn test_str_eq_true() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let reust =
            eval("(= \"hello\" \"hello\")", &mut env).unwrap();
        assert_eq!(reust, Object::Bool(true));
    }
}
