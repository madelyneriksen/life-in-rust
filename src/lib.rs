use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point {
            x,
            y,
        }
    }
    /// Returns all neighboring points/cells.
    ///
    /// This **includes** the current cell.
    pub fn neighbors(&self) -> Vec<Point> {
        let x_neighbors = vec![self.x - 1, self.x, self.x + 1];
        let y_neighbors = vec![self.y - 1, self.y, self.y + 1];
        let mut result = vec![];
        for x in &x_neighbors {
            for y in &y_neighbors {
                result.push(Point::new(*x, *y));
            }
        }
        result
    }
}

pub struct Game {
    board: HashSet<Point>,
    pub generation: usize,
}

impl Game {
    pub fn new(points: Vec<Point>) -> Game {
        let mut board = HashSet::new();
        for point in points {
            board.insert(point);
        }
        Game {
            board,
            generation: 0,
        }
    }
    /// Sums the amount of live cells in a given area.
    ///
    /// For calculating alive or dead cells.
    pub fn sum_live(&self, points: Vec<Point>) -> usize {
        let mut total = 0;
        for point in points {
            if self.board.contains(&point) {
                total += 1;
            }
        }
        total
    }
    /// Runs one generation of the Game of Life.
    pub fn run_generation(&mut self) {
        self.generation += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{Game, Point};

    #[test]
    fn point_gives_neighbors() {
        let point = Point::new(1, 1);
        let neighbors = point.neighbors();
        let expected = vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(0, 2),
            Point::new(1, 0),
            Point::new(1, 1),
            Point::new(1, 2),
            Point::new(2, 0),
            Point::new(2, 1),
            Point::new(2, 2),
        ];
        assert_eq!(neighbors, expected);
    }

    #[test]
    fn point_live_sum() {
        let starter = vec![Point::new(1, 1)];
        let neighbors = starter[0].neighbors();
        let game = Game::new(starter);
        let total = game.sum_live(neighbors);
        assert_eq!(total, 1);

        let starter = vec![
            Point::new(1, 1),
            Point::new(1, 2),
        ];
        let neighbors = starter[0].neighbors();
        let game = Game::new(starter);
        let total =  game.sum_live(neighbors);
        assert_eq!(total, 2)
    }

    #[test]
    fn game_generation_increments() {
        let starter = vec![];
        let mut game = Game::new(starter);
        assert_eq!(game.generation, 0);
        game.run_generation();
        assert_eq!(game.generation, 1);
    }
}
