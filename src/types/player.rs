#[derive(Debug)]
pub struct Player {
    pub id: usize,
    pub platinum: usize,
    pub cells: usize,
    pub pods: usize,
}

impl Player {
    pub fn new(id: usize) -> Player {
        Player {
            id,
            platinum: 0,
            cells: 0,
            pods: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_player() {
        let player = Player::new(1);
        assert_eq!(player.id, 1);
        assert_eq!(player.platinum, 0);
        assert_eq!(player.cells, 0);
        assert_eq!(player.pods, 0);
    }
}
