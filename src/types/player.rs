use std::cmp::PartialEq;

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

impl PartialEq for Player {
    fn eq(&self, other: &Player) -> bool {
        return self.id == other.id;
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
