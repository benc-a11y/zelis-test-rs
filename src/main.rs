use std::io::{self, BufReader, BufWriter, Read, Write};

fn process_stream<R: Read, W: Write>(input: R, output: W) -> io::Result<()> {
    let mut reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);
    let mut buffer = [0u8; 8192];

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }

        for byte in &mut buffer[..n] {
            if *byte == b';' {
                *byte = b':';
            }
        }

        writer.write_all(&buffer[..n])?;
    }
    writer.flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    process_stream(stdin.lock(), stdout.lock())?;

    Ok(())
}
