use litegrep::Config;
use std::{
    env,
    io::{stdout, Write},
    process,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        eprintln!("litegrep [pattern] [filepath|-]");
        process::exit(1);
    });

    let mut writer: Box<dyn Write> = Box::new(stdout());

    config.execute(&mut writer).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
}
