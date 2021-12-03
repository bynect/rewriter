mod expr;
mod parse;
mod sub;

use expr::Expr;
use std::io::{self, Write};
use sub::Subst;

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
    let s = Subst::new().extend(String::from("Y"), y);

    let mut limit = 1000usize;
    let mut line = String::new();
    loop {
        print!(">> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut line)?;

        if let Some(d) = line.trim().strip_prefix(':') {
            if let Some(d) = d.strip_prefix("limit") {
                if let Ok(l) = d.trim_start().parse::<usize>() {
                    limit = l;
                } else {
                    println!("{}", limit);
                }
            } else {
                eprintln!("Unrecognized directive {}", d)
            }
        } else {
            match parse::parse(&line) {
                None => {
                    println!("");
                    return Ok(());
                }
                Some(Ok(mut e)) => {
                    println!("{}", e);
                    let mut i = 0;
                    while let Some(t) = expr::reduce(e.clone(), &s) {
                        if t == e || i >= limit {
                            break;
                        }

                        e = t;
                        i += 1;
                        println!("{}", e)
                    }
                }
                Some(Err(e)) => eprintln!("{}", e),
            }
        }

        line.clear();
    }
}
