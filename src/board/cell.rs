use std::cmp::PartialEq;

const MAX_LINKS: usize = 6;
pub const NEUTRAL_ID: i32 = -1;
const MAX_PLAYERS: usize = 4;

pub type Pods = [usize; MAX_PLAYERS];

#[derive(Debug)]
#[derive(Clone)]
pub enum Owner {
    Owned(usize),
    UnOwned,
}

impl PartialEq for Owner {
    fn eq(&self, other: &Owner) -> bool {
        match (self, other) {
            (&Owner::UnOwned, &Owner::UnOwned) => true,
            (&Owner::Owned(x), Owner::Owned(y)) => x == *y,
            _ => false
        }
    }
}

#[derive(Debug)]
pub struct Cell {
    pub id: usize,
    pub platinum: usize,
    pub links: Vec<usize>,
    owner: Owner,
    pods: Pods,
}

impl Cell {
    pub fn new(id: usize, platinum: usize) -> Cell {
        Cell {
            id,
            platinum,
            links: Vec::with_capacity(MAX_LINKS),
            owner: Owner::UnOwned,
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
        self.owner = match owner {
            NEUTRAL_ID => Owner::UnOwned,
            id => Owner::Owned(id as usize)
        };
        self.pods = pods;
    }

    pub fn get_pods(&self) -> &Pods {
        &self.pods
    }

    pub fn get_owner(&self) -> Owner {
        self.owner.clone()
    }

    pub fn finalize(&mut self) {
        self.links.shrink_to_fit();
        self.links.sort_unstable();
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
        assert_eq!(cell.owner, Owner::UnOwned);
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
        assert_eq!(cell.owner, Owner::Owned(0));
        assert_eq!(cell.pods, [1, 0, 0, 0]);
    }

    #[test]
    fn it_returns_owner_when_neutral() {
        let mut cell = Cell::new(1, 0);
        cell.update(NEUTRAL_ID, [1, 0, 0, 0]);
        assert_eq!(cell.get_owner(), Owner::UnOwned);
    }

    #[test]
    fn it_returns_owner() {
        let mut cell = Cell::new(1, 0);
        cell.update(1, [1, 0, 0, 0]);
        assert_eq!(cell.get_owner(), Owner::Owned(1));
    }

    #[test]
    fn it_returns_pods() {
        let mut cell = Cell::new(1, 0);
        cell.update(0, [1, 0, 0, 0]);
        assert_eq!(*cell.get_pods(), [1, 0, 0, 0]);
    }

    #[test]
    fn it_finalizes() {
        let mut cell = Cell::new(1, 0);
        cell.link(&13);
        cell.link(&1);
        cell.finalize();
        assert_eq!(cell.links, vec![1, 13]);
        assert_eq!(cell.links.capacity(), 2);
    }
}