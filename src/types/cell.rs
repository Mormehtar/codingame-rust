use std::cmp::PartialEq;

const MAX_LINKS: usize = 6;
const NEUTRAL_ID: i32 = -1;
const MAX_PLAYERS: usize = 4;

pub type Pods = [usize; MAX_PLAYERS];

#[derive(Debug)]
pub struct Cell {
    pub id: usize,
    pub platinum: usize,
    pub links: Vec<usize>,
    owner: i32,
    pods: Pods,
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

    pub fn get_links(&self) -> &Vec<usize> {
        &self.links
    }

    pub fn update(&mut self, owner: i32, pods: Pods) {
        self.owner = owner;
        self.pods = pods;
    }

    pub fn get_pods(&self) -> &Pods {
        &self.pods
    }

    pub fn get_owner(&self) -> &i32 {
        &self.owner
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

    #[test]
    fn it_exposes_links() {
        let mut cell = Cell::new(1, 0);
        cell.link(&13);
        assert_eq!(*cell.get_links(), vec![13]);
    }

    #[test]
    fn it_updates() {
        let mut cell = Cell::new(1, 0);
        cell.update(0, [1, 0, 0, 0]);
        assert_eq!(cell.owner, 0);
        assert_eq!(cell.pods, [1, 0, 0, 0]);
    }

    #[test]
    fn it_returns_owner() {
        let mut cell = Cell::new(1, 0);
        cell.update(0, [1, 0, 0, 0]);
        assert_eq!(*cell.get_owner(), 0);
    }

    #[test]
    fn it_returns_pods() {
        let mut cell = Cell::new(1, 0);
        cell.update(0, [1, 0, 0, 0]);
        assert_eq!(*cell.get_pods(), [1, 0, 0, 0]);
    }
}