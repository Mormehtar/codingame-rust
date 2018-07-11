mod cell;
mod continent;
// TODO For tests. Cleanup.
pub mod player;

use board::cell::{Cell, Pods};
use board::player::Player;
use board::continent::Continent;

const START_PLATINUM: usize = 200;

#[derive(Debug)]
pub struct Board {
    cells: Vec<Cell>,
    players: Vec<Player>,
    owner: i32,
    owner_platinum: usize,
    continents: Vec<Continent>,
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
            continents: Vec::new(),
        }
    }

    pub fn add_cell(&mut self, id: usize, platinum: usize) {
        self.cells.insert(id, Cell::new(id, platinum));
    }

    pub fn link_cells(&mut self, cell_id_1: usize, cell_id_2: usize) {
        self.cells[cell_id_1].link(&cell_id_2);
        self.cells[cell_id_2].link(&cell_id_1);
    }

    pub fn get_size(&self) -> usize {
        self.cells.len()
    }

    pub fn set_cell(&mut self, id: usize, owner: i32, pods: Pods) {
        self.cells[id].update(owner, pods);
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

    pub fn finish_init(&mut self) {
        self.cells.iter_mut().for_each(|cell| cell.finalize());
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
        map.add_cell(0, 0);
        assert_eq!(map.get_cell(0).platinum, 0);
    }

    #[test]
    fn it_links_cells() {
        let mut map = Board::new(2, 1, 0);
        map.add_cell(0, 0);
        map.add_cell(1, 0);
        map.link_cells(0, 1);
        assert_eq!(*map.get_cell(0).get_links(), vec![1]);
        assert_eq!(*map.get_cell(1).get_links(), vec![0]);
    }

    #[test]
    fn it_returns_size() {
        let mut map = Board::new(2, 1, 0);
        map.add_cell(0, 0);
        map.add_cell(1, 0);
        assert_eq!(map.get_size(), 2);
    }

    #[test]
    fn it_updates_cell() {
        let mut map = Board::new(2, 2, 0);
        map.add_cell(0, 0);
        map.add_cell(1, 0);
        map.set_cell(0, 1, [1, 1, 0, 0]);
        assert_eq!(*map.get_cell(0).get_owner(), 1);
        assert_eq!(*map.get_cell(0).get_pods(), [1, 1, 0, 0]);
    }

    #[test]
    fn it_manipulates_owner_platinum() {
        let mut map = Board::new(2, 2, 0);
        map.set_owner_platinum(5);
        assert_eq!(*map.get_owner_platinum(), 5);
    }

    #[test]
    fn it_finalizes_cells_correctly() {
        // TODO Should be mocked accurately, it looks into cell implementation now.
        let mut map = Board::new(3, 2, 0);
        map.add_cell(0, 0);
        map.add_cell(1, 0);
        map.add_cell(2, 0);
        map.link_cells(2, 1);
        map.link_cells(1, 0);
        map.finish_init();
        assert_eq!(map.get_cell(0).get_links().capacity(), 1);
        assert_eq!(map.get_cell(1).get_links().capacity(), 2);
        assert_eq!(*map.get_cell(1).get_links(), vec![0, 2]);
        assert_eq!(map.get_cell(2).get_links().capacity(), 1);
    }
}