use crate::lexer::*;
use crate::object::*;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ParseError {
    err: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: {}", self.err)
    }
}

impl Error for ParseError {}

pub fn parse(input: &str) -> Result<Object, ParseError> {
    let tokens_result = tokenize(input);
    if tokens_result.is_err() {
        return Err(ParseError {
            err: format!("Tokenize error"),
        });
    }

    let mut tokens = tokens_result
        .unwrap()
        .into_iter()
        .rev()
        .collect::<Vec<_>>();
    let parse_list = parse_list(&mut tokens)?;
    Ok(parse_list)
}
pub fn parse_list(
    tokens: &mut Vec<Token>,
) -> Result<Object, ParseError> {
    let token = tokens.pop();
    if token != Some(Token::LParen) {
        return Err(ParseError {
            err: format!("Expected LParen, found {:?}", token),
        });
    }

    let mut list = vec![];
    while !tokens.is_empty() {
        let token = tokens.pop();
        if token.is_none() {
            return Err(ParseError {
                err: format!("Did not find enough tokens"),
            });
        }

        let t = token.unwrap();
        match t {
            Token::Keyword(k) => list.push(Object::Keyword(k)),
            Token::BinaryOp(s) => list.push(Object::BinaryOp(s)),
            Token::Integer(i) => list.push(Object::Integer(i)),
            Token::Float(f) => list.push(Object::Float(f)),
            Token::String(s) => list.push(Object::String(s)),
            Token::Symbol(s) => list.push(Object::Symbol(s)),
            Token::LParen => {
                tokens.push(Token::LParen);
                let sub_list = parse_list(tokens)?;
                list.push(sub_list);
            }
            Token::RParen => {
                return Ok(Object::List(list));
            }
        }
    }
    Ok(Object::List(list))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let list = parse("(+ 1 2)").unwrap();
        assert_eq!(
            list,
            Object::List(vec![
                Object::BinaryOp("+".to_string()),
                Object::Integer(1),
                Object::Integer(2)
            ])
        )
    }

    #[test]
    fn test_area_of_a_circle() {
        let program = "(
                           (define r 10)
                           (define pi 314)
                           (* pi (* r r))
                         )";
        let list = parse(program).unwrap();
        assert_eq!(
            list,
            Object::List(vec![
                Object::List(vec![
                    Object::Keyword("define".to_string()),
                    Object::Symbol("r".to_string()),
                    Object::Integer(10),
                ]),
                Object::List(vec![
                    Object::Keyword("define".to_string()),
                    Object::Symbol("pi".to_string()),
                    Object::Integer(314),
                ]),
                Object::List(vec![
                    Object::BinaryOp("*".to_string()),
                    Object::Symbol("pi".to_string()),
                    Object::List(vec![
                        Object::BinaryOp("*".to_string()),
                        Object::Symbol("r".to_string()),
                        Object::Symbol("r".to_string()),
                    ]),
                ]),
            ])
        );
    }
}
