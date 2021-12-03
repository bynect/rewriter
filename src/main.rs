mod expr;
mod parse;
mod split;
mod sub;

use expr::Expr;
use split::split_n;
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

    let mut subst = Subst::new().extend(String::from("Y"), y);
    let mut limit = 100usize;

    let mut line = String::new();
    loop {
        print!(">> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut line)?;

        // Directives or expressions
        if let Some(line) = line.strip_prefix(':') {
            //if let Some(d) = line.trim().strip_prefix("limit") {
            //    if d.is_empty() {
            //        println!("{}", limit);
            //    } else {
            //        match d.parse::<usize>() {
            //            Ok(n) => {
            //                limit = n;
            //            }
            //            Err(e) => eprintln!("{}", e),
            //        }
            //    }
            //} else {
            //    eprintln!("Unrecognized directive {}", d);
            //}

            let buf = line.split_ascii_whitespace().collect::<Vec<&str>>();
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
                    "let" => {
                        if buf.len() >= 2 {
                            let i = buf[2].start;
                            match parse::parse(&line[i..]) {
                                Some(Ok(e)) => {
                                    subst = subst.extend(buf[1].to_string(), e);
                                }
                                Some(Err(e)) => eprintln!("{}", e),
                                None => eprintln!("Expected expression"),
                            };
                        } else {
                            eprintln!("Expected 2 arguments to directive let")
                        }
                    }
                    _ => eprintln!("Unrecognized directive {}", buf[0]),
                }
            }
        } else {
            match parse::parse(&line) {
                Some(Ok(mut e)) => {
                    println!("{}", e);
                    let mut i = 0;
                    while let Some(t) = expr::reduce(e.clone(), &subst) {
                        if t == e || i >= limit {
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
