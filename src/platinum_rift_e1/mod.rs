//use std::io;

mod types {
    use std::cmp::PartialEq;

    const MAX_LINKS: usize = 6;
    const NEUTRAL_ID: i32 = -1;
    const POD_COST: usize = 20;

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
    }

    impl Map {
        pub fn new(size: usize) -> Map {
            Map {
                cells: Vec::with_capacity(size)
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
            let map = Map::new(6);
            assert_eq!(map.cells, Vec::new());
        }

        #[test]
        fn it_adds_cells() {
            let mut map = Map::new(1);
            map.add_cell(Cell::new(0, 0));
            assert_eq!(map.cells[0].id, 0);
            assert_eq!(map.cells[0].platinum, 0);
        }

        #[test]
        fn it_links_cells() {
            let mut map = Map::new(2);
            map.add_cell(Cell::new(0, 0));
            map.add_cell(Cell::new(1, 0));
            map.link_cells(0, 1);
            assert_eq!(map.cells[0].links, vec![1]);
            assert_eq!(map.cells[1].links, vec![0]);
        }
    }
}


/*
macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let player_count = parse_input!(inputs[0], i32); // the amount of players (2 to 4)
    let my_id = parse_input!(inputs[1], i32); // my player ID (0, 1, 2 or 3)
    let zone_count = parse_input!(inputs[2], i32); // the amount of zones on the map
    let link_count = parse_input!(inputs[3], i32); // the amount of links between all zones
    for i in 0..zone_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let zone_id = parse_input!(inputs[0], i32); // this zone's ID (between 0 and zoneCount-1)
        let platinum_source = parse_input!(inputs[1], i32); // the amount of Platinum this zone can provide per game turn
    }
    for i in 0..link_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let zone_1 = parse_input!(inputs[0], i32);
        let zone_2 = parse_input!(inputs[1], i32);
    }

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let platinum = parse_input!(input_line, i32); // my available Platinum
        for i in 0..zone_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let z_id = parse_input!(inputs[0], i32); // this zone's ID
            let owner_id = parse_input!(inputs[1], i32); // the player who owns this zone (-1 otherwise)
            let pods_p0 = parse_input!(inputs[2], i32); // player 0's PODs on this zone
            let pods_p1 = parse_input!(inputs[3], i32); // player 1's PODs on this zone
            let pods_p2 = parse_input!(inputs[4], i32); // player 2's PODs on this zone (always 0 for a two player game)
            let pods_p3 = parse_input!(inputs[5], i32); // player 3's PODs on this zone (always 0 for a two or three player game)
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");


        // first line for movement commands, second line for POD purchase (see the protocol in the statement for details)
        println!("WAIT");
        println!("1 73");
    }
}
*/
