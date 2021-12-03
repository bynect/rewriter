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

    let mut limit = 100usize;
    let mut line = String::new();

    loop {
        print!(">> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut line)?;

        // Directives or expressions
        if let Some(d) = line.strip_prefix(':') {
            let buf = d.split_ascii_whitespace().collect::<Vec<&str>>();
            if buf.is_empty() {
                println!("");
            } else {
                match buf[0] {
                    "limit" => {
                        if buf.len() == 2 {
                            match buf[1].parse::<usize>() {
                                Ok(n) => {
                                    limit = n;
                                }
                                Err(e) => eprintln!("{}", e),
                            }
                        } else if buf.len() == 1 {
                            println!("{}", limit);
                        } else {
                            eprintln!("Expected 1 or 2 arguments to directive limit")
                        }
                    }
                    _ => eprintln!("Unrecognized directive {}", d),
                }
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
