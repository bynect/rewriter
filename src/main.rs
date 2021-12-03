mod cmd;
mod expr;
mod parse;
mod split;
mod subst;

use cmd::Command;
use expr::Expr;
use std::io::{self, Write};
use subst::Subst;

pub struct Config {
    limit: usize,
    subst: Subst<Expr>,
}

// Commands (prefixed by :) or expressions
fn interpret(cmds: &[Command], cfg: &mut Config, line: &str) -> bool {
    if let Some(line) = line.strip_prefix(':') {
        cmd::command(cmds, cfg, line);
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
                return true;
            }
        }
    }
    false
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

    let cmds: [cmd::Command; 3] = [cmd::LIMIT_COMMAND, cmd::LET_COMMAND, cmd::FILE_COMMAND];

    let mut line = String::new();
    loop {
        print!(">> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut line)?;

        if interpret(&cmds, &mut cfg, &line) {
            return Ok(());
        }
        line.clear();
    }
}
