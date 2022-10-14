pub mod directions;

use crate::directions::{coordinate::Coordinate, direction::Direction};

use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

use serde::{Deserialize, Serialize};

trait Location {
    fn get_location(&self) -> Coordinate;
}

trait ToCircle {
    fn as_circle(&self) -> Circle;
}

#[derive(Deserialize, Serialize)]
struct Circle {
    cx: i32,
    cy: i32,
    r: i32,
    stroke: String,
    fill: String,
    #[serde(rename = "stroke-width")]
    stroke_width: i32,
}

#[derive(Clone)]
pub struct Planet {
    pub coordinate: Coordinate,
    pub weight: i32,
}

impl Planet {
    fn get_weight(&self) -> i32 {
        self.weight
    }

    fn get_circle(&self) -> Circle {
        Circle {
            cx: self.coordinate.x,
            cy: self.coordinate.y,
            r: self.weight,
            stroke: "green".to_string(),
            fill: "black".to_string(),
            stroke_width: 3,
        }
    }
}

#[derive(Clone)]
pub struct Asteroid {
    pub coordinate: Coordinate,
    pub velocity: Direction,
}

impl Asteroid {
    fn get_velocity(&self) -> Direction {
        self.velocity.clone()
    }
}

impl Location for Asteroid {
    fn get_location(&self) -> Coordinate {
        self.coordinate.clone()
    }
}
impl Location for Planet {
    fn get_location(&self) -> Coordinate {
        self.coordinate.clone()
    }
}
impl ToCircle for Asteroid {
    fn as_circle(&self) -> Circle {
        Circle {
            cx: self.coordinate.x,
            cy: self.coordinate.y,
            r: 2,
            stroke: "green".to_string(),
            fill: "black".to_string(),
            stroke_width: 3,
        }
    }
}
impl ToCircle for Planet {
    fn as_circle(&self) -> Circle {
        Circle {
            cx: self.coordinate.x,
            cy: self.coordinate.y,
            r: self.weight,
            stroke: "green".to_string(),
            fill: "black".to_string(),
            stroke_width: 3,
        }
    }
}

// #[derive(Clone)]
// pub enum ObjectType {
//     Planet(Planet),
//     Asteroid(Asteroid),
// }

// impl ObjectType {
//     fn get_circle(&self) -> Circle {
//         match self {
//             ObjectType::Planet(p) => p.as_circle(),
//             ObjectType::Asteroid(a) => a.as_circle(),
//         }
//     }
// }

fn get_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)) as f64).sqrt() as i32
}

fn apply_physics(
    planets: Vec<Planet>,
    mut asteroids: Vec<Asteroid>,
    gravitational_constant: i32,
) -> (Vec<Planet>, Vec<Asteroid>) {
    // Go through each pair of objects, and apply
    // let gravity_sources = objects
    //     .iter()
    //     .filter_map(|o| {
    //         return if let ObjectType::Planet(p) = o {
    //             Some((p.coordinate.clone(), p.weight))
    //         } else {
    //             None
    //         };
    //     })
    //     .collect::<Vec<_>>();

    for mut asteroid in &mut asteroids {
        for planet in &planets {
            let distance = get_distance(
                planet.get_location().x,
                planet.get_location().y,
                asteroid.get_location().x,
                asteroid.get_location().y,
            );
            let distance = distance * distance;

            let force = Direction {
                x: (asteroid.get_location().x - planet.get_location().x)
                    * planet.get_weight()
                    * gravitational_constant
                    / distance,
                y: (asteroid.get_location().y - planet.get_location().y)
                    * planet.get_weight()
                    * gravitational_constant
                    / distance,
            };

            asteroid.velocity.x -= force.x;
            asteroid.velocity.y -= force.y;
        }
    }

    // objects.iter_mut().for_each(|o| {
    //     if let ObjectType::Asteroid(asteroid) = o {
    //         gravity_sources
    //             .iter()
    //             .for_each(|(planet_coord, planet_weight)| {
    //                 let distance = get_distance(
    //                     planet_coord.x,
    //                     planet_coord.y,
    //                     asteroid.coordinate.x,
    //                     asteroid.coordinate.y,
    //                 );
    //                 let distance = distance * distance;

    //                 let force = Direction {
    //                     x: (asteroid.coordinate.x - planet_coord.x)
    //                         * planet_weight
    //                         * gravitational_constant
    //                         / distance,
    //                     y: (asteroid.coordinate.y - planet_coord.y)
    //                         * planet_weight
    //                         * gravitational_constant
    //                         / distance,
    //                 };
    //                 asteroid.velocity.x -= force.x;
    //                 asteroid.velocity.y -= force.y;

    //                 let vel = asteroid.velocity.clone();
    //             })
    //     }
    // });

    // Apply the new velocity to each object.
    // objects.iter_mut().for_each(|object| {
    //     if let ObjectType::Asteroid(asteroid) = object {
    //         asteroid.coordinate.x += asteroid.velocity.x;
    //         asteroid.coordinate.y += asteroid.velocity.y;
    //     }
    // });

    for asteroid in &mut asteroids {
        asteroid.coordinate.x += asteroid.velocity.x;
        asteroid.coordinate.y += asteroid.velocity.y;
    }

    (planets, asteroids)
}

fn handle_connection(
    mut stream: TcpStream,
    mut planets: Vec<Planet>,
    mut asteroids: Vec<Asteroid>,
    gravitational_constant: i32,
) -> (Vec<Planet>, Vec<Asteroid>) {
    (planets, asteroids) = apply_physics(planets, asteroids, gravitational_constant);

    // let circles = planet.iter().map(|o| o.get_circle()).collect::<Vec<_>>();
    let mut circles = planets
        .iter()
        .map(|planet| planet.as_circle())
        .collect::<Vec<_>>();
    asteroids
        .iter()
        .for_each(|asteroid| circles.push(asteroid.as_circle()));

    let contents = serde_json::to_string(&circles).unwrap();
    let status_line = "HTTP/1.1 200 OK";
    let response = format!(
        "{status_line}\r\nContentType: application/json\r\nAccess-Control-Allow-Origin: *\r\n\r\n{contents}\r\n"
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    stream.shutdown(std::net::Shutdown::Both).unwrap();

    (planets, asteroids)
}

pub fn start_server(
    uri: &str,
    mut planets: Vec<Planet>,
    mut asteroids: Vec<Asteroid>,
    gravitational_constant: i32,
) -> ! {
    let listener = TcpListener::bind(uri).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        (planets, asteroids) =
            handle_connection(stream, planets, asteroids, gravitational_constant);
    }

    unreachable!()
}
