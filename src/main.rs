use simulator_lib::directions::{coordinate::Coordinate, direction::Direction};
use simulator_lib::{start_server, Asteroid, Planet};
fn main() {
    let asteroids = vec![
        Asteroid {
            coordinate: Coordinate::new(250, 250),
            velocity: Direction { x: 30, y: -10 },
        },
        Asteroid {
            coordinate: Coordinate::new(750, 750),
            velocity: Direction { x: -30, y: 10 },
        },
    ];

    let planets = vec![Planet {
        coordinate: Coordinate::new(500, 500),
        weight: 50,
    }];

    start_server("localhost:16991", planets, asteroids, 70);
}
