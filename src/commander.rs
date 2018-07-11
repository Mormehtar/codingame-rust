pub trait Commander {
    fn new() -> Self;
    fn make_turn(&mut self) -> String;
    fn make_move(&mut self, n_pods: usize, from: usize, to: usize);
    fn buy_pods(&mut self, n_pods: usize, destination: usize);
}

pub struct BaseCommander {
    moves: Vec<Vec<usize>>,
    new_pods: Vec<Vec<usize>>,
}

impl Commander for BaseCommander {
    fn new() -> BaseCommander {
        BaseCommander {
            moves: Vec::new(),
            new_pods: Vec::new(),
        }
    }

    fn make_turn(&mut self) -> String {
        let mut result = build_command(&self.moves);
        result.push_str(&build_command(&self.new_pods));
        self.clean_up();
        return result;
    }

    fn make_move(&mut self, n_pods: usize, from: usize, to: usize) {
        self.moves.push(vec![n_pods, from, to]);
    }

    fn buy_pods(&mut self, n_pods: usize, destination: usize) {
        self.new_pods.push(vec![n_pods, destination]);
    }
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

impl BaseCommander {
    fn clean_up(&mut self) {
        self.moves = vec![];
        self.new_pods = vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_waits_if_no_command() {
        let mut commander = BaseCommander::new();
        let result = commander.make_turn();
        assert_eq!(result, "WAIT\nWAIT\n");
    }

    #[test]
    fn it_makes_moves_correctly_if_no_new_pods() {
        let mut commander = BaseCommander::new();
        commander.make_move(1, 0, 1);
        commander.make_move(1, 1, 2);
        let result = commander.make_turn();
        assert_eq!(result, "1 0 1 1 1 2\nWAIT\n");
    }

    #[test]
    fn it_buys_new_pods_correctly_if_no_moves() {
        let mut commander = BaseCommander::new();
        commander.buy_pods(1, 0);
        commander.buy_pods(1, 1);
        let result = commander.make_turn();
        assert_eq!(result, "WAIT\n1 0 1 1\n");
    }

    #[test]
    fn it_allows_to_but_and_move() {
        let mut commander = BaseCommander::new();
        commander.buy_pods(1, 0);
        commander.buy_pods(1, 1);
        commander.make_move(1, 0, 1);
        commander.make_move(1, 1, 2);
        let result = commander.make_turn();
        assert_eq!(result, "1 0 1 1 1 2\n1 0 1 1\n");
    }

    #[test]
    fn it_works_correctly_with_consqutive_turns() {
        let mut commander = BaseCommander::new();
        commander.buy_pods(1, 0);
        commander.make_move(1, 0, 1);
        let result = commander.make_turn();
        assert_eq!(result, "1 0 1\n1 0\n");
        commander.buy_pods(1, 1);
        commander.make_move(1, 1, 2);
        let result = commander.make_turn();
        assert_eq!(result, "1 1 2\n1 1\n");
    }

}
