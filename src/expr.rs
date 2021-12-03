use crate::Subst;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expr {
    App(Box<Expr>, Box<Expr>),
    Lam(String, Box<Expr>),
    Let(String, Box<Expr>, Box<Expr>),
    Var(String),
}

use Expr::*;

impl Expr {
    pub fn app(e1: Expr, e2: Expr) -> Expr {
        App(Box::new(e1), Box::new(e2))
    }

    pub fn lam(x: &str, e: Expr) -> Expr {
        Lam(x.to_owned(), Box::new(e))
    }

    pub fn let_(x: &str, e1: Expr, e2: Expr) -> Expr {
        Let(x.to_owned(), Box::new(e1), Box::new(e2))
    }

    pub fn var(x: &str) -> Expr {
        Var(x.to_owned())
    }

    pub fn is_var(&self) -> bool {
        match *self {
            Var(_) => true,
            _ => false,
        }
    }

    // x is a free variable in the expression
    //
    // let x = \x.x in x y
    // y is free, x is not
    //
    // \x.x y
    // y is free
    //
    // \x.\y.z
    // z is free
    pub fn freevar(&self, x: &str) -> bool {
        match self {
            App(e1, e2) => e1.freevar(x) || e2.freevar(x),
            Lam(y, e) if y != x => e.freevar(x),
            Let(y, e1, e2) if y != x => e1.freevar(x) || e2.freevar(x),
            Var(y) if y == x => true,
            _ => false,
        }
    }
}

pub fn reduce(e: Expr, s: &Subst<Expr>) -> Option<Expr> {
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
            // FIXME: Eta conversion should be performed separately
            //if let Some(e) = reduce(*e.clone(), &s) {
            //    return Some(Lam(x, Box::new(e)));
            //}

            //if let App(e, y) = *e {
            //    if let Var(y) = *y {
            //        if *x == y && !e.freevar(&x) {
            //            return Some(*e);
            //        }
            //    }
            //}
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

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            App(e1, e2) => {
                let v1 = e1.is_var();
                let v2 = e2.is_var();
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
