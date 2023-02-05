//! Wordle is a simple word guessing game
//! this crate provides the logic for the game.

#[derive(Debug, Copy, Clone)]
/// `PartResult` is a struct that holds the result of a guess
/// for a single letter
pub struct PartResult {
    letter: char,
    in_word: bool,
    in_position: bool,
}

impl PartResult {
    #[must_use]
    /// Returns the letter that was guessed
    pub fn letter(&self) -> char {
        self.letter
    }

    #[must_use]
    /// Returns true if the letter is in the word
    pub fn in_word(&self) -> bool {
        self.in_word
    }

    #[must_use]
    /// Returns true if the letter is in the correct position
    pub fn in_position(&self) -> bool {
        self.in_position
    }
}

#[derive(Debug)]
/// `GuessResult` is a struct that holds the result of a guess
/// for the entire word
pub struct GuessResult {
    word: String,
    parts: Vec<PartResult>,
}

impl GuessResult {
    #[must_use]
    /// Returns the word that was guessed
    pub fn word(&self) -> &str {
        &self.word
    }

    #[must_use]
    /// Returns a vector of `PartResult`
    pub fn parts(&self) -> &Vec<PartResult> {
        &self.parts
    }
}

#[derive(Debug)]
/// Wordle is the main struct for the game
pub struct Wordle<'a> {
    word: &'a str,
    count: u32,
    won: bool,
}

impl Wordle<'_> {
    #[must_use]
    /// Creates a new Wordle struct
    pub fn new(word: &str) -> Wordle {
        Wordle {
            word,
            count: 0,
            won: false,
        }
    }

    /// Wordle guess check algorithm
    /// # Panics
    /// Panics if the guess is not the same length as the word
    pub fn guess(&mut self, guess: &str) -> GuessResult {
        if self.word == guess {
            self.set_won();
        }

        assert!(
            guess.len() == self.word.len(),
            "Guess must be the same length as the word"
        );

        let mut results = Vec::new();

        for c in guess.char_indices() {
            let letter = c.1;
            let in_word = self.word.contains(letter);
            let in_position = self.word.chars().nth(c.0) == Some(letter);

            results.push(PartResult {
                letter,
                in_word,
                in_position,
            });
        }

        GuessResult {
            word: guess.to_string(),
            parts: results,
        }
    }

    #[must_use]
    /// Returns the number of guesses
    pub fn count(&self) -> u32 {
        self.count
    }

    #[must_use]
    /// Returns true if the word has been guessed
    pub fn won(&self) -> bool {
        self.won
    }

    #[must_use]
    /// Returns the length of the word
    pub fn length(&self) -> usize {
        self.word.len()
    }

    #[must_use]
    /// Returns the word
    pub fn word(&self) -> &str {
        self.word
    }

    /// Sets the won flag to true
    fn set_won(&mut self) {
        self.won = true;
    }
}
