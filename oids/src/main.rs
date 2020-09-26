use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

use toml::value::Value;
use clap::{Arg, App, AppSettings, SubCommand};

const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const SUBCOMMAND_LINT: &str = "lint";
const SUBCOMMAND_DECODE: &str = "decode";


fn main() {
    let matches = App::new(CARGO_PKG_NAME)
        .version(CARGO_PKG_VERSION)
        .author("Stacey Ell <software@e.staceyell.com")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .subcommand(
            SubCommand::with_name(SUBCOMMAND_LINT)
                .version(CARGO_PKG_VERSION)
                .arg(Arg::with_name("definition")
                    .takes_value(true)
                    .required(true)
                    .value_name("FILE")
                    .help("The definition file to use")
                    .index(1)))
        .subcommand(
            SubCommand::with_name(SUBCOMMAND_DECODE)
                .version(CARGO_PKG_VERSION)
                .arg(Arg::with_name("definition")
                    .takes_value(true)
                    .required(true)
                    .value_name("FILE")
                    .help("The definition file to use")
                    .index(1))
                .arg(Arg::with_name("oid")
                    .takes_value(true)
                    .required(true)
                    .value_name("OID")
                    .help("The OID to look up")
                    .index(2)))
        .get_matches();

    let (sub_name, args) = matches.subcommand();
    match sub_name {
        SUBCOMMAND_LINT => main_lint(args.unwrap()),
        SUBCOMMAND_DECODE => main_decode(args.unwrap()),
        _ => panic!("unknown subcommand"),
    }
}

enum OidElement<'a> {
    Known { name: &'a str, number: &'a str },
    Unknown { number: &'a str },
}

fn main_decode(matches: &clap::ArgMatches<'_>) {
    let definition = matches.value_of_os("definition").unwrap();
    let oid = matches.value_of("oid").unwrap();

    let mut f = File::open(definition).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    drop(f);

    let val: Value = toml::from_str(&buf[..]).unwrap();
    let mut table: Option<&toml::value::Table>;
    if let Value::Table(ref tab) = val {
        table = Some(tab);
    } else {
        panic!("must be table");
    }


    let mut elements = Vec::new();
    for number in oid.split('.') {
        elements.push(match table {
            Some(tab) => {
                match tab.get(number) {
                    Some(Value::Table(ref subtab)) => {
                        table = Some(subtab);
                        if let Some(Value::String(ref name)) = subtab.get("name") {
                            OidElement::Known { name, number }
                        } else {
                            OidElement::Unknown { number }
                        }
                    },
                    Some(..) => {
                        table = None;
                        OidElement::Unknown { number }
                    },
                    None => {
                        table = None;
                        OidElement::Unknown { number }
                    }
                }
            },
            None => OidElement::Unknown { number },
        });
    }

    for ele in &elements {
        match *ele {
            OidElement::Known { name, number } => {
                print!("{}({}) ", name, number);
            },
            OidElement::Unknown { number } => {
                print!("{} ", number);
            },
        }
    }
    println!();
}

fn main_lint(matches: &clap::ArgMatches<'_>) {
    let definition = matches.value_of_os("definition").unwrap();
    
    let mut f = File::open(definition).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    drop(f);

    let val: Value = toml::from_str(&buf[..]).unwrap();

    if let Value::Table(ref tab) = val {
        if let Err(err) = validate_table(&[], tab) {
            println!("{}", err);
        }
    } else {
        panic!("must be table");
    }
}

fn validate_table(path: &[&str], table: &BTreeMap<String, Value>) -> Result<(), String> {
    if path.len() != 0 {
        table.get("name").ok_or_else(|| {
            format!("missing name for path: {:?} :: {:#?}", path, table)
        })?;
    }

    for (key, child) in table {
        let mut path = path.to_vec();
        path.push(key);
        if let Value::Table(ref tab) = *child {
            validate_table(&path[..], tab)?;
        }
    }

    Ok(())
}
