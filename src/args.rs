use {
    crate::structs::Args,
    clap::{load_yaml, value_t, App},
};

#[allow(clippy::cognitive_complexity)]
pub fn get_args() -> Args {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .version(clap::crate_version!())
        .get_matches();
    Args {
        file_name: value_t!(matches.value_of("file"), String).unwrap_or_default(),
        delay: value_t!(matches.value_of("delay"), u64).unwrap_or(1),
        port: value_t!(matches.value_of("port"), u16).unwrap_or(8080),
        config_file: value_t!(matches.value_of("config"), String).unwrap_or_default(),
        static_folder: value_t!(matches.value_of("static-folder"), String)
            .unwrap_or_else(|_| "/var/lib/webtail-rs/static".to_string()),
    }
}
