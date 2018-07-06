use std::cmp::PartialEq;

const MAX_LINKS: usize = 6;
const NEUTRAL_ID: i32 = -1;
const MAX_PLAYERS: usize = 4;

#[derive(Debug)]
pub struct Cell {
    pub id: usize,
    pub platinum: usize,
    pub links: Vec<usize>,
    pub owner: i32,
    pub pods: [usize; MAX_PLAYERS],
}

impl Cell {
    pub fn new(id: usize, platinum: usize) -> Cell {
        Cell {
            id,
            platinum,
            links: Vec::with_capacity(MAX_LINKS),
            owner: NEUTRAL_ID,
            pods: [0, 0, 0, 0],
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_cells() {
        let cell = Cell::new(1, 0);
        assert_eq!(cell.id, 1);
        assert_eq!(cell.platinum, 0);
        assert_eq!(cell.owner, NEUTRAL_ID);
        assert_eq!(cell.pods, [0,0,0,0]);
        assert_eq!(cell.links, Vec::new());
    }
}