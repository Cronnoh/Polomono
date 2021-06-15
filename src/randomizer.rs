use rand::Rng;
use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
pub enum RandomizerStyle {
    NBag, // A randomized list of all pieces once
    DoubleNBag, // A randomized list containing each piece twice
    Minus1Bag, // A randomized list of all piece except for one, remember the excluded piece and don't exclude it twice in a row
    Random, // Fully random list of pieces
    Classic, //Random list of pieces with rerolls on pieces that appear twice in a row
    Streak, // Get a random piece a random number of times in a row
}

pub struct Randomizer {
    piece_list: Vec<String>,
    pub style: RandomizerStyle,
    _remembered_piece: Option<String>,
}

impl Randomizer {
    pub fn new(piece_list: Vec<String>, style: RandomizerStyle) -> Self {
        Self {
            piece_list,
            style,
            _remembered_piece: None,
        }
    }

    pub fn generate_pieces(&self, cannot_start_with: &Option<Vec<String>>) -> Vec<String> {
        let mut new_pieces: Vec<String>;
        match self.style {
            RandomizerStyle::NBag => new_pieces = self.n_bag(),
            RandomizerStyle::DoubleNBag => new_pieces = self.double_n_bag(),
            RandomizerStyle::Minus1Bag => new_pieces = self.n_bag(),
            RandomizerStyle::Random => new_pieces = self.n_bag(),
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
