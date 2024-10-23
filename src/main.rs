use std::{
    fs::File,
    io::{BufReader, Write},
    path::PathBuf,
};

use clap::{self, Parser, ValueEnum};
use schema::Hospital;

mod schema;

#[derive(ValueEnum, Debug, Clone, Default)]
#[allow(clippy::upper_case_acronyms)]
enum Type {
    #[default]
    JSON,
    CSV,
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value_t, value_enum)]
    file_type: Type,
    file: PathBuf,
    #[arg(short, long, default_value = "./output.db")]
    database: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    if cli.database.exists() {
        println!("Database already exists");
    }

    let file = File::open(cli.file).unwrap();
    let reader = BufReader::new(file);

    let hospital_info: Hospital = serde_json::from_reader(reader).unwrap();

    println!("{} bytes", size_of_val(&hospital_info));

    // write to file
    let mut file = File::create("./output.json").unwrap();
    file.write_all(serde_json::to_string(&hospital_info).unwrap().as_bytes())
        .unwrap();
}
