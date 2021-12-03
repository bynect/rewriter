mod cmd;
mod expr;
mod parse;
mod split;
mod subst;

use cmd::Command;
use expr::Expr;
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;
use subst::Subst;

pub struct Config {
    limit: usize,
    subst: Subst<Expr>,
    echo: bool,
    file: Option<PathBuf>,
    bind: HashMap<String, Expr>,
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
                if cfg.file.is_some() && cfg.echo {
                    println!("")
                }
                // FIXME: Quit on CTRL+D
                //return true;
            }
        }
    }
    false
}

fn main() -> Result<(), io::Error> {
    let mut cfg = Config {
        subst: Subst::new(),
        limit: 100usize,
        echo: false,
        file: None,
        bind: HashMap::new(),
    };

    let cmds = [
        cmd::BIND_COMMAND,
        cmd::FILE_COMMAND,
        cmd::HELP_COMMAND,
        cmd::LIMIT_COMMAND,
        cmd::SHOW_COMMAND,
    ];

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
