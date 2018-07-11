use board::Board;
use local_io::ReadBuffer;

pub fn init_board(reader: &mut ReadBuffer) -> Board {
    let (player_count, my_id, zone_count, link_count) = read_header(reader);
    let mut board = Board::new(zone_count, player_count, my_id);
    fill_cells(reader, &mut board, zone_count);
    fill_links(reader, &mut board, link_count);
    board.finish_init();
    return board;
}

fn read_header(reader: &mut ReadBuffer) -> (usize, i32, usize, usize) {
    let first_line = reader.read_line();
    let player_count = first_line[0].parse().unwrap(); // the amount of players (2 to 4)
    let my_id = first_line[1].parse().unwrap(); // my player ID (0, 1, 2 or 3)
    let zone_count = first_line[2].parse().unwrap(); // the amount of zones on the map
    let link_count = first_line[3].parse().unwrap(); // the amount of links between all zones
    (player_count, my_id, zone_count, link_count)
}

fn fill_cells(reader: &mut ReadBuffer, board: &mut Board, n_cells: usize) {
    for _i in 0..n_cells {
        let data = reader.read_line();
        let id = data[0].parse().unwrap();
        let platinum = data[1].parse().unwrap();
        board.add_cell(id, platinum);
    }
}

fn fill_links(reader: &mut ReadBuffer, board: &mut Board, n_links: usize) {
    for _i in 0..n_links {
        let data = reader.read_line();
        let id1 = data[0].parse().unwrap();
        let id2 = data[1].parse().unwrap();
        board.link_cells(id1, id2);
    }
}

pub fn get_turn(reader: &mut ReadBuffer, board: &mut Board) {
    {
        let data = reader.read_line();
        board.set_owner_platinum(data[0].parse().unwrap());
    }
    for _i in 0..board.get_size() {
        let data = reader.read_line();
        let zid = data[0].parse().unwrap();
        let owner_id = data[1].parse().unwrap();
        let pods = [
            data[2].parse().unwrap(),
            data[3].parse().unwrap(),
            data[4].parse().unwrap(),
            data[5].parse().unwrap(),
        ];
        board.set_cell(zid, owner_id, pods);
    }
    board.finish_turn_update();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use board::player::Player;

    #[test]
    fn it_parses_input_correctly() {
        const TEST_INPUT: &str =
"2 0 3 2
0 0
1 0
2 2
0 1
1 2";
        let mut buffer = ReadBuffer::new(Box::new(Cursor::new(TEST_INPUT)));
        let board = init_board(&mut buffer);
        assert_eq!(*board.get_owner(), 0 as i32);
        assert_eq!(*board.get_player(0), Player::new(0));
        assert_eq!(*board.get_player(1), Player::new(1));
        let cell = board.get_cell(0);
        assert_eq!(cell.platinum, 0);
        assert_eq!(cell.links, vec![1]);
        let cell = board.get_cell(1);
        assert_eq!(cell.platinum, 0);
        assert_eq!(cell.links, vec![0, 2]);
        let cell = board.get_cell(2);
        assert_eq!(cell.platinum, 2);
        assert_eq!(cell.links, vec![1]);
    }

    #[test]
    fn it_parses_turn_correctly() {
        const TEST_INPUT: &str =
"1 0 2 0
0 0
1 1
10
0 0 1 0 0 0
1 -1 0 0 0 0";
        let mut buffer = ReadBuffer::new(Box::new(Cursor::new(TEST_INPUT)));
        let mut board = init_board(&mut buffer);
        get_turn(&mut buffer, &mut board);
        assert_eq!(*board.get_owner_platinum(), 10);
        let cell = board.get_cell(0);
        assert_eq!(*cell.get_owner(), 0);
        assert_eq!(*cell.get_pods(), [1, 0, 0, 0]);
        let cell = board.get_cell(1);
        assert_eq!(*cell.get_owner(), -1);
        assert_eq!(*cell.get_pods(), [0, 0, 0, 0]);
    }
}
