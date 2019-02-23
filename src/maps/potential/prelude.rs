use board::Board;
use board::cell::Cell;
use maps::continent::Continent;

trait PotentialElement: Ord {
    fn build_potential(continent_id: usize, board: &Board, cell: &Cell) -> Self;
    fn get_cell_id(&self) -> usize;
    fn get_continent_id(&self) -> usize;
    fn expand(&self, &mut other: Self) -> bool;
}

trait PotentialMap<Item: PotentialElement> {
    fn new(continent: &Continent)-> Self;
    fn add_potential(&mut self, potential: Item);
    fn build_potentials(board: &Board, continent: &Continent) -> Self {
        let mut map = Self::new(continent);
        let cells = continent.get_cells();
        for continent_id in 0..cells.len() {
            map.add_potential(
                PotentialElement::build_potential(
                    continent_id,
                    board,
                    board.get_cell(cells[continent_id]),
                )
            );
        }
        map
    }
}
