#[derive(Clone)]
pub struct Args {
    pub file_name: String,
    pub delay: u64,
    pub port: u16,
    pub config_file: String,
    pub static_folder: String,
}

pub struct File {
    pub data: String,
    pub state: State,
    pub name: String,
    pub delay: u64,
}

#[derive(Clone, Copy)]
pub enum State {
    Flush,
    Sleep,
    Write,
}
