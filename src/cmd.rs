use crate::{parse, split, split::Match, Config};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

pub struct Command<'a> {
    pub name: &'a str,
    pub usage: &'a str,
    pub desc: &'a str,
    pub args: Arg,
    pub fun: fn(&[Command], &mut Config, &str, Option<Match>),
}

#[derive(Eq, PartialEq)]
pub enum Arg {
    CheckNone,
    CheckSome,
    NoCheck,
}

pub fn command<'a>(cmds: &[Command], cfg: &mut Config, line: &str) {
    let mut it = split::split_n_whitespace(line, 2);

    if let Some(fst) = it.next() {
        for cmd in cmds {
            if fst.slice == cmd.name {
                match it.next() {
                    Some(rest) => {
                        if cmd.args == Arg::CheckNone {
                            eprintln!(
                                "Command `{}` expected no arguments, but `{}` was given.",
                                cmd.name, rest.slice
                            )
                        } else {
                            (cmd.fun)(cmds, cfg, line, Some(rest))
                        }
                    }
                    None => {
                        if cmd.args == Arg::CheckSome {
                            eprintln!(
                                "Command `{}` expected some arguments, but none was given.",
                                cmd.name
                            )
                        } else {
                            (cmd.fun)(cmds, cfg, line, None)
                        }
                    }
                }
                return;
            }
        }

        let mut tmp = vec![];
        for cmd in cmds {
            if cmd.name.starts_with(fst.slice) {
                tmp.push((cmd.usage, cmd.desc));
            }
        }

        eprintln!("Unrecognized command `{}`.", fst.slice);
        if tmp.len() != 0 {
            eprintln!("Similar commands:");
            for (usage, desc) in tmp {
                eprintln!("\t{}\t\t{}", usage, desc)
            }
        }
    } else {
        help_command(cmds, cfg, "", None)
    }
}

pub const LIMIT_COMMAND: Command<'static> = Command {
    name: "limit",
    usage: ":limit [num]",
    desc: "Manipulate the limit of reduction steps",
    args: Arg::NoCheck,
    fun: limit_command,
};

fn limit_command(_: &[Command], cfg: &mut Config, _: &str, arg: Option<Match>) {
    if let Some(arg) = arg {
        let mut it = split::split_n_whitespace(arg.slice, 2);
        if let Some(fst) = it.next() {
            if let Some(rest) = it.next() {
                eprintln!("Unexpected trailing characters `{}`.", rest.slice);
                return;
            }

            match fst.slice.parse::<usize>() {
                Ok(n) => cfg.limit = n,
                Err(e) => eprintln!("Expected number but {}.", e),
            }
        } else {
            println!("{}", cfg.limit)
        }
    } else {
        println!("{}", cfg.limit)
    }
}

pub const BIND_COMMAND: Command<'static> = Command {
    name: "bind",
    usage: ":bind name expr",
    desc: "Define a binding",
    args: Arg::CheckSome,
    fun: bind_command,
};

fn bind_command(_: &[Command], cfg: &mut Config, _: &str, arg: Option<Match>) {
    // Should be checked in command()
    debug_assert!(arg.is_some());
    let arg = arg.unwrap();

    let mut it = split::split_n_whitespace(arg.slice, 2);
    if let Some(x) = it.next() {
        if let Some(e) = it.next() {
            debug_assert!(it.next().is_none());
            match parse::parse(e.slice) {
                Some(Ok(e)) => {
                    if cfg.echo {
                        println!("{} = {}", x.slice, &e)
                    }

                    cfg.bind.insert(x.slice.to_string(), e.clone());
                    cfg.subst = cfg.subst.extend(x.slice.to_string(), e.clone());
                }
                Some(Err(e)) => eprintln!("{}", e),
                None => eprintln!("Expected expression after binding."),
            };
        } else {
            eprintln!("Expected expression after binding.")
        }
    } else {
        eprintln!("Expected binding name and expression.")
    }
}

pub const FILE_COMMAND: Command<'static> = Command {
    name: "file",
    usage: ":file name",
    desc: "Parse and evaluate a file",
    args: Arg::CheckSome,
    fun: file_command,
};

fn file_command(cmds: &[Command], cfg: &mut Config, _: &str, arg: Option<Match>) {
    // Should be checked in command()
    debug_assert!(arg.is_some());
    let arg = arg.unwrap();

    fn read(path: &PathBuf) -> io::Result<io::Lines<io::BufReader<File>>> {
        let file = File::open(path)?;
        Ok(io::BufReader::new(file).lines())
    }

    let name = arg.slice.trim_end();
    let path = {
        match Path::new(name).canonicalize() {
            Ok(f) => f,
            Err(e) => {
                eprintln!("File `{}` failed to load {}", name, e);
                return;
            }
        }
    };

    if let Some(f) = &cfg.file {
        if *f == path {
            eprintln!("File `{}` loads itself.", path.display());
            return;
        }
    }

    let prev = cfg.file.replace(path);
    match read(cfg.file.as_ref().unwrap()) {
        Ok(lines) => {
            for line in lines {
                if let Ok(line) = line {
                    if cfg.echo {
                        println!(">> {}", line)
                    }
                    if crate::interpret(cmds, cfg, &line) {
                        break;
                    }
                }
            }
        }
        Err(e) => eprintln!("{}", e),
    }
    cfg.file = prev;
}

pub const HELP_COMMAND: Command<'static> = Command {
    name: "help",
    usage: ":help",
    desc: "Display this menu",
    args: Arg::CheckNone,
    fun: help_command,
};

fn help_command(cmds: &[Command], _: &mut Config, _: &str, _: Option<Match>) {
    println!("Available commands:");
    for cmd in cmds {
        println!("\t{}\t\t{}", cmd.usage, cmd.desc)
    }
}

pub const SHOW_COMMAND: Command<'static> = Command {
    name: "show",
    usage: ":show",
    desc: "Display bindings",
    args: Arg::CheckNone,
    fun: show_command,
};

fn show_command(_: &[Command], cfg: &mut Config, _: &str, _: Option<Match>) {
    println!("Bindings:");
    for (k, v) in cfg.bind.iter() {
        println!("{} = {}", k, v)
    }
}
