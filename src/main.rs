mod parse;
mod sub;

use std::fmt::{self, Display};
use std::io::{self, Write};
use sub::Subst;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expr {
    App(Box<Expr>, Box<Expr>),
    Lam(String, Box<Expr>),
    Let(String, Box<Expr>, Box<Expr>),
    Var(String),
}

//fn substitute(x: &String, e: Box<Expr>, s: &Box<Expr>) -> Box<Expr> {
//    match *e {
//        App(e1, e2) => Box::new(App(substitute(x, e1, s), substitute(x, e2, s))),
//        Lam(y, e) if y != *x => Box::new(Lam(y, substitute(x, e, s))),
//        Let(y, e1, e2) if y != *x => Box::new(Let(y, substitute(x, e1, s), substitute(x, e2, s))),
//        Var(y) if y == *x => s.clone(),
//        _ => e,
//    }
//}

//fn reduce(e: Expr) -> Option<Expr> {
//    match e {
//        App(e1, e2) => {
//            if let Lam(x, e) = *e1 {
//            } else {
//                None
//            }
//        }
//        Lam(x, e) => {}
//        Let(x, e1, e2) => {
//            if let Some(e1) = reduce(*e1) {
//                Let(x, e1, e2)
//            } else {
//                Some(substitute(x, *e1, *e2))
//            }
//        }
//        Var(x) => None,
//    }
//}

//fn eval(e: Expr, env: Sub<Box<Expr>>) -> Option<Expr> {
//    match e {
//        App(e1, e2) => {
//            if let Lam(x, e) = *e1 {
//                let s = env.insert();
//            }
//            else
//            {
//                None
//            }
//        },
//        Lam(x, e) => {
//        },
//        Let(x, e1, e2) => {
//        },
//        Var(x) => x,
//    }
//}

fn contain(x: &str, e: &Expr) -> bool {
    match e {
        App(e1, e2) => contain(x, e1) || contain(x, e2),
        Lam(y, e) if y != x => contain(x, e),
        Let(y, e1, e2) if y != x => contain(x, e1) || contain(x, e2),
        Var(y) if y == x => true,
        _ => false,
    }
}

fn reduce(e: Expr, s: &Subst<Expr>) -> Option<Expr> {
    match e {
        App(e1, e2) => {
            if let Some(e2) = reduce(*e2.clone(), &s) {
                return Some(App(e1, Box::new(e2)));
            }
            if let Some(e1) = reduce(*e1.clone(), &s) {
                return Some(App(Box::new(e1), e2));
            }
            if let Lam(x, e1) = *e1 {
                let s = s.extend(x, *e2);
                Some(s.apply(*e1))
            } else {
                None
            }
        }
        Lam(x, e) => {
            if let Some(e) = reduce(*e.clone(), &s) {
                return Some(Lam(x, Box::new(e)));
            }

            if let App(e, y) = *e {
                if let Var(y) = *y {
                    if *x == y && !contain(&x, &e) {
                        return Some(*e);
                    }
                }
            }
            None
        }
        Let(x, e1, e2) => {
            if let Some(e1) = reduce(*e1.clone(), &s) {
                return Some(Let(x, Box::new(e1), e2.clone()));
            }

            if let Some(e2) = reduce(*e2.clone(), &s) {
                return Some(Let(x, e1.clone(), Box::new(e2)));
            }

            let s = s.extend(x, *e1);
            Some(s.apply(*e2))
        }
        Var(x) => {
            let e = s.apply(Var(x.clone()));
            match e {
                Var(y) if x == y => None,
                _ => Some(e),
            }
        }
    }
}

//fn app(e1: Expr, e2: Expr) -> Expr {
//    App(Box::new(e1), Box::new(e2))
//}
//
//fn lam(x: &str, e: Expr) -> Expr {
//    Lam(x.to_owned(), Box::new(e))
//}
//
//fn let_(x: &str, e1: Expr, e2: Expr) -> Expr {
//    Let(x.to_owned(), Box::new(e1), Box::new(e2))
//}
//
//fn var(x: &str) -> Expr {
//    Var(x.to_owned())
//}

use Expr::*;

fn is_var(e: &Expr) -> bool {
    match e {
        Var(_) => true,
        _ => false,
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            App(e1, e2) => {
                let v1 = is_var(e1);
                let v2 = is_var(e2);
                write!(
                    f,
                    "{}{}{} {}{}{}",
                    if !v1 { "(" } else { "" },
                    e1,
                    if !v1 { ")" } else { "" },
                    if !v2 { "(" } else { "" },
                    e2,
                    if !v2 { ")" } else { "" }
                )
            }
            Lam(x, e) => write!(f, "λ{}.{}", x, e),
            Let(x, e1, e2) => write!(f, "let {} = {} in {}", x, e1, e2),
            Var(x) => write!(f, "{}", x),
        }
    }
}

fn main() -> Result<(), io::Error> {
    let mut line = String::new();
    loop {
        print!(">> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut line)?;

        match parse::parse(&line) {
            None => {
                println!("");
                return Ok(());
            }
            Some(Ok(mut e)) => {
                let s = Subst::new();
                println!("{}", e);
                while let Some(t) = reduce(e.clone(), &s) {
                    if t == e {
                        break;
                    }
                    e = t;
                    println!("{}", e)
                }
            }
            Some(Err(e)) => eprintln!("{}", e),
        }

        line.clear();
    }
}
