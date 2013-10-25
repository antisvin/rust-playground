use std::{io,path};

fn load(filename: ~str) -> ~[~str] {
}


struct Chunk {
    data: ~str,
    first: ~str,
    max_num: int
}

impl Chunk {
    fn new(data: ~str) -> Chunk {
        Chunk{data: data, first: ~"", max_num: 0}
    }
}


fn main() {
    let read_result = io::file_reader(~path::Path(~"numbers.txt"));

    match read_result {
        Ok(file) => {
            while !file.eof() {
                let buf: ~uint[32768];
                match file.read(buf) {
                    Some(bytes) => {
                        println(fmt!("%i bytes read", bytes))
                    },
                    None => {
                        println(fmt!("Error reading file: %s", e))
                    }
                }
            }
        },
        Err(e) => {
            println(fmt!("Error reading file: %s", e));
            return
        }
    }

}
