use types::board::Board;

struct Continent {
    cells: Vec<usize>,
}

fn set_continent_mark(
    board: &Board,
    point: usize,
    value: usize,
    mut map: &mut Vec<usize>,
) {
    map[point] = value;
    board.get_cell(point).get_links().iter()
        .filter(|&point| map[*point] == 0)
        .for_each(|point| {
            set_continent_mark(board, *point, value, map)
        });
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
        (0..temp.len()).filter(|i| temp[*i] == 0).for_each(|i| {
            continent_number += 1;
            set_continent_mark(board, i, continent_number, &mut temp);
        });
        let mut continents: Vec<Continent> = (0..continent_number)
            .map(|_| Continent::new()).collect();
        for i in 0..temp.len() {
            continents[temp[i]].cells.push(i);
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