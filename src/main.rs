use std::collections::{HashMap, VecDeque};
use std::io::{Error, Read};
use std::fs::File;
use std::thread;

fn tuples() {
    let location = ("KCLE", 41.4094069, -81.8646911);
    println!("Location Name: {}. latitude: {}, longitude: {}", location.0, location.1, location.2);
    let (name, latitude, longitude) = location; // destructurar
    println!("Location Name: {}. latitude: {}, longitude: {}", name, latitude, longitude);
}

fn array() {
    let location:[f32;2] = [ 41.4094069, -81.8646911 ];
    println!("Location latitude: {}, longitude: {}", location[0], location[1]);
}
fn strings(){
    // &str: inmutable, pueden estrar en el heap stack o embebidos en el codigo (string literal)
    // String: mutable, solo está en el heap
    let name_slice: &str = "Juan Perez";
    let name_string: String = "Juan Perez".to_string();
    let name_string_from: String = String::from("Juan Perez");
    let name_str_from_string: &str = name_string.as_str();
    println!("&str: {}, String: {}, String_from:{}, String as &str: {}",
             name_slice, name_string, name_string_from, name_str_from_string);
}

fn string_op(){
    let airline = "Aerolineas";
    let argentine = "Argentinas";

    // concatenar usando array
    let concat_airline_name = [airline, " ", argentine].concat();
    // usando la macro format!
    let format_airline_name = format!("{} {}", airline, argentine);
    println!("Concatenando: {}, Formateando: {}", concat_airline_name, format_airline_name);
    // usando mut String
    let mut slogan = String::new();
    slogan.push_str("We hit the groud");
    slogan.push(' ');
    slogan = slogan + "every time";
    println!("{}", slogan);
}

fn variables(){
    // casteo
    let el_float: f32 = 17.2;
    let el_byte: u8 = 5;
    // castear de byte a float32 para poder hacer la operacion
    let byte_casteado = el_byte as f32;
    let result = el_float / byte_casteado;
    println!("{}", result);
}

fn scopes(){
    let scope_test = "outer scope";
    println!("{}", scope_test);
    {
        let scope_test = "inner scope";
        println!("{}", scope_test);
    }
    println!("{}", scope_test);
}

fn operators(){
    // Exponente
    let squared = i32::pow(8, 2);
    println!("{}", squared);
    let float_integer = f32::powi(6.5, 3);
    let float_float = f32::powf(6.5, 3.0);
    println!("float to int:{} float to float:{}", float_integer, float_float);
}

fn distance(){
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

    let route = [
        ("KCLE", 41.4075, -81.851111),
        ("LEYIR", 41.51030, -83.8808),
        ("PIONS", 41.6539, -84.4819),
        // ...
        ("PUDVY", 41.5427, -109.3420),
        ("WEGEM", 41.4456, -109.99),
        ("KSLC", 40.7861, -111.9822)
    ];

    let mut total_distance = 0.0;
    let mut previous_waypoint: Option<(&str, f64, f64)> = None; // seria como una tupla nulleable

    for waypoint in route.iter() {
        match previous_waypoint {
            None => {
                previous_waypoint = Option::from(waypoint.clone());
                continue;
            }
            Some(previous_waypoint_value) => {
                let previous_waypoint_radians = previous_waypoint_value.1.to_radians();
                let waypoint_radians = waypoint.1.to_radians();

                let delta_latitude = (previous_waypoint_value.1 - waypoint.1).to_radians();
                let delta_longitude = (previous_waypoint_value.2 - waypoint.2).to_radians();

                let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
                    + previous_waypoint_radians.cos()
                    * waypoint_radians.cos()
                    * f64::powi((delta_longitude / 2.0).sin(), 2 );
                let central_angle = 2.0 *inner_central_angle.sqrt().asin();
                let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;
                total_distance += distance;
                previous_waypoint = Option::from(waypoint.clone());
                println!("The distance between {} and {} is {:.1} kilometers",
                         previous_waypoint_value.0 , waypoint.0, distance);
            }
        }
    }
    println!("The total distance is {:.1} kilometers", total_distance);
}

fn options() {
    // Option se usa para indicar si algo tiene valor o no (no existe el null)
    let phrase = String::from("Aerolineas Bolivianas");
    let letter = phrase.chars().nth(5);

    let value = match letter {
        Some(character) => character.to_string(),
        None => String::from("No value")
    };

    println!("The 5th char from '{}' is '{}'", phrase, value);
}

fn match1(){
    let animal = "Duck";
    match animal {
        "Duck" => println!("Quak!"),
        "Dog" => println!("Bark!"),
        _ => println!("All quiet out here...") // Default
    }
}

fn match2(){
    let ndb_freq: u16 = 384;
    match ndb_freq {
        ndb_freq if ndb_freq >= 200 && ndb_freq <= 500 => {
            println!("NDB frequency is valid")
        }
        _ => println!("NDB frequency is not valid!")
    }
}

enum NavigationAids {
    NDB(u16),
    VOR(String, f32),
    VORDME(String, f32),
    FIX{name: String, latitude:f32, longitude:f32}
}

fn print_navaids(navaid: &NavigationAids){
    match navaid {
        NavigationAids::NDB(khz) => {
            println!("NDB frequency is {} kilohertz", khz);
        }
        NavigationAids::VOR(id, freq) => {
            println!("VOR identifier is {} and frequency is {} kilohertz", id, freq);
        }
        NavigationAids::VORDME(id, freq) => {
            println!("VORDME identifier is {} and frequency is {} kilohertz", id, freq);
        }
        NavigationAids::FIX{name, latitude, longitude } => {
            println!("FIX {} is at {} latitude and {} longitude", name, latitude, longitude);
        }
    }
}

fn enums(){
    let ndb_uwl = NavigationAids::NDB(385);
    let vor_dqn = NavigationAids::VOR(String::from("DQN"), 114.5);
    let vor_dme_sgh = NavigationAids::VORDME(String::from("SGH"), 113.2);
    let fix_tarry = NavigationAids::FIX {
        name: String::from("TARRY"),
        latitude: 40.05333,
        longitude: -83.91367
    };
    print_navaids(&ndb_uwl);
    print_navaids(&vor_dqn);
    print_navaids(&vor_dme_sgh);
    print_navaids(&fix_tarry);
}

fn loops(){
    let mut counter = 0;
    loop{
        counter += 1;
        println!("{}", counter);
        if counter == 3 {
            break;
        }
    }
    while counter < 6 {
        counter += 1;
        println!("{}", counter);
    }
    for index in 7..10 { // tambien se puede escribir 7..=9 para que tome el ultimo valor
        println!("{}", index);
    }
    let aircrafts = ["Boing", "Airbus", "Embraer"];
    for aircraft in aircrafts.iter() {
        println!("{}", aircraft);
    }
}

fn borrowing() {
    let mut original = String::from("original value");
    println!("outer original value:\t\"{}\"", original);
    {
        let next = &mut original;   // se pone &mut para indicar que es un puntero a la var
        *next = String::from("next value"); // se usa * para obtener el puntero como en C y cambiar la var
        println!("inner scope next:\t\t\"{}\"", next);
        println!("inner scope original:\t\"{}\"", original);
    }   // next solo existe en el scope, mientras esté valido el scope original no puede cambiar,
        // porque fue "prestada" a next.
    println!("outer original value:\t\"{}\"", original);
}

// Se puede expecificar el tiempo de vida de cada una de las vars usando el 'a
fn explicit_lifetime_fn<'a>(p1: &'a i32, p2: &'a i32) -> &'a i32{
    if p1 > p2 {
        p1
    }
    else {
        p2
    }
}
fn lifetime(){
    let value_one = 24;
    let value_two = 67;
    let value = explicit_lifetime_fn(&value_one, &value_two);
    println!("value is {}", value);
}

fn closures(){
    let name = "Aerolineas Argentinas";
    let write_message = |slogan: String| -> String { // parametros entre |
        String::from(format!("{}. {}", name, slogan))            // retorna como funcion sin ;
    };
    let phrase = write_message(String::from("Te llevamos donde queres."));

    println!("{}", phrase);
}

// Error propagation con el ? propagamos hacia arriba el Err
fn read_file(filename: &str) -> Result<String, Error> {
    let mut file_handle = File::open(filename)?;
    let mut file_data = String::new();
    file_handle.read_to_string(&mut file_data)?;
    Ok(file_data)
}
fn error_handling() {
    let filename = "C:\\Temp\\ExampleData.txt";
    let file_data = read_file(filename);
    match file_data {
        Ok(data) => {
            println!("{}", data);
        }
        Err(_) => {
            println!("Error");
        }
    }
}

fn vectors() {
    // Vectors
    let vec_macro = vec![1,2,3]; // generado con la macro
    println!("{}", vec_macro[0]);

    let mut flights:Vec<&str> = Vec::new();
    flights.push("DA113\tto Boston departs at 06:20");
    flights.push("DA41\tto Berlin departs at 15:30");
    flights.push("DA2815\tto Nashville departs at 17:11");

    // inserta en posicion 1
    flights.insert(1, "DA918\tto Orlando departs at 11:12");

    for flight in flights.iter() {
        println!("{}", flight);
    }
    // if let y get para ejecutar si hay valor
    if let Some (flight_value) = flights.get(2) {
        println!("{}", flight_value);
    }
}

//
fn double_ended_queue(){
    let mut flights:VecDeque<&str> = VecDeque::new();
    flights.push_front("DA918\tto Orlando departs at 11:12");
    flights.push_back("DA428\tto Slat Lake CCity departs at 12:05");
    flights.push_front("DA98\tto London departs at 09:43");
    flights.push_front("DA113\tto Boston departs at 06:20");

    // si quisieramos mutarlo, existe el iter_mut
    for flight in flights.iter() {
        println!("{}", flight);
    }
}

fn maps() {
    let mut flights = HashMap::new();

    flights.insert("DA918", ("11:12","Orlando"));
    flights.insert("DA428", ("12:05","Slat Lake City"));
    flights.insert("DA98", ("09:43","London"));
    flights.insert("DA113", ("06:20","Boston"));

    let flight_number = "DA428";
    let option = flights.get(flight_number);
    let (time,destination) = option.unwrap();
    println!("{} {} {}", flight_number, time, destination);

    let flight_number_insert = "DA531";
    if !flights.contains_key(flight_number_insert){
        flights.insert(flight_number, ("12:00", "Puerto Rico"));
    } else {
        println!("Flight {} is already entered!", flight_number_insert);
    }

    for flight in flights.iter() {
        println!("{:?}", flight);
    }

}

fn threads() {
    let outer_scope = 412;

    let join_handle = thread::spawn(move || {
       outer_scope * 2
    });

    let result = join_handle.join();
    match result {
        Ok(value) => {
            println!("thread result {}", value);
        }
        Err(_) => {}
    }
}

fn main() {
    tuples();
    array();
    strings();
    string_op();
    variables();
    scopes();
    operators();
    distance();
    options();
    match1();
    match2();
    enums();
    loops();
    borrowing();
    lifetime();
    closures();
    error_handling();
    vectors();
    double_ended_queue();
    maps();
    threads();
}

