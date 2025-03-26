use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args {:?}", args);


    let config  = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // let contents = fs::read_to_string(&config.filename)
    //     .expect("Something went wrong reading the file");
    // println!("With txt:\n{}", contents);

    if let Err(e) = run(&config){
        println!("Application error: {}", e);
        process::exit(1);
    }
    // run(&config);

}

