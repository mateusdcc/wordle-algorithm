use wordle::Wordle;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_new() {
        let word = Wordle::new("hello");
        assert_eq!(word.word(), "hello");
        assert_eq!(word.count(), 0);
        assert_eq!(word.won(), false);
    }

    #[test]
    fn test_word_guess_one_letter() {
        let mut word = Wordle::new("hello");
        let guess = "sadge";
        let result = word.guess(guess);

        assert_eq!(result.parts().len(), word.length());
        let part = result.parts().get(4).unwrap();
        assert_eq!(part.letter(), 'e');
        assert_eq!(part.in_word(), true);
        assert_eq!(part.in_position(), true);
    }

    #[test]
    fn test_word_guess_three_letters() {
        let mut word = Wordle::new("hello");
        let guess = "sadge";
        let result = word.guess(guess);

        assert_eq!(result.parts().len(), word.length());
        let part = result.parts().get(0).unwrap();
        assert_eq!(part.letter(), 's');
        assert_eq!(part.in_word(), false);
        assert_eq!(part.in_position(), false);

        let part = result.parts().get(1).unwrap();
        assert_eq!(part.letter(), 'a');
        assert_eq!(part.in_word(), false);
        assert_eq!(part.in_position(), false);

        let part = result.parts().get(2).unwrap();
        assert_eq!(part.letter(), 'd');
        assert_eq!(part.in_word(), false);
        assert_eq!(part.in_position(), false);

        let part = result.parts().get(3).unwrap();
        assert_eq!(part.letter(), 'g');
        assert_eq!(part.in_word(), false);
        assert_eq!(part.in_position(), false);

        let part = result.parts().get(4).unwrap();
        assert_eq!(part.letter(), 'e');
        assert_eq!(part.in_word(), true);
        assert_eq!(part.in_position(), true);
    }
}