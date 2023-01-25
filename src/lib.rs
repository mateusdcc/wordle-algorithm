//! Wordle is a simple word guessing game
//! this crate provides the logic for the game.

#[derive(Debug, Copy, Clone)]
/// PartResult is a struct that holds the result of a guess
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
/// GuessResult is a struct that holds the result of a guess
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
    /// Returns a vector of PartResult
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

        let mut guess_sym: Vec<(usize, char)> = guess.char_indices().collect();
        let filtered_list = guess
            .char_indices()
            .map(|c| {
                let mut found = false;
                for (i, g) in guess_sym.clone().iter().enumerate() {
                    if g.1 == c.1 {
                        guess_sym.remove(i);
                        found = true;
                    } else {
                        found = false;
                    }
                }
                (c.0, c.1, found)
            })
            .collect::<Vec<(usize, char, bool)>>();

        let mut results = Vec::new();

        for (i, c, b) in filtered_list {
            let mut part;
            if b {
                part = PartResult {
                    letter: c,
                    in_word: true,
                    in_position: false,
                };

                if guess.char_indices().nth(i).unwrap().1 == c {
                    part.in_position = true;
                }
            } else {
                part = PartResult {
                    letter: c,
                    in_word: false,
                    in_position: false,
                }
            }

            results.push(part);
        }

        self.count += 1;

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
