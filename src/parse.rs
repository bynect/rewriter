use crate::Expr;
use std::iter::Peekable;

#[derive(Debug, Eq, PartialEq)]
enum Token {
    ParL,
    ParR,
    In,
    Eq,
    Let,
    Lam,
    Dot,
    Eof,
    Var(String),
}

type Lexer<'a, I> = (Peekable<I>, Option<Token>);

fn backtrack<I: Iterator<Item = char>>(lex: &mut Lexer<I>, t: Token) {
    debug_assert!(lex.1.is_none());
    lex.1 = Some(t);
}

fn token<I: Iterator<Item = char>>(lex: &mut Lexer<I>) -> Result<Token, String> {
    if let Some(t) = lex.1.take() {
        return Ok(t);
    }

    let it = &mut lex.0;
    while let Some(&c) = it.peek() {
        match c {
            '(' => {
                it.next();
                return Ok(Token::ParL);
            }
            ')' => {
                it.next();
                return Ok(Token::ParR);
            }
            '\\' | 'λ' => {
                it.next();
                return Ok(Token::Lam);
            }
            '.' => {
                it.next();
                return Ok(Token::Dot);
            }
            '=' => {
                it.next();
                return Ok(Token::Eq);
            }
            'a'..='z' | 'A'..='Z' => {
                let mut buf = String::new();
                while let Some(&c) = it.peek() {
                    match c {
                        'a'..='z' | 'A'..='Z' | '_' => {
                            buf.push(c);
                            it.next();
                        }
                        _ => break,
                    }
                }

                while let Some(&c) = it.peek() {
                    if c == '\'' {
                        buf.push(c);
                        it.next();
                    } else {
                        break;
                    }
                }

                return Ok(if buf == "let" {
                    Token::Let
                } else if buf == "in" {
                    Token::In
                } else {
                    Token::Var(buf)
                });
            }
            '-' => {
                if let Some('-') = it.next() {
                    while let Some(&c) = it.peek() {
                        if c == '\n' {
                            break;
                        } else {
                            it.next();
                        }
                    }
                }
            }
            ' ' | '\t' | '\n' => {
                it.next();
            }
            c => Err(format!("Unexpected char {:?}", c))?,
        };
    }
    Ok(Token::Eof)
}

fn expect<I: Iterator<Item = char>>(lex: &mut Lexer<I>, e: Token) -> Result<Token, String> {
    let t = token(lex)?;
    if t != e {
        Err(format!("Expected {:?} but got {:?}", e, t))
    } else {
        Ok(t)
    }
}

fn parse_expr<I: Iterator<Item = char>>(lex: &mut Lexer<I>) -> Result<Expr, String> {
    let mut e1 = match token(lex)? {
        Token::ParL => {
            let e = parse_expr(lex)?;
            expect(lex, Token::ParR)?;
            Ok(e)
        }
        Token::Lam => {
            let t = token(lex)?;
            if let Token::Var(x) = t {
                expect(lex, Token::Dot)?;
                let e = parse_expr(lex)?;
                Ok(Expr::Lam(x, Box::new(e)))
            } else {
                Err(format!("Expected Var but got {:?}", t))
            }
        }
        Token::Let => {
            let t = token(lex)?;
            if let Token::Var(x) = t {
                expect(lex, Token::Eq)?;
                let e1 = parse_expr(lex)?;

                expect(lex, Token::In)?;
                let e2 = parse_expr(lex)?;
                Ok(Expr::Let(x, Box::new(e1), Box::new(e2)))
            } else {
                Err(format!("Expected Var but got {:?}", t))
            }
        }
        Token::Var(x) => Ok(Expr::Var(x)),
        t => Err(format!("Unexpected token {:?}", t)),
    }?;

    while let Some(e2) = parse_base(lex) {
        e1 = Expr::App(Box::new(e1), Box::new(e2?));
    }
    Ok(e1)
}

// Result<T, E> -> T | Option<Result<T, E>>
macro_rules! trans {
    ($e:expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => return Some(Err(e)),
        }
    };
}

fn parse_base<I: Iterator<Item = char>>(lex: &mut Lexer<I>) -> Option<Result<Expr, String>> {
    Some(match trans!(token(lex)) {
        Token::ParL => {
            let e = trans!(parse_expr(lex));
            trans!(expect(lex, Token::ParR));
            Ok(e)
        }
        Token::Var(x) => Ok(Expr::Var(x)),
        t @ (Token::Dot | Token::Lam) => Err(format!("Unexpected token {:?}", t)),
        t => {
            backtrack(lex, t);
            return None;
        }
    })
}

pub fn parse(s: &str) -> Option<Result<Expr, String>> {
    let mut lex = (s.chars().peekable(), None);
    match trans!(token(&mut lex)) {
        Token::Eof => None,
        t => {
            backtrack(&mut lex, t);
            let e = parse_expr(&mut lex);
            if let Ok(Token::Eof) = token(&mut lex) {
                Some(e)
            } else {
                Some(Err(String::from("Parsing error")))
            }
        }
    }
}
