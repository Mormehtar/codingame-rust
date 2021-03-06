
//macro_rules! parse_input {
//    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
//}
macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.parse::<$t>().unwrap())
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
