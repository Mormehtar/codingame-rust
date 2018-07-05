use std::io;

pub struct ReadBuffer {
    stream: Box<io::BufRead>,
    buffer: String,
}

impl ReadBuffer {
    pub fn new(stream: Box<io::BufRead>) -> ReadBuffer {
        ReadBuffer {
            stream,
            buffer: String::new()
        }
    }

    pub fn read_line(&mut self) -> Vec<&str> {
        self.stream.read_line(&mut self.buffer).unwrap();
        self.buffer.split_whitespace().map(|data: &str| {
            data.trim()
        }).collect::<Vec<&str>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_lines_correctly() {
        let mut stream = io::Cursor::new(b"f o o\nb a r");
        let mut buf = ReadBuffer::new(Box::new(stream));
        let result = buf.read_line();
        assert_eq!(result[0], "f");
        assert_eq!(result[1], "o");
        assert_eq!(result[2], "o");
    }
}