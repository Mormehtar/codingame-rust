use types::board::Board;

struct Continent {
    cells: Vec<usize>,
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

    fn finalize(&mut self) {
        self.cells.sort_unstable();
    }

    fn new() -> Continent {
        Continent {
            cells: Vec::new(),
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
            .map(|_| Continent::new()).collect();
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
}