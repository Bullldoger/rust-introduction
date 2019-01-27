use std::env;
use std::fs::File;
use std::error::Error;

use std::{
    io::{BufWriter, Write},
};

extern crate clap;

use clap::{Arg, App};

struct Filter {
    filter_arg: i32, 
    func: fn(&City, i32) -> bool
}

impl Filter {
    fn apply_filter(&self, city : &City) -> bool {
        (self.func)(city, self.filter_arg)
    }
}

fn population_greater_then(city : &City, p : i32) -> bool {
    city.population > p
}

fn population_less_then(city : &City, p : i32) -> bool {
    city.population < p
}

struct City {
    country: String,
    city: String,
    accent_city: String,
    region: i32,
    population: i32,
    latitude: f32,
    longitude: f32
}

struct CitiesProvider {
    cities: Vec<City>
}

impl CitiesProvider {
    fn add_city(&mut self, _city: City) -> &mut Self {
        self.cities.push(_city);
        
        self
    }
    
    fn apply_filters(&mut self, _filters: &Vec<Filter>) -> Vec<&City> {
        
        let mut result : Vec<&City> = Vec::new();
        
        for _city in &self.cities {
            
            let mut f : bool = true;
            
            for _filter in _filters {
                if !_filter.apply_filter(_city) {
                    f = false
                }
            }
            
            if f {
                result.push(&_city);
            }
            
        }
        
        result
    }
}

fn read_data(file_name: String) -> Result<(CitiesProvider), Box<Error>> {
    
    let file = File::open(file_name).expect("file not found");
    let mut rdr = csv::Reader::from_reader(file);
    let mut i : i32 = 0;
    
    let mut cp = CitiesProvider {
        cities: vec![],
    };
    
    for result in rdr.records() {
        
        let _record = result?;
        
        let _country : String = String::from(&_record[0]);
        let _city_name : String = String::from(&_record[1]);
        let _accent_city : String = String::from(&_record[2]);
        let _latitude : f32 = String::from(&_record[5]).parse::<f32>().unwrap();
        let _longitude : f32 = String::from(&_record[6]).parse::<f32>().unwrap();
        
        let mut _population : i32 = 0;
        let mut _region : i32 = 0;
        
        match String::from(&_record[3]).parse::<i32>() {
            Ok(v) => { 
                _region = v as i32;
            }
            Err(_e) => {
                _region = -1;
            }
        }
        
        match String::from(&_record[4]).parse::<i32>() {
            
            Ok(_pop) => { 
                _population = _pop as i32;
            }
            Err(_e) => {
                _population = -1 as i32;
            }
        }
        
        let _city : City = City {
            country: _country,
            city: _city_name,
            accent_city: _accent_city,
            region: _region,
            population: _population,
            latitude: _latitude,
            longitude: _longitude
        };
        
        cp.add_city(_city);
    
    }   
    Ok(cp)
}

fn print_TSV(results : Vec<&City>) {
    
    println!("\tcountry\tcity\taccent_city\tregion\tpopulation\tlatitude\tlongitude\t");
    
    for city in results {
        
        println!("\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t", city.country, city.city, 
            city.accent_city, city.region, city.population, city.latitude,
            city.longitude)
        
    }
}

fn print_TSV_to_file(results : Vec<&City>, file_name : String) {
    
    let write_file = File::create(file_name).unwrap();
    let mut writer = BufWriter::new(&write_file);

    write!(&mut writer, "country\tcity\taccent_city\tregion\tpopulation\tlatitude\tlongitude");
    
    for city in results {
        write!(&mut writer, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t\n", city.country, city.city, 
            city.accent_city, city.region, city.population, city.latitude,
            city.longitude);
        
    }
}

fn main() -> Result<(), Box<Error>> {
    let mut filters : Vec<Filter> = vec![];
    
    let matches = App::new("My Test Program")
        .version("0.1.0")
        .author("Hackerman Jones <hckrmnjones@hack.gov>")
        .about("Teaches argument parsing")
        .arg(Arg::with_name("population-not-none")
                 .long("population-not-none")
                 .takes_value(false)
                 .help("will return only cities with known population"))
        .arg(Arg::with_name("population-none")
                 .long("population-none")
                 .takes_value(false)
                 .help("will return cities with unknown population"))
        .arg(Arg::with_name("population-gt")
                 .long("population-gt")
                 .takes_value(true)
                 .help("will return cities with population greater than given number"))
        .arg(Arg::with_name("population-lt")
                 .long("population-lt")
                 .takes_value(true)
                 .help("will return cities with population smaller than given number"))
        .arg(Arg::with_name("out")
                 .long("out")
                 .takes_value(true)
                 .help("will return cities with population smaller than given number"))
        .get_matches();
    
    let num_str = matches.value_of("population-not-none");
    match num_str {
        None => { },
        Some(s) => {
            let f = Filter {
                filter_arg: -1,
                func: population_greater_then,
            };
                    
            filters.push(f);
        }
    }
    
    let num_str = matches.value_of("population-none");
    match num_str {
        None => {
            let f = Filter {
                filter_arg: 0,
                func: population_greater_then,
            };
                    
            filters.push(f);
        },
        Some(s) => {
            let f = Filter {
                filter_arg: 0,
                func: population_less_then,
            };
                    
            filters.push(f);
        }
    }
    
    let num_str = matches.value_of("population-gt");
    match num_str {
        None => { },
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => {
                    let f = Filter {
                        filter_arg: n,
                        func: population_greater_then,
                    };
                    
                    filters.push(f);
                    
                },
                Err(_) => { },
            }
        }
    }
    
    let num_str = matches.value_of("population-lt");
    match num_str {
        None => { },
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => {
                    let f = Filter {
                        filter_arg: n,
                        func: population_less_then,
                    };
                    
                    filters.push(f);
                    
                },
                Err(_) => { },
            }
        }
    }

    let mut cities_controller = read_data(String::from("worldcitiespop.csv"))?;
    let results = cities_controller.apply_filters(&filters);
    
    let num_str = matches.value_of("out");
    match num_str {
        None => {
            print_TSV(results);
        },
        Some(s) => {
            match s.parse::<String>() {
                Ok(file_name) => {
                    print_TSV_to_file(results, file_name);
                },
                Err(_) => { },
            }
        }
    }

    Ok(())
}
