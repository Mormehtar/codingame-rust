use types::cell::Cell;
use types::player::Player;

const START_PLATINUM: usize = 200;

#[derive(Debug)]
pub struct Board {
    cells: Vec<Cell>,
    players: Vec<Player>,
    owner: i32,
    owner_platinum: usize,
}

impl Board {
    pub fn new(size: usize, players_count: usize, owner: i32) -> Board {
        if owner >= players_count as i32 {
            panic!("Owner id not in players!");
        }
        let players = (0..players_count)
            .map(|id| Player::new(id))
            .collect::<Vec<Player>>();

        Board {
            owner,
            cells: Vec::with_capacity(size),
            players,
            owner_platinum: START_PLATINUM,
        }
    }

    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.insert(cell.id, cell);
    }

    pub fn link_cells(&mut self, cell_id_1: usize, cell_id_2: usize) {
        self.cells[cell_id_1].link(&cell_id_2);
        self.cells[cell_id_2].link(&cell_id_1);
    }

    pub fn get_size(&self) -> usize {
        self.cells.len()
    }

    pub fn set_cell(&mut self, id: usize, owner: i32, pods: [usize; 4]) {
        let cell = &mut self.cells[id];
        cell.owner = owner;
        cell.pods = pods;
    }

    pub fn get_cell(&self, id: usize) -> &Cell {
        &self.cells[id]
    }

    pub fn set_owner_platinum(&mut self, platinum: usize) {
        self.owner_platinum = platinum;
    }

    pub fn get_owner_platinum(&self) -> &usize {
        &self.owner_platinum
    }

    pub fn get_owner(&self) -> &i32 {
        &self.owner
    }

    pub fn get_player(&self, id: i32) -> &Player {
        &self.players[id as usize]
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
        assert_eq!(map.get_cell(0).platinum, 0);
    }

    #[test]
    fn it_links_cells() {
        let mut map = Board::new(2, 1, 0);
        map.add_cell(Cell::new(0, 0));
        map.add_cell(Cell::new(1, 0));
        map.link_cells(0, 1);
        assert_eq!(map.get_cell(0).links, vec![1]);
        assert_eq!(map.get_cell(1).links, vec![0]);
    }

    #[test]
    fn it_returns_size() {
        let mut map = Board::new(2, 1, 0);
        map.add_cell(Cell::new(0, 0));
        map.add_cell(Cell::new(1, 0));
        assert_eq!(map.get_size(), 2);
    }

    #[test]
    fn it_updates_cell() {
        let mut map = Board::new(2, 2, 0);
        map.add_cell(Cell::new(0, 0));
        map.add_cell(Cell::new(1, 0));
        map.set_cell(0, 1, [1, 1, 0, 0]);
        assert_eq!(map.get_cell(0).owner, 1);
        assert_eq!(map.get_cell(0).pods, [1, 1, 0, 0]);
    }

    #[test]
    fn it_manipulates_owner_platinum() {
        let mut map = Board::new(2, 2, 0);
        map.set_owner_platinum(5);
        assert_eq!(*map.get_owner_platinum(), 5);
    }
}