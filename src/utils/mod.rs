use std;
use board::Board;
use strategy::Strategy;
use local_io::ReadBuffer;

pub fn init_board(reader: &mut ReadBuffer) -> Board {
    let [player_count, my_id, zone_count, link_count] = read_header(reader);
    let mut board = Board::new(zone_count, player_count, my_id);
    fill_cells(reader, &mut board, zone_count);
    fill_links(reader, &mut board, link_count);
    board.finish_init();
    return board;
}

fn read_header(reader: &mut ReadBuffer) -> [usize; 4] {
    let line = reader.read_line();
    let mut iterator = line.iter().take(4)
        .map(|element| element.parse().unwrap());
    [
        iterator.next().unwrap(),
        iterator.next().unwrap(),
        iterator.next().unwrap(),
        iterator.next().unwrap(),
    ]
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
        let mut iterator = data.iter().skip(2).take(4)
            .map(|element| element.parse().unwrap());
        let pods: [usize; 4] = [
            iterator.next().unwrap(),
            iterator.next().unwrap(),
            iterator.next().unwrap(),
            iterator.next().unwrap(),
        ];
        board.set_cell(zid, owner_id, pods);
    }
}

pub fn main<StrategyImplementation: Strategy, WriteBuffer: std::io::Write>(
    mut reader: ReadBuffer, write: &mut WriteBuffer
) {
    let board = init_board(&mut reader);
    let mut strategy = StrategyImplementation::new(board);
    loop {
        get_turn(&mut reader, strategy.get_board());
        strategy.finish_turn_update();
        strategy.build_turn();
        write.write(strategy.collect_commands().as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use board::player::Player;
    use board::cell::Owner;

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
        assert_eq!(board.get_owner(), 0);
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
        assert_eq!(board.get_owner_platinum(), 10);
        let cell = board.get_cell(0);
        assert_eq!(cell.get_owner(), Owner::Owned(0));
        assert_eq!(*cell.get_pods(), [1, 0, 0, 0]);
        let cell = board.get_cell(1);
        assert_eq!(cell.get_owner(), Owner::UnOwned);
        assert_eq!(*cell.get_pods(), [0, 0, 0, 0]);
    }
}
