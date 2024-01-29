use std::{thread::sleep, time};

fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage {} <time> <url>", args[0]);
        panic!();
    }
    let time_check: u64 = args[1].parse().unwrap();
    let dur = time::Duration::from_secs(time_check);

    let url: &String = &args[2];

   // print!("print unit{:?}", ());

 

    loop {
        let res = reqwest::blocking::get(args[2].clone());
        match res {
            Ok(code) => {
                match code.status() {
                    reqwest::StatusCode::OK => {
                        println!("Checking '{}'. Result: OK({})", url, code.status().as_u16());
                    }
                    _ => {
                        println!(
                            "Checking '{}'. Result: ERR({})",
                            url,
                            code.status().as_u16()
                        );
                    }
                };
            }
            Err(error) => {
                panic!("{}", error);
            }
        };

        sleep(dur);
    }
}
