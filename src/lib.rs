pub mod directions;

use crate::directions::{coordinate::Coordinate, direction::Direction};

use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

use serde::{Deserialize, Serialize};

pub trait Location {
    fn get_location(&self) -> Coordinate;
    fn update_location(&mut self, new_location: Coordinate);
}

pub trait ToCircle {
    fn as_circle(&self) -> Circle;
}

pub trait Mass {
    fn get_weight(&self) -> i32;
}

pub trait Velocity {
    fn get_velocity(&self) -> Direction;
    fn update_velocity(&mut self, new_velocity: Direction);
}

#[derive(Deserialize, Serialize)]
pub struct Circle {
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

impl Mass for Planet {
    fn get_weight(&self) -> i32 {
        self.weight
    }
}

#[derive(Clone)]
pub struct Asteroid {
    pub coordinate: Coordinate,
    pub velocity: Direction,
}

impl Location for Asteroid {
    fn get_location(&self) -> Coordinate {
        self.coordinate.clone()
    }
    fn update_location(&mut self, new_location: Coordinate) {
        self.coordinate = new_location;
    }
}
impl Location for Planet {
    fn get_location(&self) -> Coordinate {
        self.coordinate.clone()
    }
    fn update_location(&mut self, new_location: Coordinate) {
        self.coordinate = new_location;
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
impl Velocity for Asteroid {
    fn get_velocity(&self) -> Direction {
        self.velocity.clone()
    }
    fn update_velocity(&mut self, new_velocity: Direction) {
        self.velocity = new_velocity;
    }
}

fn get_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)) as f64).sqrt() as i32
}

fn apply_physics<T, U>(
    planets: Vec<T>,
    mut asteroids: Vec<U>,
    gravitational_constant: i32,
) -> (Vec<T>, Vec<U>)
where
    T: Mass + Location,
    U: Location + Velocity,
{
    for asteroid in &mut asteroids {
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

            asteroid.update_velocity(Direction {
                x: asteroid.get_velocity().x - force.x,
                y: asteroid.get_velocity().y - force.y,
            });
        }
    }
    for asteroid in &mut asteroids {
        asteroid.update_location(Coordinate {
            x: asteroid.get_location().x + asteroid.get_velocity().x,
            y: asteroid.get_location().y + asteroid.get_velocity().y,
        });
    }

    (planets, asteroids)
}

fn handle_connection<T, U>(
    mut stream: TcpStream,
    mut planets: Vec<T>,
    mut asteroids: Vec<U>,
    gravitational_constant: i32,
) -> (Vec<T>, Vec<U>)
where
    T: Mass + Location + ToCircle,
    U: Location + Velocity + ToCircle,
{
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

pub fn start_server<T, U>(
    uri: &str,
    mut planets: Vec<T>,
    mut asteroids: Vec<U>,
    gravitational_constant: i32,
) -> !
where
    T: Mass + Location + ToCircle,
    U: Location + Velocity + ToCircle,
{
    let listener = TcpListener::bind(uri).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        (planets, asteroids) =
            handle_connection(stream, planets, asteroids, gravitational_constant);
    }

    unreachable!()
}
