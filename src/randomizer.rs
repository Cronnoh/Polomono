use rand::Rng;
use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
pub enum RandomizerStyle {
    NBag, // A randomized list of all pieces once
    DoubleNBag, // A randomized list containing each piece twice
    Minus1Bag, // A randomized list of all piece except for one, remember the excluded piece and don't exclude it twice in a row
    FullRandom, // Fully random list of pieces
    Classic, //Random list of pieces with rerolls on pieces that appear twice in a row
    Streak, // Get a random piece a random number of times in a row
}

pub struct Randomizer {
    piece_list: Vec<String>,
    pub style: RandomizerStyle,
    remembered_piece: Option<String>,
}

impl Randomizer {
    pub fn new(piece_list: Vec<String>, style: RandomizerStyle) -> Self {
        Self {
            piece_list,
            style,
            remembered_piece: None,
        }
    }

    pub fn generate_pieces(&mut self, cannot_start_with: &Option<Vec<String>>) -> Vec<String> {
        let mut new_pieces: Vec<String>;
        match self.style {
            RandomizerStyle::NBag => new_pieces = self.n_bag(),
            RandomizerStyle::DoubleNBag => new_pieces = self.double_n_bag(),
            RandomizerStyle::Minus1Bag => new_pieces = self.minus_1_bag(),
            RandomizerStyle::FullRandom => new_pieces = self.full_random(),
            RandomizerStyle::Classic => new_pieces = self.n_bag(),
            RandomizerStyle::Streak => new_pieces = self.n_bag(),
        }
        if let Some(disallowed) = cannot_start_with {
            fix_starting_piece(&mut new_pieces, disallowed);
        }
        new_pieces
    }

    fn n_bag(&self) -> Vec<String> {
        let mut bag = self.piece_list.clone();
        randomize(&mut bag);
        bag
    }

    fn double_n_bag(&self) -> Vec<String> {
        let mut bag = self.piece_list.clone();
        bag.append(&mut self.piece_list.clone());
        randomize(&mut bag);
        bag
    }

    fn minus_1_bag(&mut self) -> Vec<String> {
        let mut bag = self.piece_list.clone();
        randomize(&mut bag);
        if bag.len() < 2 {
            return bag;
        }

        // Ensure that the same piece is not removed from 2 bags in a row
        match &self.remembered_piece {
            Some(piece) => {
                let mut removed = bag.remove(bag.len()-1);
                let mut tested = Vec::new();
                while removed == *piece && !bag.is_empty() {
                    tested.push(removed);
                    removed = bag.remove(bag.len()-1);
                }
                self.remembered_piece = Some(removed);
                bag.append(&mut tested);
            }
            None => {
                self.remembered_piece = Some(bag.remove(bag.len()-1));
            }
        }
        bag
    }

    fn full_random(&self) -> Vec<String> {
        let mut pieces = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let r = rng.gen_range(0..self.piece_list.len());
            pieces.push(self.piece_list[r].clone());
        }

        pieces
    }
}

fn randomize<T>(bag: &mut [T]) {
    let mut rng = rand::thread_rng();
    let len = bag.len();
    for i in 0..len {
        bag.swap(i, rng.gen_range(i..len));
    }
}

fn fix_starting_piece(list: &mut Vec<String>, disallowed: &Vec<String>) {
    let len = list.len();
    if disallowed.contains(&list[len-1]) {
        for i in (0..len-2).rev() {
            if !disallowed.contains(&list[i]) {
                list.swap(i, len-1);
            }
        }
    }
}
