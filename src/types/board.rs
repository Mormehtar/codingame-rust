use types::cell::Cell;
use types::player::Player;

#[derive(Debug)]
pub struct Board {
    pub cells: Vec<Cell>,
    pub players: Vec<Player>,
    pub owner: usize,
}

impl Board {
    pub fn new(size: usize, players_count: usize, owner: usize) -> Board {
        if owner >= players_count {
            panic!("Owner id not in players!");
        }
        let players = (0..players_count)
            .map(|id| Player::new(id))
            .collect::<Vec<Player>>();

        Board {
            owner,
            cells: Vec::with_capacity(size),
            players,
        }
    }

    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.insert(cell.id, cell);
    }

    pub fn link_cells(&mut self, cell_id_1: usize, cell_id_2: usize) {
        self.cells[cell_id_1].link(&cell_id_2);
        self.cells[cell_id_2].link(&cell_id_1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_map() {
        let map = Board::new(6, 2, 1);
        assert_eq!(map.cells, Vec::new());
    }

    #[test]
    fn it_adds_cells() {
        let mut map = Board::new(1, 1, 0);
        map.add_cell(Cell::new(0, 0));
        assert_eq!(map.cells[0].id, 0);
        assert_eq!(map.cells[0].platinum, 0);
    }

    #[test]
    fn it_links_cells() {
        let mut map = Board::new(2, 1, 0);
        map.add_cell(Cell::new(0, 0));
        map.add_cell(Cell::new(1, 0));
        map.link_cells(0, 1);
        assert_eq!(map.cells[0].links, vec![1]);
        assert_eq!(map.cells[1].links, vec![0]);
    }
}