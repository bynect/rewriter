mod dir;
mod expr;
mod parse;
mod split;
mod sub;

use expr::Expr;
use std::io::{self, Write};
use sub::Subst;

pub struct Config {
    limit: usize,
    subst: Subst<Expr>,
}

fn main() -> Result<(), io::Error> {
    let y = Expr::lam(
        "f",
        Expr::app(
            Expr::lam(
                "x",
                Expr::app(Expr::var("f"), Expr::app(Expr::var("x"), Expr::var("x"))),
            ),
            Expr::lam(
                "x",
                Expr::app(Expr::var("f"), Expr::app(Expr::var("x"), Expr::var("x"))),
            ),
        ),
    );

    let mut cfg = Config {
        subst: Subst::new().extend(String::from("Y"), y),
        limit: 100usize,
    };

    let dirs = [dir::LIMIT_DIRECTIVE, dir::LET_DIRECTIVE];

    let mut line = String::new();
    loop {
        print!(">> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut line)?;

        // Directives (prefixed by :) or expressions
        if let Some(line) = line.strip_prefix(':') {
            dir::directive(&dirs, &mut cfg, line);
        } else {
            match parse::parse(&line) {
                Some(Ok(mut e)) => {
                    println!("{}", e);
                    let mut i = 0;
                    while let Some(t) = expr::reduce(e.clone(), &cfg.subst) {
                        if t == e || i >= cfg.limit {
                            break;
                        }

                        e = t;
                        i += 1;
                        println!("{}", e)
                    }
                }
                Some(Err(e)) => eprintln!("{}", e),
                None => {
                    // Quit on CTRL+D
                    println!("");
                    return Ok(());
                }
            }
        }

        line.clear();
    }
}
