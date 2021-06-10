use rocket::response::{Content, Stream};

use {
    crate::{
        structs::{File, State},
        utils,
    },
    rocket::{get, http::ContentType},
    std::{
        io::{self, prelude::*, BufReader, Read, Write},
        time::Duration,
    },
};

const BUF_SIZE: usize = 52428800;

impl Read for File {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        match self.state {
            State::Flush => {
                self.state = State::Sleep;
                return Err(io::ErrorKind::WouldBlock.into());
            }
            State::Sleep => std::thread::sleep(Duration::from_secs(self.delay)),
            State::Write => {}
        }

        self.state = State::Flush;

        self.data.clear();

        self.data += &format!("data: Watching file {}\ndata: \n", self.name);

        for line in utils::return_bufreader(&self.name).lines() {
            self.data += &format!("data: {}\n", line.expect("Error"))
        }
        self.data += "\n\n";

        buf.write_all(self.data.as_bytes())?;

        Ok(self.data.len())
    }
}

type CounterStream = Stream<BufReader<File>>;

#[get("/updates")]
pub fn updates(file_data: rocket::State<File>) -> Content<CounterStream> {
    let reader = BufReader::with_capacity(
        BUF_SIZE,
        File {
            data: file_data.data.clone(),
            state: file_data.state,
            name: file_data.name.clone(),
            delay: file_data.delay,
        },
    );

    let ct = ContentType::with_params("text", "event-stream", ("charset", "utf-8"));
    Content(ct, Stream::from(reader))
}
