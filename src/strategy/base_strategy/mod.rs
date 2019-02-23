use board::Board;
use maps::continent::Continent;
use commander::Commander;
use commander::BaseCommander;

use strategy::prelude::*;

pub struct BaseStrategy {
    board: Board,
    continents: Vec<Continent>,
    commander: BaseCommander,
}

impl Strategy for BaseStrategy {
    fn new(board: Board) -> BaseStrategy {
        let mut strategy = BaseStrategy {
            board,
            commander: BaseCommander::new(),
            continents: Vec::new(),
        };
        strategy.finish_init();
        strategy
    }
    fn get_board(&mut self) -> &mut Board {
        &mut self.board
    }

    fn finish_turn_update(&mut self) {
        for continent in &mut self.continents {
            continent.collect_stats(&self.board);
        }
    }

    fn build_turn(&mut self) {

    }
    fn collect_commands(&mut self) -> String {
        self.commander.make_turn()
    }
}

impl BaseStrategy {
    fn finish_init(&mut self) {
        self.continents = Continent::build_continents(&self.board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use board::cell::NEUTRAL_ID;

    #[test]
    fn it_should_init_correctly() {
        let mut map = Board::new(3, 2, 0);
        map.add_cell(0, 0);
        map.add_cell(1, 0);
        map.add_cell(2, 0);
        map.link_cells(2, 1);
        map.link_cells(1, 0);
        let mut strategy = BaseStrategy::new(map);
        assert_eq!(strategy.continents.len(), 1);
    }

    fn it_should_collect_stats_correctly() {
        let mut board = Board::new(3, 1, 0);
        board.add_cell(0, 0);
        board.add_cell(1, 5);
        board.add_cell(2, 1);
        board.link_cells(2, 1);
        board.link_cells(1, 0);
        let mut strategy = BaseStrategy::new(board);
        {
            let mut board = strategy.get_board();
            board.set_cell(0, NEUTRAL_ID, [2, 2, 0, 0]);
            board.set_cell(1, 0, [5, 0, 0, 0]);
            board.set_cell(2, 1, [0, 3, 0, 0]);
        }
        strategy.finish_turn_update();
        assert_eq!(strategy.continents[0].get_owned_cells(), &[1, 1, 0, 0]);
        assert_eq!(strategy.continents[0].get_pods(), &[7, 5, 0, 0]);
    }
}
