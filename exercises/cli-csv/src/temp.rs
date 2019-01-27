// use std::fs::File;
// use std::error::Error;

// struct City {
//     country: String,
//     city: String,
//     accent_city: String,
//     region: i32,
//     population: i32,
//     latitude: f32,
//     longitude: f32
// }

// struct args {
//     args: Vec
// }

// struct CitiesProvider {
//     cities: Vec<City>
// }

// impl CitiesProvider {
//     fn add_city(&mut self, _city: City) -> &mut Self {
//         self.cities.push(_city);
        
//         self
//     }
    
//     fn apply_filters(&mut self, _filters: &[fn(&City) -> bool]) -> Vec<&City> {
        
//         let mut result : Vec<&City> = Vec::new();
        
//         for _city in &self.cities {
            
//             let mut f : bool = true;
            
//             for _filter in _filters {
//                 if !_filter(&_city) {
//                     f = false
//                 }
//             }
            
//             if f {
//                 result.push(&_city);
//             }
            
//         }
        
//         result
//     }
// }

// fn read_data(file_name: String) -> Result<(CitiesProvider), Box<Error>> {
    
//     let file = File::open(file_name).expect("file not found");
//     let mut rdr = csv::Reader::from_reader(file);

//     let mut cp = CitiesProvider {
//         cities: vec![],
//     };
    
//     for result in rdr.records() {
        
//         let _record = result?;
        
//         let _country : String = String::from(&_record[0]);
//         let _city_name : String = String::from(&_record[1]);
//         let _accent_city : String = String::from(&_record[2]);
//         let _latitude : f32 = String::from(&_record[5]).parse::<f32>().unwrap();
//         let _longitude : f32 = String::from(&_record[6]).parse::<f32>().unwrap();
        
//         let mut _population : i32 = 0;
//         let mut _region : i32 = 0;
        
//         match String::from(&_record[3]).parse::<i32>() {
//             Ok(v) => { 
//                 _region = v as i32;
//             }
//             Err(_e) => {
//                 _region = -1;
//             }
//         }
        
//         match String::from(&_record[4]).parse::<i32>() {
            
//             Ok(_pop) => { 
//                 _population = _pop as i32;
//             }
//             Err(_e) => {
//                 _population = -1 as i32;
//             }
//         }
        
//         let _city : City = City {
//             country: _country,
//             city: _city_name,
//             accent_city: _accent_city,
//             region: _region,
//             population: _population,
//             latitude: _latitude,
//             longitude: _longitude
//         };
        
//         cp.add_city(_city);
    
//     }   
    
//     Ok(cp)
    
// }

// fn population_not_none(_city : &City) -> bool {
//     _city.population > -1
// }

// fn population_none(_city : &City) -> bool {
//     _city.population == -1
// }

// fn population_more_then(_city : &City) -> bool {
//     _city.population > 3300
// }

// fn main() -> Result<(), Box<Error>> {
//     let mut r = read_data(String::from("worldcitiespop.csv"))?;
//     let results = r.apply_filters(&[population_not_none]);
    
//     for c in results {
//         println!("{:?} {:?}", c.city, c.population);                
//     }
//     Ok(())
// }