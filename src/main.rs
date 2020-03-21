extern crate csv;
use csv::{ReaderBuilder};
use serde::{Deserialize, Serialize};

pub struct Position {
    latitude: f32,
    longtitude: f32
}

impl Position {
    pub fn default() -> Position {
        Position {
            latitude: 0.0,
            longtitude: 0.0
        }
    }
    pub fn new(latitude: f32, longtitude: f32) -> Position {
        Position {
            latitude: latitude,
            longtitude: longtitude
        }
    }
    pub fn update(mut self, latitude: f32, longtitude: f32) {
        self.latitude = latitude;
        self.longtitude = longtitude;
    }
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub struct Country {
    pub name: String,
    pub population: i64
}

pub struct Person {
    pub infected: bool,
    pub position: Position
} 

impl Person {
    pub fn new() -> Person {
        Person {
            infected: false,
            position: Position::default(),
        }
    }
}

fn read_file<T>() -> Vec<T> where T: for<'de> Deserialize<'de> {
    let mut list = Vec::<T>::new();
    let mut reader = match ReaderBuilder::new().has_headers(true).from_path("data/WorldData.csv") {
        Ok(x) => x,
        Err(_x) => panic!("Cannot read the input file")
    };
    for result in reader.deserialize::<T>() {
        let record = match result {
            Ok(x) => x,
            Err(x) => panic!("{:?}", x)
        };
        list.push(record);
    }
    println!("Finished reading the file");
    list
}


fn main() {
    let countries = read_file::<Country>();
    let mut people: Vec<Person> = Vec::new();
    for country in countries {
        println!("Populating {}", &country.name);
        for _i in 1..country.population {
            people.push(Person::new());    
        }
        println!("Populated");
    }
    println!("loaded country data");
    /*for i in 1..POPULATION {
        people.push(Person::new(String::from("John"), false));
    }*/
}
