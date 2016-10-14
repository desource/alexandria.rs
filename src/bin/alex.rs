extern crate alexandria;

use std::env;
use std::error;
use std::fmt;
use std::io::{Write, stderr, stdin};
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
        GenKeyCmd => {
            match alexandria::gen_key() {
                Ok(key) => {
                    println!("{}", key);
                }
                Err(()) => {
                    let _ = writeln!(&mut stderr(), "failed");
                    exit(1);
                }
            }
        }
        PubKeyCmd => {
            let mut input = String::new();
            let ref s = {
                match stdin().read_line(&mut input) {
                    Ok(n) => &input[..n-1],
                    Err(err) => {
                        let _ = writeln!(&mut stderr(), "pubkey failed {}", err);
                        exit(1);
                    }
                }
            };
            match alexandria::pub_key(s) {
                Ok(key) => {
                    println!("{}", key);
                }
                Err(()) => {
                    let _ = writeln!(&mut stderr(), "pubkey failed for `{}`", s);
                    exit(1);
                }
            }
        }
        EncryptCmd => { println!("TODO"); }
        DecryptCmd => { println!("TODO"); }
        VersionCmd => print_version(),
        HelpCmd(_kind) => {
            print_usage(0);
        }
    }
}

fn print_version() -> ! {
    println!("alex(andria) {}", env!("CARGO_PKG_VERSION"));
    exit(0);
}

fn print_usage(code: i32) -> ! {
    print!(r#"NAME:
  alex(andria) - A encrypt and decrypt messages efficently

USAGE:
  alex <command> [<args>...]

COMMANDS:
  genkey        Generate a new private key
  pubkey        Generate public key from private key
  encrypt, enc  Encrypt a message
  decrypt, dec  Decrypt a message
  version       Print version information
  help          Help about any command

GLOBAL OPTIONS:
  -d, --debug   Print debug info to stderr

See 'alex help <command>' for more information on a specific command.
"#);

    exit(code);
}


pub enum Cmd {
    GenKeyCmd,
    PubKeyCmd,
    EncryptCmd,
    DecryptCmd,
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

    // ignore process for now
    let _process = args.next();

    while let Some(arg) = args.next() {

        let (flag, _value) = split_arg(&arg);

        match &*flag {
            "-h" | "-help" | "--help" => return Ok(HelpCmd(DefaultHelp)),
            "genkey" => return Ok(GenKeyCmd),
            "pubkey" => return Ok(PubKeyCmd),
            "encrypt" | "enc" => return Ok(EncryptCmd),
            "decrypt" | "dec" => return Ok(DecryptCmd),
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
