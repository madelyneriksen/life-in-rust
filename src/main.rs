use conway::{Game, Point};
use std::time::Instant;

fn main() {
    let starter = vec![Point::new(8, 9), Point::new(8, 8), Point::new(8, 7)];
    let mut game = Game::new(starter);
    let instant = Instant::now();
    for _ in 0..1000 {
        game.run_generation();
    }
    println!(
        "Ran {} generations in {}ms 🔥",
        game.generation,
        instant.elapsed().as_millis()
    );
}
