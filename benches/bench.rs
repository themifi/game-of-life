#![feature(test)]

extern crate test;
extern crate game_of_life;

use game_of_life::Universe;

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = Universe::new();

    b.iter(|| {
        universe.tick();
    });
}
