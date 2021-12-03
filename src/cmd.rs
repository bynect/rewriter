use crate::{parse, split, split::Match, Config};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
        println!("Available commands:");
        for cmd in cmds {
            println!("\t{}\t\t{}", cmd.usage, cmd.desc)
        }
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

pub const LET_COMMAND: Command<'static> = Command {
    name: "let",
    usage: ":let name expr",
    desc: "Define a persisting substitution",
    args: Arg::CheckSome,
    fun: let_command,
};

fn let_command(_: &[Command], cfg: &mut Config, _: &str, arg: Option<Match>) {
    // Should be checked in command()
    debug_assert!(arg.is_some());
    let arg = arg.unwrap();

    let mut it = split::split_n_whitespace(arg.slice, 2);
    if let Some(x) = it.next() {
        if let Some(e) = it.next() {
            debug_assert!(it.next().is_none());
            match parse::parse(e.slice) {
                Some(Ok(e)) => cfg.subst = cfg.subst.extend(x.slice.to_string(), e),
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

    fn read(path: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
        let file = File::open(path)?;
        Ok(io::BufReader::new(file).lines())
    }

    let path = Path::new(arg.slice.trim_end());
    match read(&path) {
        Ok(lines) => {
            for line in lines {
                if let Ok(line) = line {
                    println!(">> {}", line);
                    if super::interpret(cmds, cfg, &line) {
                        break;
                    }
                }
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}
