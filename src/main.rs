mod expr;
mod parse;
mod split;
mod sub;

use expr::Expr;
use std::io::{self, Write};
use sub::Subst;

struct Config {
    limit: usize,
    subst: Subst<Expr>,
}

fn directive(cfg: &mut Config, line: &str) {
    let buf = split::split_n_whitespace(line, 2).collect::<Vec<_>>();
    if buf.is_empty() {
        println!("");
    } else {
        match buf[0].slice {
            "limit" => {
                if buf.len() == 2 {
                    let mut it = split::split_n_whitespace(buf[1].slice, 2);
                    if let Some(first) = it.next() {
                        if let Some(rest) = it.next() {
                            eprintln!("Unexpected trailing characters `{}`", rest.slice);
                            return;
                        }

                        match first.slice.parse::<usize>() {
                            Ok(n) => cfg.limit = n,
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    } else {
                        println!("limit = {}", cfg.limit)
                    }
                } else {
                    println!("limit = {}", cfg.limit)
                }
            }
            "let" => {
                if buf.len() == 2 {
                    let mut it = split::split_n_whitespace(buf[1].slice, 2);
                    if let Some(x) = it.next() {
                        if let Some(e) = it.next() {
                            debug_assert!(it.next().is_none());
                            match parse::parse(e.slice) {
                                Some(Ok(e)) => {
                                    cfg.subst = cfg.subst.extend(x.slice.to_string(), e);
                                }
                                Some(Err(e)) => eprintln!("{}", e),
                                None => eprintln!("Expected expression"),
                            };
                        } else {
                            eprintln!("Expected expression")
                        }
                    } else {
                        eprintln!("Expected 2 arguments to directive let")
                    }
                } else {
                    eprintln!("Expected 2 arguments to directive let")
                }
            }
            _ => eprintln!("Unrecognized directive {}", buf[0].slice),
        }
    }
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

    let mut line = String::new();
    loop {
        print!(">> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut line)?;

        // Directives or expressions
        if let Some(line) = line.strip_prefix(':') {
            directive(&mut cfg, line);
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

            //let buf = line.split_ascii_whitespace().collect::<Vec<&str>>();
            //if buf.is_empty() {
            //    println!("");
            //} else {
            //    match buf[0] {
            //        "limit" => {
            //            if buf.len() == 2 {
            //                match buf[1].parse::<usize>() {
            //                    Ok(n) => {
            //                        limit = n;
            //                    }
            //                    Err(e) => eprintln!("{}", e),
            //                }
            //            } else if buf.len() == 1 {
            //                println!("{}", limit);
            //            } else {
            //                eprintln!("Expected 1 or 2 arguments to directive limit")
            //            }
            //        }
            //        "let" => {
            //            if buf.len() >= 2 {
            //                let i = buf[2].start;
            //                match parse::parse(&line[i..]) {
            //                    Some(Ok(e)) => {
            //                        subst = subst.extend(buf[1].to_string(), e);
            //                    }
            //                    Some(Err(e)) => eprintln!("{}", e),
            //                    None => eprintln!("Expected expression"),
            //                };
            //            } else {
            //                eprintln!("Expected 2 arguments to directive let")
            //            }
            //        }
            //        _ => eprintln!("Unrecognized directive {}", buf[0]),
            //    }
            //}
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
