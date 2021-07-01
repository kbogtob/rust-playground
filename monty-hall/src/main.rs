extern crate rand;
extern crate rand_chacha;

use std::{collections::HashSet, ops::Range};

use rand::{Rng, RngCore, prelude::SliceRandom, thread_rng};

#[derive(Debug)]
#[derive(PartialEq)]
enum DoorKinds {
    Car,
    Goat,
}

struct Game {
    doors: Vec<DoorKinds>,
}

impl Game {
    fn new(doors: Vec<DoorKinds>) -> Self {
        Game {
            doors
        }
    }

    fn random(rng: & mut dyn RngCore) -> Self {
        let mut doors = vec![DoorKinds::Goat, DoorKinds::Car, DoorKinds::Goat];
        doors.shuffle(rng);

        Self::new(doors)
    }

    fn doors(&self) -> &Vec<DoorKinds> {
        &self.doors
    }

    fn has_won(&self, player: &Player) -> bool {
        let final_choice = *player.final_choice().expect("Player hasn't played yet");

        let door = self.doors.get(final_choice).expect("Unknown door");

        *door == DoorKinds::Car
    }

    fn open_goat(&self, player_first_choice: &usize) -> usize {
        for (index, door) in self.doors.iter().enumerate() {
            if index != *player_first_choice && *door == DoorKinds::Goat {
                return index
            }
        }

        panic!("Did not find a goat")
    }
}

#[derive(Debug)]
struct Player {
    first_choice: Option<usize>,
    final_choice: Option<usize>,
    strategy: Box<dyn Strategy>,
}

impl Player {
    fn new(strategy: Box<dyn Strategy>) -> Player {
        Player {
            first_choice: None,
            final_choice: None,
            strategy,
        }
    }

    fn first_choice(&self) -> Option<&usize> {
        self.first_choice.as_ref()
    }

    fn final_choice(&self) -> Option<&usize> {
        self.final_choice.as_ref()
    }

    fn select(&mut self, doors: &Range<usize>) {
        if let None = self.first_choice {
            self.first_choice = Some(self.strategy.select(doors));
        }
    }

    fn validate(&mut self, doors: &Range<usize>, opened: &usize) {
        let first_choice = self.first_choice.as_ref().expect("Validating player without letting first choice");

        if let None = self.final_choice {
            self.final_choice = Some(self.strategy.validate(doors, first_choice, opened));
        }
    }
}

trait Behavior {
    fn select(&self, range: &Range<usize>) -> usize;
    fn validate(&self, doors: &Range<usize>, first_choice: &usize, opened: &usize) -> usize;
}

trait Strategy: Behavior + std::fmt::Debug {}

#[derive(Debug)]
struct KeepDoorStrategy {}

impl Behavior for KeepDoorStrategy {
    fn select(&self, range: &Range<usize>) -> usize {
        let mut rng = thread_rng();
        rng.gen_range(range.clone())
    }

    fn validate(&self, _doors: &Range<usize>, first_choice: &usize, _opened: &usize) -> usize {
        *first_choice
    }
}

impl Strategy for KeepDoorStrategy {}

#[derive(Debug)]
struct ChangeDoorStrategy {}

impl Behavior for ChangeDoorStrategy {
    fn select(&self, range: &Range<usize>) -> usize {
        let mut rng = thread_rng();
        rng.gen_range(range.clone())
    }

    fn validate(&self, doors: &Range<usize>, first_choice: &usize, opened: &usize) -> usize {
        let mut options = doors.clone().collect::<HashSet<usize>>();

        options.remove(first_choice);
        options.remove(opened);

        let mut options = options.into_iter().collect::<Vec<usize>>();
        
        options.shuffle(& mut thread_rng()); 

        *options.get(0).unwrap()
    }
}

impl Strategy for ChangeDoorStrategy {}

fn regen_players() -> Vec<Player> {
    vec![
        Player::new(Box::new(KeepDoorStrategy {})),
        Player::new(Box::new(ChangeDoorStrategy {})),
    ]
}

fn main() {
    let mut scores = vec![0, 0];
    let mut rng = thread_rng();
    
    for _i in 0..1000000 {
        let game = Game::random(& mut rng);
        let door_options = 0..game.doors().len();

        let mut players = regen_players();
        
        for (index, player) in players.iter_mut().enumerate() {
            player.select(&door_options);
            let first_choice = player.first_choice().expect("First choice should have been made");
            let a_goat_index = game.open_goat(first_choice);
            player.validate(&door_options, &a_goat_index);

            if game.has_won(player) {
                *scores.get_mut(index).unwrap() += 1;
            }
        }
    }

    println!("Scores are:");
    println!("Conservative player: {}", scores.get(0).unwrap());
    println!("Changing player: {}", scores.get(1).unwrap());
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use super::*;

    #[test]
    fn it_builds_random_games() {
        let mut rng = ChaCha8Rng::seed_from_u64(10u64);
        let game = Game::random(& mut rng);

        assert_eq!(game.doors(), &vec![DoorKinds::Goat, DoorKinds::Goat, DoorKinds::Car]);

        let game = Game::random(& mut rng);

        assert_eq!(game.doors(), &vec![DoorKinds::Car, DoorKinds::Goat, DoorKinds::Goat]);
    }
}