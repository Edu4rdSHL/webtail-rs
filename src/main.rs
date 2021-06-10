use std::{collections::HashMap, path::Path, u16};

use webtail_rs::{
    structs::{File, State},
    utils,
};

use {
    rayon::prelude::*,
    rocket::{
        config::{Config, Environment, LoggingLevel},
        routes,
    },
    rocket_contrib::serve::StaticFiles,
    webtail_rs::{args, worker::*},
};

fn main() {
    let args = args::get_args();

    if args.file_name.is_empty() && args.config_file.is_empty() {
        eprintln!("No files to process.");
        std::process::exit(1)
    }

    let mut input_files: HashMap<String, u16> = HashMap::new();

    if !args.config_file.is_empty() {
        let buffer = utils::return_bufreader(&args.config_file);
        input_files = utils::read_config_file(buffer);
    }
    if !args.file_name.is_empty() {
        input_files.insert(args.file_name.clone(), args.port);
    }

    input_files
        .par_iter()
        .map(|(file, port)| {
            if Path::new(&file).exists() {
                println!(
                    "\n=> Watching file \"{}\" in http://127.0.0.1:{}",
                    file, port
                );
                let error = rocket::custom(
                    Config::build(Environment::Production)
                        .log_level(LoggingLevel::Off)
                        .port(port.to_owned())
                        .unwrap(),
                )
                .manage(File {
                    data: String::new(),
                    name: file.clone(),
                    state: State::Write,
                    delay: args.delay,
                })
                .mount("/", routes![updates])
                .mount("/", StaticFiles::from(args.static_folder.clone()))
                .launch();
                eprintln!(
                    "Launch failed for file {} in port {}! Error: {}",
                    file, port, error
                );
            } else {
                eprintln!("Ignoring file {} because doesn't exists.", file)
            }
        })
        .collect::<Vec<()>>();
}
