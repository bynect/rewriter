use crate::{parse, split, split::Match, Config};

pub struct Directive<'a> {
    pub name: &'a str,
    pub desc: &'a str,
    pub check_arg: bool,
    pub fun: fn(&mut Config, &str, Option<Match>),
}

pub fn directive<'a>(dirs: &[Directive], cfg: &mut Config, line: &str) {
    let mut it = split::split_n_whitespace(line, 2);

    if let Some(fst) = it.next() {
        for dir in dirs {
            if fst.slice == dir.name {
                match it.next() {
                    Some(rest) => (dir.fun)(cfg, line, Some(rest)),
                    None => {
                        if dir.check_arg {
                            eprintln!("Expected arguments for directive `{}`", dir.name)
                        } else {
                            (dir.fun)(cfg, line, None)
                        }
                    }
                }
                return;
            }
        }

        let mut tmp = vec![];
        for dir in dirs {
            if dir.name.starts_with(fst.slice) {
                tmp.push((dir.name, dir.desc));
            }
        }

        eprintln!("Unrecognized directive `{}`", fst.slice);
        if tmp.len() != 0 {
            eprintln!("Directives with a similar name");
            for (name, desc) in tmp {
                eprintln!("\t:{}\t\t{}", name, desc)
            }
        }
    } else {
        println!("Available directives");
        for dir in dirs {
            println!("\t:{}\t\t{}", dir.name, dir.desc)
        }
    }
}

pub const LIMIT_DIRECTIVE: Directive<'static> = Directive {
    name: "limit",
    desc: "Set/Get the limit of reduction steps",
    check_arg: false,
    fun: limit_directive,
};

pub const LET_DIRECTIVE: Directive<'static> = Directive {
    name: "let",
    desc: "Define a persisting substitution",
    check_arg: true,
    fun: let_directive,
};

fn limit_directive(cfg: &mut Config, _: &str, arg: Option<Match>) {
    if let Some(arg) = arg {
        let mut it = split::split_n_whitespace(arg.slice, 2);
        if let Some(fst) = it.next() {
            if let Some(rest) = it.next() {
                eprintln!("Unexpected trailing characters `{}`", rest.slice);
                return;
            }

            match fst.slice.parse::<usize>() {
                Ok(n) => cfg.limit = n,
                Err(e) => eprintln!("Error: {}", e),
            }
        } else {
            println!("{}", cfg.limit)
        }
    } else {
        println!("{}", cfg.limit)
    }
}

fn let_directive(cfg: &mut Config, _: &str, arg: Option<Match>) {
    // Should be checked in directive()
    debug_assert!(arg.is_some());
    let arg = arg.unwrap();

    let mut it = split::split_n_whitespace(arg.slice, 2);
    if let Some(x) = it.next() {
        if let Some(e) = it.next() {
            debug_assert!(it.next().is_none());
            match parse::parse(e.slice) {
                Some(Ok(e)) => cfg.subst = cfg.subst.extend(x.slice.to_string(), e),
                Some(Err(e)) => eprintln!("{}", e),
                None => eprintln!("Expected expression"),
            };
        } else {
            eprintln!("Expected expression")
        }
    } else {
        eprintln!("Expected binding name and expression")
    }
}
