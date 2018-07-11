use board::Board;
use board::cell::Owner;

#[derive(Debug)]
pub struct Continent {
    cells: Vec<usize>,
    platinum: usize,
    pods: [usize; 4],
    owned_cells: [usize; 4],
}

fn set_continent_mark(
    board: &Board,
    point: usize,
    value: usize,
    map: &mut Vec<usize>,
) {
    // TODO Try iterators again!
    map[point] = value;
    for link in &board.get_cell(point).links {
        if map[*link] == 0 {
            set_continent_mark(board, *link, value, map)
        }
    }
}

impl Continent {

    pub fn finalize(&mut self, board: &Board) {
        self.cells.sort_unstable();
        for i in &self.cells {
            self.platinum += board.get_cell(*i).platinum;
        }
    }

    pub fn collect_stats(&mut self, board: &Board) {
        self.pods = [0, 0, 0, 0];
        self.owned_cells = [0, 0, 0, 0];
        for i in &self.cells {
            let cell = board.get_cell(*i);
            let pods = cell.get_pods();
            for id in 0..pods.len() {
                self.pods[id] += pods[id];
            }
            match cell.get_owner() {
                Owner::Owned(id) => self.owned_cells[*id] += 1,
                _ => (),
            };
        }
    }

    pub fn new() -> Continent {
        Continent {
            cells: Vec::new(),
            platinum: 0,
            pods: [0, 0, 0, 0],
            owned_cells: [0, 0, 0, 0],
        }
    }

    pub fn build_continents(board: &Board) -> Vec<Continent> {
        let mut continent_number: usize = 0;
        let mut temp: Vec<usize> = (0..board.get_size()).map(|_| 0).collect();
        // TODO Try iterators again!
        for i in 0..temp.len() {
            if temp[i] == 0 {
                continent_number += 1;
                set_continent_mark(&board, i, continent_number, &mut temp);
            }
        }
        let mut continents: Vec<Continent> = (0..continent_number)
            .map(|_| Continent::new()).collect();
        for i in 0..temp.len() {
            continents[temp[i] - 1].cells.push(i);
        }
        for i in 0..continents.len() {
            continents[i].finalize(board);
        }
        continents
    }

    pub fn get_owned_cells(&self) -> &[usize; 4] {
        &self.owned_cells
    }

    pub fn get_pods(&self) -> &[usize; 4] {
        &self.pods
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use board::cell::NEUTRAL_ID;

    #[test]
    fn it_should_correctly_parse_board_on_continents() {
        let mut board = Board::new(10, 1, 0);
        (0..10).for_each(|i| board.add_cell(i, i));
        [[0, 1], [0, 2], [0, 3], [2, 3], [2, 4], [2, 5], [5, 6], [7, 8], [7, 9]].iter()
            .for_each(|data| board.link_cells(data[0], data[1]));
        let continents = Continent::build_continents(&board);
        assert_eq!(continents.len(), 2);
        assert_eq!(continents[0].cells.len(), 7);
        // TODO Should not relay on knowledge of implementation, but should check finalization.
        assert_eq!(continents[0].platinum, 1 + 2 + 3 + 4 + 5 + 6);
        assert_eq!(continents[1].cells.len(), 3);
        assert_eq!(continents[1].cells, vec![7, 8, 9]);
        // TODO Should not relay on knowledge of implementation, but should check finalization.
        assert_eq!(continents[1].platinum, 7 + 8 + 9);
    }

    #[test]
    fn it_should_collect_platinum_in_finalize() {
        let mut board = Board::new(3, 1, 0);
        board.add_cell(0, 0);
        board.add_cell(1, 5);
        board.add_cell(2, 1);
        let mut continent = Continent::new();
        continent.cells.push(2);
        continent.cells.push(0);
        continent.cells.push(1);
        continent.finalize(&board);
        assert_eq!(continent.cells, vec![0, 1, 2]);
        assert_eq!(continent.platinum, 6);
    }

    #[test]
    fn it_should_collect_stats_correctly() {
        let mut board = Board::new(3, 1, 0);
        board.add_cell(0, 0);
        board.add_cell(1, 5);
        board.add_cell(2, 1);
        let mut continent = Continent::new();
        continent.cells.push(2);
        continent.cells.push(0);
        continent.cells.push(1);
        continent.finalize(&board);
        board.set_cell(0, NEUTRAL_ID, [2, 2, 0, 0]);
        board.set_cell(1, 0, [5, 0, 0, 0]);
        board.set_cell(2, 1, [0, 3, 0, 0]);
        continent.collect_stats(&board);
        assert_eq!(continent.owned_cells, [1, 1, 0, 0]);
        assert_eq!(continent.pods, [7, 5, 0, 0]);
    }
}