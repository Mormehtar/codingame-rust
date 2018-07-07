use std::io;

pub struct Commander {
    stream: Box<io::Write>,
    moves: Vec<Vec<usize>>,
    new_pods: Vec<Vec<usize>>,
}

fn build_command(commands: &Vec<Vec<usize>>) -> String {
    if commands.len() == 0 {
        return "WAIT\n".to_string();
    }
    let mut result = commands.iter()
        .map(
            |command| command.iter()
                .map(|element| element.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
        .collect::<Vec<_>>()
        .join(" ");
    result.push_str("\n");
    result
}

impl Commander {
    pub fn new(stream: Box<io::Write>) -> Commander {
        Commander {
            stream,
            moves: Vec::new(),
            new_pods: Vec::new(),
        }
    }

    pub fn make_turn(&mut self) {
        self.stream.write(build_command(&self.moves).as_bytes()).unwrap();
        self.stream.write(build_command(&self.new_pods).as_bytes()).unwrap();
        self.stream.flush().unwrap();
        self.clean_up();
    }

    fn clean_up(&mut self) {
        self.moves = vec![];
        self.new_pods = vec![];
    }

    fn make_move(&mut self, n_pods: usize, from: usize, to: usize) {
        self.moves.push(vec![n_pods, from, to]);
    }

    fn buy_pods(&mut self, n_pods: usize, destination: usize) {
        self.new_pods.push(vec![n_pods, destination]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_waits_if_no_command() {
        let stream = Box::new(io::Cursor::new(vec![]));
        let mut commander = Commander::new(stream);
        commander.make_turn();
        let result = commander.stream.into_inner();
        assert_eq!(String::from_utf8(result).unwrap(), "WAIT\nWAIT\n");
    }
}
