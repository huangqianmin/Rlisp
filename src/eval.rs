use std::cell::RefCell;
use std::rc::Rc;

use crate::env::*;
use crate::object::*;
use crate::parser::*;

fn eval_binary_op(
    operation: &str,
    list: &[Object],
    env: Rc<RefCell<Env>>,
) -> Result<Object, String> {
    if list.len() != 2 {
        return Err(format!(
            "Invalid number of arguments for binary operation"
        ));
    }

    let left = eval_obj(&list[0], env.clone()).unwrap();
    let right = eval_obj(&list[1], env.clone()).unwrap();

    match operation {
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
        "*" => match (&left, &right) {
            (Object::Integer(l), Object::Integer(r)) => {
                return Ok(Object::Integer(l * r))
            }
            (Object::Float(l), Object::Float(r)) => {
                return Ok(Object::Float(l * r))
            }
            (Object::Integer(l), Object::Float(r)) => {
                return Ok(Object::Float(*l as f64 * r))
            }
            (Object::Float(l), Object::Integer(r)) => {
                return Ok(Object::Float(l * *r as f64))
            }
            _ => Err(format!(
                "Invalid types for * operator {} {}",
                left, right
            )),
        },
        "/" => match (&left, &right) {
            (Object::Integer(l), Object::Integer(r)) => {
                return Ok(Object::Integer(l / r))
            }
            (Object::Float(l), Object::Float(r)) => {
                return Ok(Object::Float(l / r))
            }
            (Object::Integer(l), Object::Float(r)) => {
                return Ok(Object::Float(*l as f64 / r))
            }
            (Object::Float(l), Object::Integer(r)) => {
                return Ok(Object::Float(l / *r as f64))
            }
            _ => Err(format!(
                "Invalid types for / operator {} {}",
                left, right
            )),
        },
        "%" => match (&left, &right) {
            (Object::Integer(l), Object::Integer(r)) => {
                return Ok(Object::Integer(l % r))
            }
            (Object::Float(l), Object::Float(r)) => {
                return Ok(Object::Float(l % r))
            }
            (Object::Integer(l), Object::Float(r)) => {
                return Ok(Object::Float(*l as f64 % r))
            }
            (Object::Float(l), Object::Integer(r)) => {
                return Ok(Object::Float(l % *r as f64))
            }
            _ => Err(format!(
                "Invalid types for % operator {} {}",
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
        ">" => match (&left, &right) {
            (Object::Integer(l), Object::Integer(r)) => {
                return Ok(Object::Bool(l > r))
            }
            (Object::String(l), Object::String(r)) => {
                return Ok(Object::Bool(l > r))
            }
            _ => Err(format!(
                "Invalid types for > operator {} {}",
                left, right
            )),
        },
        "<" => match (&left, &right) {
            (Object::Integer(l), Object::Integer(r)) => {
                return Ok(Object::Bool(l < r))
            }
            (Object::String(l), Object::String(r)) => {
                return Ok(Object::Bool(l < r))
            }
            _ => Err(format!(
                "Invalid types for < operator {} {}",
                left, right
            )),
        },
        _ => todo!(),
    }
}

fn eval_begin(
    list: &[Object],
    env: Rc<RefCell<Env>>,
) -> Result<Object, String> {
    let mut result = Object::Void;

    let new_env = Rc::new(RefCell::new(Env::extend(env)));

    for obj in list {
        result = eval_obj(obj, new_env.clone())?;
    }
    Ok(result)
}

fn eval_define(
    list: &[Object],
    env: Rc<RefCell<Env>>,
) -> Result<Object, String> {
    if list.len() != 2 {
        return Err(format!(
            "Invalid number of arguments for define"
        ));
    }

    let name = match &list[0] {
        Object::Symbol(name) => name.clone(),
        _ => return Err(format!("Invalid define")),
    };
    let value = eval_obj(&list[1], env.clone())?;

    env.borrow_mut().set(name, value);
    Ok(Object::Void)
}

fn eval_keyword(
    head: &str,
    list: &[Object],
    env: Rc<RefCell<Env>>,
) -> Result<Object, String> {
    match head {
        "begin" => eval_begin(list, env.clone()),
        "define" => eval_define(list, env.clone()),
        _ => todo!(),
    }
}

fn eval_symbol(
    name: &str,
    env: Rc<RefCell<Env>>,
) -> Result<Object, String> {
    match env.borrow().get(name) {
        Some(value) => Ok(value),
        None => Err(format!("Undefined symbol {}", name)),
    }
}

fn eval_obj(
    obj: &Object,
    env: Rc<RefCell<Env>>,
) -> Result<Object, String> {
    let current_obj = obj;
    let current_env = env.clone();
    loop {
        match current_obj {
            Object::List(list) => {
                let head = &list[0];
                match head {
                    Object::BinaryOp(op) => {
                        return eval_binary_op(
                            op,
                            &list[1..],
                            current_env,
                        )
                    }
                    Object::Keyword(keyword) => {
                        return eval_keyword(
                            keyword,
                            &list[1..],
                            current_env,
                        )
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
            Object::Symbol(s) => {
                return eval_symbol(s, current_env)
            }
            _ => todo!(),
        }
    }
}

pub fn eval(
    input: &str,
    env: Rc<RefCell<Env>>,
) -> Result<Object, String> {
    let parsed_list = parse(input);
    if parsed_list.is_err() {
        return Err(format!("{}", parsed_list.err().unwrap()));
    }
    eval_obj(&parsed_list.unwrap(), env.clone())
}

mod tests {
    use super::*;

    #[test]
    fn test_add_int_int() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(+ 1 2)", env).unwrap();
        assert_eq!(result, Object::Integer(3));
    }

    #[test]
    fn test_add_int_float() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(+ 1 3.0)", env).unwrap();
        assert_eq!(result, Object::Float(4.0));
    }

    #[test]
    fn test_add_float_int() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(+ 2.0 2)", env).unwrap();
        assert_eq!(result, Object::Float(4.0));
    }

    #[test]
    fn test_add_float_float() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(+ 1.0 2.0)", env).unwrap();
        assert_eq!(result, Object::Float(3.0));
    }

    #[test]
    fn test_str_add() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result =
            eval("(+ \"hello\" \"world\")", env).unwrap();
        assert_eq!(
            result,
            Object::String("helloworld".to_string())
        );
    }

    #[test]
    fn test_sub_int_int() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(- 1 2)", env).unwrap();
        assert_eq!(result, Object::Integer(-1));
    }

    #[test]
    fn test_sub_int_float() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(- 1.0 2.0)", env).unwrap();
        assert_eq!(result, Object::Float(-1.0));
    }

    #[test]
    fn test_sub_float_int() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(- 2.0 2)", env).unwrap();
        assert_eq!(result, Object::Float(0.0))
    }

    #[test]
    fn test_str_whitespace_add() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result =
            eval("(+ \"hello \" \"world\")", env).unwrap();
        assert_eq!(
            result,
            Object::String("hello world".to_string())
        );
    }

    #[test]
    fn test_int_eq_false() {
        let env = Rc::new(RefCell::new(Env::new()));

        let reust = eval("(= 1 2)", env).unwrap();
        assert_eq!(reust, Object::Bool(false));
    }

    #[test]
    fn test_int_eq_true() {
        let env = Rc::new(RefCell::new(Env::new()));

        let reust = eval("(= 100 100)", env).unwrap();
        assert_eq!(reust, Object::Bool(true));
    }

    #[test]
    fn test_str_eq_false() {
        let env = Rc::new(RefCell::new(Env::new()));

        let reust =
            eval("(= \"hello\" \"world\")", env).unwrap();
        assert_eq!(reust, Object::Bool(false));
    }

    #[test]
    fn test_str_eq_true() {
        let env = Rc::new(RefCell::new(Env::new()));
        let reust =
            eval("(= \"hello\" \"hello\")", env).unwrap();
        assert_eq!(reust, Object::Bool(true));
    }

    #[test]
    fn test_greater_than_str() {
        let env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(> \"B\" \"A\")", env).unwrap();
        assert_eq!(result, Object::Bool(true));
    }

    #[test]
    fn test_greater_than_int() {
        let env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(> 2 3)", env);
        assert_eq!(result, Ok(Object::Bool(false)));
    }

    #[test]
    fn test_less_than_str() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(< \"abcd\" \"abcf\")", env).unwrap();
        assert_eq!(result, Object::Bool(true));
    }

    #[test]
    fn test_less_than_int() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(< 2 1)", env).unwrap();
        assert_eq!(result, Object::Bool(false));
    }

    #[test]
    fn test_mutiply_int_int() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(* 2 3)", env).unwrap();
        assert_eq!(result, Object::Integer(6));
    }

    #[test]
    fn test_mutiply_int_float() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(* 3 2.0)", env).unwrap();
        assert_eq!(result, Object::Float(6.0));
    }

    #[test]
    fn test_mutiply_float_int() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(* 2.0 3)", env).unwrap();
        assert_eq!(result, Object::Float(6.0));
    }

    #[test]
    fn test_mutiply_float_float() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(* 2.0 3.0)", env).unwrap();
        assert_eq!(result, Object::Float(6.0));
    }

    #[test]
    fn test_divide_int_int() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(/ 6 2)", env).unwrap();
        assert_eq!(result, Object::Integer(3));
    }

    #[test]
    fn test_divide_int_float() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(/ 6 2.0)", env).unwrap();
        assert_eq!(result, Object::Float(3.0));
    }

    #[test]
    fn test_divide_float_int() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(/ 6.0 2)", env).unwrap();
        assert_eq!(result, Object::Float(3.0));
    }

    #[test]
    fn test_divide_float_float() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(/ 6.0 2.0)", env).unwrap();
        assert_eq!(result, Object::Float(3.0));
    }

    #[test]
    fn test_modulo_int_int() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(% 7 2)", env).unwrap();
        assert_eq!(result, Object::Integer(1));
    }

    #[test]
    fn test_modulo_int_float() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(% 7 2.0)", env).unwrap();
        assert_eq!(result, Object::Float(1.0));
    }

    #[test]
    fn test_modulo_float_int() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(% 7.0 2)", env).unwrap();
        assert_eq!(result, Object::Float(1.0));
    }

    #[test]
    fn test_modulo_float_float() {
        let env = Rc::new(RefCell::new(Env::new()));

        let result = eval("(% 7.0 2.0)", env).unwrap();
        assert_eq!(result, Object::Float(1.0));
    }

    #[test]
    fn test_area_of_a_circle() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
                (begin
                    (define r 5.0)
                    (define pi 3.14)
                    (* pi (* r r))
                )";
        let result = eval(program, env).unwrap();
        assert_eq!(
            result,
            Object::Float((3.14 * 5.0 * 5.0) as f64)
        );
    }

    #[test]
    fn test_str_whitespaces_add() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
            (
                (define fruits \"apples mangoes bananas \")
                (define vegetables \"carrots broccoli\")
                (+ fruits vegetables)
            )
            ";
        let result = eval(program, env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::String(
                "apples mangoes bananas carrots broccoli"
                    .to_string()
            )])
        );
    }

    
}
