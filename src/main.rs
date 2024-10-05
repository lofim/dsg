mod generator;
mod model;

use std::{fs::File, io::BufReader, path::PathBuf};

use anyhow::Result;
use clap::{value_parser, Arg, Command};
use generator::generate_ddl;
use model::Table;

fn main() {
    let dataset_size = Arg::new("size")
        .long("size")
        .short('s')
        .required(true)
        .value_parser(value_parser!(u32));

    let config_path = Arg::new("config")
        .long("config")
        .short('c')
        .required(true)
        .value_parser(value_parser!(PathBuf));

    let generate = Command::new("generate")
        .alias("g")
        .arg(dataset_size)
        .arg(config_path)
        .about("Generates a new data set with specified set length");

    let introspect = Command::new("introspect")
        .alias("i")
        .about("Introspects and existing PostgreSQL instance and infers model");

    let cli = Command::new("dsg")
        .about("This is a data set generator utility")
        .subcommand_required(true)
        .subcommand(generate)
        .subcommand(introspect);

    let result = run(cli);
    if let Err(e) = result {
        println!("Error while executing program: {}", e)
    }
}

fn run(app: Command) -> Result<()> {
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("generate", sub_matches)) => {
            let size = sub_matches.get_one::<u32>("size").unwrap();
            let config_path = sub_matches.get_one::<PathBuf>("config").unwrap();
            println!(
                "Generate submatch was execututed, size: {}, config: {:?}",
                size, config_path
            );

            let file = File::open(config_path)?;
            let reader = BufReader::new(file);
            let table = serde_json::from_reader::<_, Table>(reader)?;

            println!(
                "Generaing dataset for config: {}",
                serde_json::to_string(&table)?
            );

            // TODO: validation of input model
            let content = generate_ddl(&table, *size)?;
            println!("{}", content);

            Ok(())
        }
        Some(("introspect", sub_matches)) => {
            println!("Introspecting database, {:?}", sub_matches);
            Ok(())
        }
        _ => unreachable!(),
    }
}
