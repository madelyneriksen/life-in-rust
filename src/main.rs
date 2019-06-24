use conway::{Point, Game};

fn main() {
    let starter = vec![
        Point::new(8, 9),
        Point::new(8, 8),
        Point::new(8, 7),
    ];
    let mut game = Game::new(starter);
    game.run_generation();
    println!("{}", game.generation);
}
