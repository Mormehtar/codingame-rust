use std::cmp::PartialEq;

const MAX_LINKS: usize = 6;
const NEUTRAL_ID: i32 = -1;

#[derive(Debug)]
pub struct Cell {
    id: usize,
    platinum: usize,
    links: Vec<usize>,
    owner: i32,
    pods: (usize, usize, usize, usize),
}

impl Cell {
    pub fn new(id: usize, platinum: usize) -> Cell {
        Cell {
            id,
            platinum,
            links: Vec::with_capacity(MAX_LINKS),
            owner: NEUTRAL_ID,
            pods: (0, 0, 0, 0),
        }
    }

    pub fn link(&mut self, id: &usize) {
        self.links.push(id.clone());
    }

    pub fn finalize(&mut self) {
        self.links.shrink_to_fit();
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        return self.id == other.id;
    }
}

#[derive(Debug)]
pub struct Map {
    cells: Vec<Cell>,
    players: Vec<Player>,
    owner: usize,
}

impl Map {
    pub fn new(size: usize, players_count: usize, owner: usize) -> Map {
        if owner >= players_count {
            panic!("Owner id not in players!");
        }
        let mut players = (0..players_count)
            .map(|id| Player::new(id))
            .collect::<Vec<Player>>();

        Map {
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

#[derive(Debug)]
pub struct Player {
    pub id: usize,
    pub platinum: usize,
    pub cells: usize,
    pub pods: usize,
}

impl Player {
    pub fn new(id: usize) -> Player {
        Player {
            id,
            platinum: 0,
            cells: 0,
            pods: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_cells() {
        let cell = Cell::new(1, 0);
        assert_eq!(cell.id, 1);
        assert_eq!(cell.platinum, 0);
        assert_eq!(cell.owner, NEUTRAL_ID);
        assert_eq!(cell.pods, (0,0,0,0));
        assert_eq!(cell.links, Vec::new());
    }

    #[test]
    fn it_creates_map() {
        let map = Map::new(6, 2, 1);
        assert_eq!(map.cells, Vec::new());
    }

    #[test]
    fn it_adds_cells() {
        let mut map = Map::new(1, 1, 0);
        map.add_cell(Cell::new(0, 0));
        assert_eq!(map.cells[0].id, 0);
        assert_eq!(map.cells[0].platinum, 0);
    }

    #[test]
    fn it_links_cells() {
        let mut map = Map::new(2, 1, 0);
        map.add_cell(Cell::new(0, 0));
        map.add_cell(Cell::new(1, 0));
        map.link_cells(0, 1);
        assert_eq!(map.cells[0].links, vec![1]);
        assert_eq!(map.cells[1].links, vec![0]);
    }

    #[test]
    fn it_creates_player() {
        let player = Player::new(1);
        assert_eq!(player.id, 1);
        assert_eq!(player.platinum, 0);
        assert_eq!(player.cells, 0);
        assert_eq!(player.pods, 0);
    }
}
