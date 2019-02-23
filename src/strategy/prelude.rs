pub trait Strategy {
    fn new(board: Board) -> Self;
    fn get_board(&mut self) -> &mut Board;
    fn finish_turn_update(&mut self);
    fn build_turn(&mut self);
    fn collect_commands(&mut self) -> String;
}