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

    let (name, value) = match &list[0] {
        Object::Symbol(name) => {
            (name.clone(), eval_obj(&list[1], env.clone())?)
        }
        Object::List(l) => {
            let name = match &l[0] {
                Object::Symbol(name) => name.clone(),
                _ => return Err(format!("Invalid define")),
            };

            let params = Object::List(l[1..].to_vec());
            let body = Object::List(vec![list[1].clone()]);
            let value =
                eval_lambda(&[params, body], env.clone())?;

            (name, value)
        }
        _ => return Err(format!("Invalid define")),
    };

    env.borrow_mut().set(name, value);
    Ok(Object::Void)
}

fn eval_lambda(
    list: &[Object],
    env: Rc<RefCell<Env>>,
) -> Result<Object, String> {
    if list.len() != 2 {
        return Err(format!(
            "Invalid number of arguments for lambda"
        ));
    }

    let params = match &list[0] {
        Object::List(list) => {
            let mut params = vec![];
            for param in list {
                match param {
                    Object::Symbol(param) => {
                        params.push(param.clone())
                    }
                    _ => {
                        return Err(format!(
                            "Invalid lambda parameter {:?}",
                            param
                        ))
                    }
                }
            }
            params
        }
        _ => return Err(format!("Invalid lambda")),
    };

    let body = match &list[1] {
        Object::List(list) => list.to_vec(),
        _ => return Err(format!("Invalid lambda")),
    };

    Ok(Object::Lambda(params, body, env.clone()))
}

fn eval_keyword(
    head: &str,
    list: &[Object],
    env: Rc<RefCell<Env>>,
) -> Result<Object, String> {
    match head {
        "begin" => eval_begin(list, env.clone()),
        "define" => eval_define(list, env.clone()),
        "lambda" => eval_lambda(list, env.clone()),
        _ => todo!(),
    }
}

fn eval_symbol(
    name: String,
    env: Rc<RefCell<Env>>,
) -> Result<Object, String> {
    match env.borrow().get(&name) {
        Some(value) => Ok(value),
        None => Err(format!("Undefined symbol {}", name)),
    }
}

fn eval_obj(
    obj: &Object,
    env: Rc<RefCell<Env>>,
) -> Result<Object, String> {
    let mut current_obj = Box::new(obj.clone());
    let mut current_env = env.clone();
    loop {
        match *current_obj {
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
                        if keyword == "if" {
                            //  todo 无else 可能
                            if list.len() != 4 {
                                return Err(format!("Invalid number of arguments for if"));
                            }

                            let cond_obj = eval_obj(
                                &list[1],
                                current_env.clone(),
                            )?;
                            let cond = match cond_obj {
                                Object::Bool(cond) => cond,
                                _ => {
                                    return Err(format!(
                                        "Condition must be bool"
                                    ))
                                }
                            };

                            if cond {
                                current_obj =
                                    Box::new(list[2].clone());
                            } else {
                                current_obj =
                                    Box::new(list[3].clone());
                            }
                            continue;
                        }
                        return eval_keyword(
                            keyword,
                            &list[1..],
                            current_env,
                        );
                    }
                    Object::Symbol(sym) => {
                        let lambda =
                            current_env.borrow().get(sym);

                        if lambda.is_none() {
                            return Err(format!(
                                "Unbound function: {}",
                                sym
                            ));
                        }

                        let func = lambda.unwrap();
                        match func {
                            Object::Lambda(
                                params,
                                body,
                                func_env,
                            ) => {
                                let new_env = Rc::new(
                                    RefCell::new(Env::extend(
                                        func_env.clone(),
                                    )),
                                );

                                //  传入参数
                                for (param, arg) in params
                                    .iter()
                                    .zip(list[1..].iter())
                                {
                                    new_env.borrow_mut().set(
                                        param.clone(),
                                        eval_obj(
                                            arg,
                                            current_env.clone(),
                                        )?,
                                    )
                                }
                                current_obj =
                                    Box::new(Object::List(body));
                                current_env = new_env;
                            }
                            _ => {
                                return Err(format!(
                                    "Not a lambda {} {}",
                                    sym, func
                                ))
                            }
                        }
                    }
                    Object::List(_) => {
                        current_obj = Box::new(head.clone());
                        continue;
                    }
                    _ => todo!(),
                }
            }
            Object::Integer(i) => return Ok(Object::Integer(i)),
            Object::Float(f) => return Ok(Object::Float(f)),
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
    fn test_area_of_a_circle_float() {
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

    #[test]
    fn test_area_of_a_circle_int() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
                (begin
                    (define r 10)
                    (define pi 314)
                    (* pi (* r r))
                )";
        let result = eval(program, env).unwrap();
        assert_eq!(result, Object::Integer(314 * 10 * 10));
    }

    #[test]
    fn test_sqr_function() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
                (begin
                    (define sqr (lambda (r) (* r r))) 
                    (sqr 10)
                )";
        let result = eval(program, env).unwrap();
        assert_eq!(result, Object::Integer((10 * 10) as i64));
    }

    #[test]
    fn test_fibonaci() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
              (begin
                  (define fib (lambda (n) 
                      (if (< n 2) 1 
                          (+ (fib (- n 1)) 
                              (fib (- n 2))))))
                  (fib 10)
              )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(result, Object::Integer((89) as i64));
    }

    #[test]
    fn test_factorial() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
              (begin
                  (define fact (lambda (n) (if (< n 1) 1 (* n (fact (- n 1))))))
                  (fact 5)
              )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(result, Object::Integer((120) as i64));
    }

    #[test]
    fn test_circle_area_no_lambda() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
              (begin
                  (define pi 314)
                  (define r 10)
                  (define (sqr r) (* r r))
                  (define (area r) (* pi (sqr r)))
                  (area r)
              )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(
            result,
            Object::Integer((314 * 10 * 10) as i64)
        );
    }

    #[test]
    fn test_circle_area_function() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
              (begin
                  (define pi 314)
                  (define r 10)
                  (define sqr (lambda (r) (* r r)))
                  (define area (lambda (r) (* pi (sqr r))))
                  (area r)
              )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(
            result,
            Object::Integer((314 * 10 * 10) as i64)
        );
    }

    #[test]
    fn test_tail_recursion() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
              (begin
                  (define sum-n 
                     (lambda (n a) 
                        (if (= n 0) a 
                            (sum-n (- n 1) (+ n a)))))
                  (sum-n 10000 0)
              )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(result, Object::Integer((50_005_000) as i64));
    }

    #[test]
    fn test_tail_recursive_factorial() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
              (begin
                  (define fact 
                      (lambda (n a) 
                        (if (= n 1) a 
                          (fact (- n 1) (* n a)))))
                          
                  (fact 10 1)
              )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(result, Object::Integer((3628800) as i64));
    }
}
