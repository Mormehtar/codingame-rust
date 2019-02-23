use std::cmp::Ordering;
use board::Board;
use board::cell::{ Cell, Owner };
use maps::continent::Continent;

const MULTIPLIER: usize = 20;
const FADE: usize = 1;

struct PotentialElement {
    potential: usize,
    cell_id: usize,
}

impl PartialEq for PotentialElement {
    fn eq(&self, other: &Self) -> bool {
        return self.potential == other.potential;
    }
}

impl Eq for PotentialElement {}

impl PartialOrd for PotentialElement {
    fn partial_cmp(&self, other: &self) -> Option<Ordering>{
        self.potential.partial_cmp(other.potential)
    }
}

impl Ord for PotentialElement {
    fn cmp(&self, other: &Self) -> Ordering {
        self.potential.cmp(orther.potential)
    }
}

impl PotentialElement {
    fn build_potential(board: &Board, cell: &Cell) -> Self {
        let owner = Owner::Owned(board.get_owner());
        Self {
            cell_id: cell.get_id(),
            potential: match cell.get_owner() {
                owner => 0,
                _ => cell.get_platinum() * MULTIPLIER,
            }
        }
    }
    fn expand(&self, &mut other: Self) -> bool {
        if other.potential >= self.potential {
            return false;
        }
        other.potential = self.potential - FADE;
        true
    }
}

struct PotentialMap {
    potentials: Vec<PotentialElement>,
}

impl PotentialMap {
    pub fn new(continent: &Continent) -> Self {
        PotentialMap {
            potentials: Vec::with_capacity(continent.get_cells().len())
        }
    }

    pub fn build_potentials(board: &Board, continent: &Continent) -> Self {
        let map = Self::new(continent);
        for id in continent.get_cells() {
            map.potentials.push(PotentialElement::build_potential(board, board.get_cell(*id)));
        }
        map
    }
}