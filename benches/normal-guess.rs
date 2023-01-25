#![feature(test)]
extern crate test;

use test::Bencher;
use wordle::Wordle;

#[bench]
fn bench_word_guess(b: &mut Bencher) {
    let mut word = Wordle::new("hello");
    let guess = "sadge";
    b.iter(|| word.guess(guess));
}
