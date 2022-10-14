use simulator_lib::directions::{coordinate::Coordinate, direction::Direction};
use simulator_lib::{start_server, Asteroid, ObjectType, Planet};
fn main() {
    let mut objects = vec![
        ObjectType::Planet(Planet {
            coordinate: Coordinate::new(500, 500),
            weight: 50,
        }),
        ObjectType::Asteroid(Asteroid {
            coordinate: Coordinate::new(250, 250),
            velocity: Direction { x: 30, y: -10 },
        }),
        ObjectType::Asteroid(Asteroid {
            coordinate: Coordinate::new(750, 750),
            velocity: Direction { x: -30, y: 10 },
        }),
    ];

    start_server("localhost:16991", objects, 70);
}
