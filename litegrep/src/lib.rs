use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

pub struct Config {
    pattern: String,
    input: Box<dyn BufRead>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let pattern = args.get(1).ok_or_else(|| "Not enough arguments")?;
        let filepath = args.get(2).ok_or_else(|| "Not enough arguments")?;

        let input: Box<dyn BufRead> = if filepath == "-" {
            Box::new(BufReader::new(io::stdin()))
        } else {
            let file = File::open(filepath)
                .or_else(|err| return Err(err.to_string()))
                .unwrap();
            Box::new(BufReader::new(file))
        };

        Ok(Config {
            pattern: pattern.clone(),
            input,
        })
    }

    pub fn execute(self, writer: &mut Box<dyn Write>) -> Result<(), io::Error> {
        // pub fn execute(self) -> Result<(), io::Error> {
        for line in self.input.lines() {
            match line {
                Ok(read) => {
                    if read.contains(&self.pattern) {
                        writer.write_all(&read.as_bytes()).unwrap();
                        writer.write_all(&"\n".as_bytes()).unwrap();
                    }
                }
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }
}
