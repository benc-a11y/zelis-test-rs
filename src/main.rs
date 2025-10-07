use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::process;

const BUFFER_SIZE: usize = 8192;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match parse_args(&args) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error: {}", err);
            print_usage(&args[0]);
            process::exit(1);
        }
    };

    if let Err(e) = run(config) {
        eprintln!("Error processing file: {}", e);
        process::exit(1);
    }
}

struct Config {
    input: InputSource,
    output: OutputSource,
}

enum InputSource {
    Stdin,
    File(String),
}

enum OutputSource {
    Stdout,
    File(String),
}

fn parse_args(args: &[String]) -> Result<Config, String> {
    let mut input = InputSource::Stdin;
    let mut output = OutputSource::Stdout;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--input" => {
                if i + 1 >= args.len() {
                    return Err("--input requires a file path".to_string());
                }
                input = InputSource::File(args[i + 1].clone());
                i += 2;
            }
            "--output" => {
                if i + 1 >= args.len() {
                    return Err("--output requires a file path".to_string());
                }
                output = OutputSource::File(args[i + 1].clone());
                i += 2;
            }
            "--help" | "-h" => {
                return Err("help requested".to_string());
            }
            _ => {
                return Err(format!("Unknown argument: {}", args[i]));
            }
        }
    }

    Ok(Config { input, output })
}

fn print_usage(program_name: &str) {
    eprintln!("Usage: {} [OPTIONS]", program_name);
    eprintln!();
    eprintln!("JSON corruption correction tool - replaces semicolons with colons");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --input <file>   Read from file instead of STDIN");
    eprintln!("  --output <file>  Write to file instead of STDOUT");
    eprintln!("  --help, -h       Show this help message");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  {} < input.json > output.json", program_name);
    eprintln!("  {} --input input.json --output output.json", program_name);
}

fn run(config: Config) -> io::Result<()> {
    let reader: Box<dyn Read> = match config.input {
        InputSource::Stdin => Box::new(io::stdin()),
        InputSource::File(path) => Box::new(File::open(path)?),
    };

    let writer: Box<dyn Write> = match config.output {
        OutputSource::Stdout => Box::new(io::stdout()),
        OutputSource::File(path) => Box::new(File::create(path)?),
    };

    let mut buf_reader = BufReader::with_capacity(BUFFER_SIZE, reader);
    let mut buf_writer = BufWriter::with_capacity(BUFFER_SIZE, writer);

    let mut buffer = [0u8; BUFFER_SIZE];

    loop {
        let bytes_read = buf_reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        for byte in &mut buffer[..bytes_read] {
            if *byte == b';' {
                *byte = b':';
            }
        }

        buf_writer.write_all(&buffer[..bytes_read])?;
    }

    buf_writer.flush()?;
    Ok(())
}
