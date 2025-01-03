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
            let body = list[1].clone();
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
        Object::List(l) => l.clone(),
        _ => return Err(format!("Invalid lambda")),
    };

    Ok(Object::Lambda(params, body, env.clone()))
}

fn eval_list(
    list: &[Object],
    env: Rc<RefCell<Env>>,
) -> Result<Object, String> {
    let mut list_data = vec![];

    for obj in list {
        list_data.push(eval_obj(obj, env.clone())?);
    }
    Ok(Object::ListData(list_data))
}

fn eval_car(
    list: &[Object],
    env: Rc<RefCell<Env>>,
) -> Result<Object, String> {
    if list.len() != 1 {
        return Err(format!(
            "Invalid number of arguments for car"
        ));
    }

    let obj = eval_obj(&list[0], env.clone())?;

    match obj {
        Object::ListData(list) => Ok(list[0].clone()),
        _ => {
            Err(format!("Invalid type car argument {}", list[0]))
        }
    }
}

fn eval_cdr(
    list: &[Object],
    env: Rc<RefCell<Env>>,
) -> Result<Object, String> {
    if list.len() != 1 {
        return Err(format!(
            "Invalid number of arguments for cdr"
        ));
    }

    let obj = eval_obj(&list[0], env.clone())?;

    match obj {
        Object::ListData(list) => {
            if list.len() >= 1 {
                Ok(Object::ListData(list[1..].to_vec()))
            } else {
                Err(format!("Invalid number of list data"))
            }
        }
        _ => {
            Err(format!("Invalid type cdr argument {}", list[0]))
        }
    }
}

fn eval_length(
    list: &[Object],
    env: Rc<RefCell<Env>>,
) -> Result<Object, String> {
    if list.len() != 1 {
        return Err(format!(
            "Invalid number of arguments for length"
        ));
    }

    let obj = eval_obj(&list[0], env.clone())?;

    match obj {
        Object::ListData(list) => {
            Ok(Object::Integer(list.len() as i64))
        }
        _ => Err(format!(
            "Invalid type length argument {}",
            list[0]
        )),
    }
}

fn eval_is_null(
    list: &[Object],
    env: Rc<RefCell<Env>>,
) -> Result<Object, String> {
    if list.len() != 1 {
        return Err(format!(
            "Invalid number of arguments for is_null"
        ));
    }

    let obj = eval_obj(&list[0], env.clone())?;

    match obj {
        Object::ListData(list) => {
            if list.is_empty() {
                Ok(Object::Bool(true))
            } else {
                Ok(Object::Bool(false))
            }
        }
        _ => Err(format!(
            "Invalid type null? argument {}",
            list[0]
        )),
    }
}

fn eval_cond(
    list: &[Object],
    env: Rc<RefCell<Env>>,
) -> Result<Object, String> {
    for obj in list {
        match obj {
            Object::List(list) => {
                if list.len() != 2 {
                    return Err(format!(
                        "Invalid number of arguments for cond"
                    ));
                }

                if list[0] == Object::Keyword("else".to_string())
                {
                    return eval_obj(&list[1], env.clone());
                }
                let cond_obj = eval_obj(&list[0], env.clone())?;
                let cond = match cond_obj {
                    Object::Bool(cond) => cond,
                    _ => {
                        return Err(format!(
                            "Invalid type cond argument {}",
                            cond_obj
                        ))
                    }
                };
                if cond {
                    return eval_obj(&list[1], env.clone());
                }
            }
            _ => {
                return Err(format!(
                    "Invalid type cond argument {}",
                    obj
                ))
            }
        }
    }

    Err("No cond clause matched".to_string())
}

fn eval_let(
    list: &[Object],
    env: Rc<RefCell<Env>>,
) -> Result<Object, String> {
    if list.len() != 2 {
        return Err(format!(
            "Invalid number of arguments for let"
        ));
    }

    let new_env =
        Rc::new(RefCell::new(Env::extend(env.clone())));
    let bindings = match &list[0] {
        Object::List(list) => list.to_vec(),
        _ => return Err(format!("Invalid let")),
    };

    for obj in bindings {
        match obj {
            Object::List(list) => {
                if list.len() != 2 {
                    return Err(format!(
                        "Invalid number of arguments for let"
                    ));
                }

                let name = match &list[0] {
                    Object::Symbol(name) => name.clone(),
                    _ => {
                        return Err(format!(
                            "Invalid let argument {}",
                            list[0]
                        ))
                    }
                };

                let value = eval_obj(&list[1], env.clone())?;
                new_env.borrow_mut().set(name, value);
            }
            _ => {
                return Err(format!(
                    "Invalid let argument {}",
                    obj
                ))
            }
        }
    }

    return eval_obj(&list[1], new_env);
}

fn eval_cons(
    list: &[Object],
    env: Rc<RefCell<Env>>,
) -> Result<Object, String> {
    if list.len() != 2 {
        return Err(format!(
            "Invalid number of arguments for cons"
        ));
    }

    let head = eval_obj(&list[0], env.clone())?;
    let tail = eval_obj(&list[1], env.clone())?;

    //  合并listdata
    match tail {
        Object::ListData(mut l) => {
            l.insert(0, head);
            Ok(Object::ListData(l))
        }
        _ => Err(format!("Invalid type cons argument {}", tail)),
    }
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
        "list" => eval_list(list, env.clone()),
        "car" => eval_car(list, env.clone()),
        "cdr" => eval_cdr(list, env.clone()),
        "length" => eval_length(list, env.clone()),
        "null?" => eval_is_null(list, env.clone()),
        "cond" => eval_cond(list, env.clone()),
        "let" => eval_let(list, env.clone()),
        "cons" => eval_cons(list, env.clone()),
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
                        //  放在这里，进行尾递归优化
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
                    Object::Lambda(params, body, func_env) => {
                        let new_env = Rc::new(RefCell::new(
                            Env::extend(func_env.clone()),
                        ));

                        for (param, arg) in
                            params.iter().zip(list[1..].iter())
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
                            Box::new(Object::List(body.clone()));
                        current_env = new_env;
                    }
                    _ => {
                        let mut new_list = vec![];
                        for obj in list {
                            let result = eval_obj(
                                &obj,
                                current_env.clone(),
                            )?;

                            if result != Object::Void {
                                new_list.push(result);
                            }
                        }

                        let head = &new_list[0];
                        match head {
                            Object::Lambda(_, _, _) => {
                                current_obj = Box::new(
                                    Object::List(new_list),
                                );
                            }
                            _ => {
                                return Ok(Object::List(
                                    new_list,
                                ))
                            }
                        }
                    }
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

    #[test]
    fn test_closure1() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
              (begin
                  (define add-n 
                     (lambda (n) 
                        (lambda (a) (+ n a))))
                  (define add-5 (add-n 5))
                  (add-5 10)
              )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(result, Object::Integer((15) as i64));
    }

    #[test]
    fn test_tail_recursive_fibonnaci() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
              (begin
                  (define fib
                    (lambda (n a b) 
                       (if (= n 0) a 
                         (if (= n 1) b 
                            (fib (- n 1) b (+ a b))))))
                    
                  (fib 10 0 1)
              )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(result, Object::Integer((55) as i64));
    }

    #[test]
    fn test_inline_lambda() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
          (begin
              ((lambda (x y) (+ x y)) 10 20)
          )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(result, Object::Integer((30) as i64));
    }

    #[test]
    fn test_car() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
          (begin
              (car (list 1 2 3))
          )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(result, Object::Integer((1) as i64));
    }

    #[test]
    fn test_cdr() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
          (begin
              (cdr (list 1 2 3))
          )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(
            result,
            Object::ListData(vec![
                Object::Integer(2),
                Object::Integer(3),
            ])
        );
    }

    #[test]
    fn test_length() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
          (begin
              (length (list 1 2 3))
          )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(result, Object::Integer((3) as i64));
    }

    #[test]
    fn test_sum_list_of_integers() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
          (begin
              (define sum-list 
                  (lambda (l) 
                      (if (null? l) 0 
                          (+ (car l) (sum-list (cdr l))))))
              (sum-list (list 1 2 3 4 5))
          )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(result, Object::Integer(15));
    }

    #[test]
    fn test_function_application() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
          (begin
              (define (double value) 
                  (* 2 value))
              (define (apply-twice fn value) 
                  (fn (fn value)))
          
              (apply-twice double 5)
          )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(result, Object::Integer((20) as i64));
    }

    #[test]
    fn test_begin_scope_test() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
          (begin
              (define a 10)
              (define b 20)
              (define c 30)
              (begin
                  (define a 20)
                  (define b 30)
                  (define c 40)
                  (list a b c)
              )
          )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(
            result,
            Object::ListData(vec![
                Object::Integer(20),
                Object::Integer(30),
                Object::Integer(40),
            ])
        );
    }

    #[test]
    fn test_begin_scope_test_2() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
          (begin 
              (define x 10)
              (begin
                  (define x 20)
                  x 
              )
              x
          )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(result, Object::Integer((10) as i64));
    }

    #[test]
    fn test_cond_1() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
              (cond ((> 2 1) 5) 
                    ((< 2 1) 10) 
                    (else 15)
              )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(result, Object::Integer(5));
    }

    #[test]
    fn test_cond_2() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
              (cond ((> 1 2) 5) 
                    ((< 1 2) 10) 
                    (else 15)
          )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(result, Object::Integer(10));
    }

    #[test]
    fn test_cond_3() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
              (cond ((> 1 2) 5) 
                    ((< 1 0) 10) 
                    (else 15)
              )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(result, Object::Integer(15));
    }

    #[test]
    fn test_let_1() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
          (begin
              (let ((a 10) (b 20))
                  (list a b)
              )
          )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(
            result,
            Object::ListData(vec![
                Object::Integer(10),
                Object::Integer(20),
            ])
        );
    }

    #[test]
    fn test_let_2() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
          (begin
              (define a 100)
              (let ((a 10) (b 20))
                  (list a b)
              )
              a
          )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(result, Object::Integer(100));
    }

    #[test]
    fn test_let_3() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
              (let ((x 2) (y 3))
                  (let ((x 7)
                        (z (+ x y)))
                      (* z x))) 
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(result, Object::Integer(35));
    }

    #[test]
    fn test_map() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
          (begin
              (define (map f l)
                  (if (null? l) 
                      (list) 
                      (cons (f (car l)) (map f (cdr l)))))
              (map (lambda (x) (* x x)) (list 1 2 3 4 5))
          )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(
            result,
            Object::ListData(vec![
                Object::Integer(1),
                Object::Integer(4),
                Object::Integer(9),
                Object::Integer(16),
                Object::Integer(25),
            ])
        );
    }

    #[test]
    fn test_filter() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
          (begin
              (define (filter f l)
                  (if (null? l) 
                      (list) 
                      (if (f (car l)) 
                          (cons (car l) (filter f (cdr l))) 
                          (filter f (cdr l)))))
              (filter (lambda (x) (> x 2)) (list 1 2 3 4 5))
          )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(
            result,
            Object::ListData(vec![
                Object::Integer(3),
                Object::Integer(4),
                Object::Integer(5),
            ])
        );
    }

    #[test]
    fn test_fold_left() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
          (begin
              (define (fold-left f acc l)
                  (if (null? l) 
                      acc 
                      (fold-left f (f acc (car l)) (cdr l))))
              (fold-left (lambda (acc x) (+ acc x)) 0 (list 1 2 3 4 5))
          )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(result, Object::Integer(15));
    }

    #[test]
    fn test_reduce() {
        let env = Rc::new(RefCell::new(Env::new()));
        let program = "
          (begin
              (define (reduce f l)
                  (if (null? l) 
                      (list) 
                      (if (null? (cdr l)) 
                          (car l) 
                          (f (car l) (reduce f (cdr l))))))
              (reduce (lambda (x y) (+ x y)) (list 1 2 3 4 5))
          )
          ";

        let result = eval(program, env).unwrap();
        assert_eq!(result, Object::Integer(15));
    }
}
