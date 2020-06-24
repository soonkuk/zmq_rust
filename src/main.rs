//  Asynchronous client-to-server (DEALER to ROUTER)
//
//  While this example runs in a single process, that is to make
//  it easier to start and stop the example. Each task has its own
//  context and conceptually acts as a separate process.
// #![crate_name = "asyncsrv"]
extern crate termion;

// use rand::{thread_rng, Rng};
// use std::time::Duration;
use std::{thread};

use clap::{Arg, App};

use termion::{color};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct HealthData {
	location: Location,
	hrv: i8,
	ecg: i8,
	temperature: f32,
    time_stamp: String
}

fn server_task() {
    let context = zmq::Context::new();
    let frontend = context.socket(zmq::ROUTER).unwrap();
    frontend.set_rcvhwm(0).expect("failed to setting hwm");
    frontend
        .bind("tcp://*:54325")
        .expect("server failed binding frontend");
    let backend = context.socket(zmq::DEALER).unwrap();
    backend
        .bind("inproc://backend")
        .expect("server failed binding backend");
    for _ in 0..1000 {
        let ctx = context.clone();
        thread::spawn(move || server_worker(&ctx));
    }
    zmq::proxy(&frontend, &backend).expect("server failed proxying");
}

fn server_worker(context: &zmq::Context) {
    let worker = context.socket(zmq::DEALER).unwrap();
    worker
        .connect("inproc://backend")
        .expect("worker failed to connect to backend");
    // let mut rng = thread_rng();

    loop {
        let identity = worker
            .recv_string(0)
            .expect("worker failed receiving identity")
            .unwrap();
        let message = worker
            .recv_string(0)
            .expect("worker failed receiving message")
            .unwrap();
        // println!("{}", &message);
        let res: HealthData = serde_json::from_str(&message).unwrap();
        // let mut decoder = Decoder::new(&message_bytes[..]);
        // let res: Query = Decodable::decode(&mut decoder).unwrap();
        let str: &str = "[success]";
        // thread::sleep(Duration::from_millis(rng.gen_range(0, 100) + 1));
        worker
            .send(&identity, zmq::SNDMORE)
            .expect("worker failed sending identity");
        worker
            .send(&str, 0)
            .expect("worker failed sending message");
        message_parser(res, identity);
    }
}

fn message_parser(data: HealthData, id: String) {
    let v: Vec<&str> = id.split("||").collect();

    if data.temperature > 37.5 {
        println!("{}", color::Fg(color::Red));
        println!("{} : {:?}", &id, &data);
        println!("{}", color::Fg(color::White));
    }
    match v[0] {
        "blue" => {
            print!("{}", color::Fg(color::Blue));
            print!("{}", id);
            print!("{}", color::Fg(color::White));
            println!(" : {:?}", &data);
        },
        "yellow" => {
            print!("{}", color::Fg(color::Yellow));
            print!("{}", id);
            print!("{}", color::Fg(color::White));
            println!(" : {:?}", &data);
        },
        "green" => {
            print!("{}", color::Fg(color::Green));
            print!("{}", id);
            print!("{}", color::Fg(color::White));
            println!(" : {:?}", &data);
        },
        "magenta" => {
            print!("{}", color::Fg(color::Magenta));
            print!("{}", id);
            print!("{}", color::Fg(color::White));
            println!(" : {:?}", &data);
        },
        "cyan" => {
            print!("{}", color::Fg(color::Cyan));
            print!("{}", id);
            print!("{}", color::Fg(color::White));
            println!(" : {:?}", &data);
        },
        _ => println!("{} : {:?}", &id, &data),
    }
}

fn main() {
    let matches = App::new("zmq_rust")
    .version("1.0")
    .author("Kevin K. <kbknapp@gmail.com>")
    .about("zmq server")
    .subcommand(App::new("server")
        .about("server features")
        .version("1.3")
        .author("Someone E. <someone_else@other.com>")
        .arg(Arg::new("run").short('r').about("run server")))
    .get_matches();
    
    if let Some(ref matches) = matches.subcommand_matches("server") {
        // "$ zmq server" was run
        if matches.is_present("run") {
            // "$ zmq server -r" was run
            let handle = thread::spawn(server_task);
            handle.join().unwrap();
        } else {
            println!("type command zmq server run");
        }
    }
}

/*
fn build_query(location: Location, hrv: u8, ecg: u8, temperature: f32, timestamp: String) -> Query {
    Query {
        location: location,
        hrv: hrv,
        ecg: ecg,
        temperature: temperature,
        time_stamp: timestamp
    }
}
*/
#[derive(Debug, Serialize, Deserialize)]
struct Location {
	latitude:  f32,
	longitude: f32,
}
/*
fn build_location(latitude: f32, longitude: f32) -> Location {
    Location {
        latitude: latitude,
        longitude: longitude
    }
}
*/
