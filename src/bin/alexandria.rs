extern crate alexandria;

use std::env;
use std::error;
use std::fmt;
use std::io::{Write, stderr};
use std::process::exit;


use self::Cmd::*;
use self::HelpKind::*;
use self::Fail::*;

fn main() {
    let cmd = match parse_args() {
        Ok(cmd) => cmd,
        Err(err) => {
            let _ = writeln!(&mut stderr(), "{}", err);
            print_usage(1);
        }
    };

    match cmd {
        EncryptCmd => { println!("TODO"); }
        DecryptCmd => { println!("TODO"); }
        KeysCmd    => { println!("TODO"); }
        VersionCmd => print_version(),
        HelpCmd(_kind) => {
            print_usage(0);
        }
    }
}

fn print_version() -> ! {
    println!("alexandria {}", env!("CARGO_PKG_VERSION"));
    exit(0);
}

fn print_usage(code: i32) -> ! {
    print!(r#"USAGE:
      alexandria <command> [<args>...]

COMMANDS:
      encrypt
      decrypt
      keys
      version  Print version information
      help     Help about any command

OPTIONS:
      --keystore FILE          Which keystore to use
  -v, --verbose                Use verbose output

See 'alexandria help <command>' for more information on a specific command.
"#);

    exit(code);
}


pub enum Cmd {
    EncryptCmd,
    DecryptCmd,
    KeysCmd,
    VersionCmd,
    HelpCmd(HelpKind),
}

pub enum HelpKind {
    DefaultHelp,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fail {
    UnknownFlag(String),
    UnknownCommand(String),
    MissingPath(&'static str),
    MissingArgument(String),
    InvalidDir(String),
}

impl error::Error for Fail {
    fn description(&self) -> &str {
        match *self {
            UnknownFlag(_)     => "unknown flag",
            UnknownCommand(_)  => "unknown command",
            MissingPath(_)     => "missing path",
            MissingArgument(_) => "missing argument",
            InvalidDir(_)      => "invalid path",
        }
    }
}

impl fmt::Display for Fail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnknownFlag(ref nm)     => write!(f, "Unknown flag '{}'", *nm),
            UnknownCommand(ref nm)  => write!(f, "Unknown command '{}'", *nm),
            MissingPath(ref nm)     => write!(f, "Required '{}' PATH missing", *nm),
            MissingArgument(ref nm) => write!(f, "Flag '{}' missing argument", *nm),
            InvalidDir(ref nm)      => write!(f, "Invalid directory '{}'", *nm),
        }
    }
}

pub fn parse_args() -> Result<Cmd, Fail> {

    // let mut opts = GlobalOptions{
    //     verbose:   false,
    // };

    let mut args = env::args();

    // ignore process
    let _process = args.next();

    while let Some(arg) = args.next() {

        let (flag, _value) = split_arg(&arg);

        match &*flag {
            "-h" | "-help" | "--help" => return Ok(HelpCmd(DefaultHelp)),
            "encrypt" => return Ok(EncryptCmd),
            "decrypt" => return Ok(DecryptCmd),
            "keys"    => return Ok(KeysCmd),
            "version" => return Ok(VersionCmd),
            "help"    => return parse_help(args),
            _         => return Err(UnknownCommand(flag)),
        }
    }

    Ok(HelpCmd(DefaultHelp))
}


fn parse_help(mut args: env::Args) -> Result<Cmd, Fail> {
    if let Some(arg) = args.next() {
        return match &*arg {
            // "index"  => Ok(HelpCmd(IndexHelp)),
            _ => if arg.starts_with("-") {
                Err(UnknownFlag(arg))
            } else {
                Err(UnknownCommand(arg))
            }
        }
    }
    Ok(HelpCmd(DefaultHelp))
}

fn split_arg(arg: &String) -> (String, Option<String>) {
    if let Some(idx) = arg.find('=') {
        let (flag, value) = arg.split_at(idx);
        if value.len() > 1 {
            (flag.into(), Some(value[1..].into()))
        } else {
            (flag.into(), None)
        }
    } else {
        (arg.clone(), None)
    }
}
