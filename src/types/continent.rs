use types::board::Board;

struct Continent<'a> {
    board: &'a Board,
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

impl <'a>Continent<'a> {

    fn finalize(&mut self) {
        self.cells.sort_unstable();
        for i in &self.cells {
            self.platinum += self.board.get_cell(*i).platinum;
        }
    }

    fn new(board: &Board) -> Continent {
        Continent {
            board,
            cells: Vec::new(),
            platinum: 0,
            pods: [0, 0, 0, 0],
            owned_cells: [0, 0, 0, 0],
        }
    }

    fn build_continents(board: &Board) -> Vec<Continent> {
        let mut continent_number: usize = 0;
        let mut temp: Vec<usize> = (0..board.get_size()).map(|_| 0).collect();
        // TODO Try iterators again!
        for i in 0..temp.len() {
            if temp[i] == 0 {
                continent_number += 1;
                set_continent_mark(board, i, continent_number, &mut temp);
            }
        }
        let mut continents: Vec<Continent> = (0..continent_number)
            .map(|_| Continent::new(board)).collect();
        for i in 0..temp.len() {
            continents[temp[i] - 1].cells.push(i);
        }
        for i in 0..continents.len() {
            continents[i].finalize();
        }
        continents
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::cell::Cell;

    #[test]
    fn it_should_correctly_parse_board_on_continents() {
        let mut board = Board::new(10, 1, 0);
        (0..10).for_each(|i| board.add_cell(Cell::new(i, 0)));
        [[0, 1], [0, 2], [0, 3], [2, 3], [2, 4], [2, 5], [5, 6], [7, 8], [7, 9]].iter()
            .for_each(|data| board.link_cells(data[0], data[1]));
        let continents = Continent::build_continents(&board);
        assert_eq!(continents.len(), 2);
        assert_eq!(continents[0].cells.len(), 7);
        assert_eq!(continents[1].cells.len(), 3);
        assert_eq!(continents[1].cells, vec![7, 8, 9]);
    }

    #[test]
    fn it_should_collect_platinum_in_finalize() {
        let mut board = Board::new(3, 1, 0);
        board.add_cell(Cell::new(0, 0));
        board.add_cell(Cell::new(1, 5));
        board.add_cell(Cell::new(2, 1));
        let mut continent = Continent::new(&board);
        continent.cells.push(2);
        continent.cells.push(0);
        continent.cells.push(1);
        continent.finalize();
        assert_eq!(continent.cells, vec![0, 1, 2]);
        assert_eq!(continent.platinum, 6);
    }
}