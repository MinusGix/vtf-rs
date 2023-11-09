use std::env;
use std::fs;
use vtf::Error;

fn main() -> Result<(), Error> {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: info <path to vtf file>");
    }

    let buf = fs::read(&args[1])?;

    let vtf = vtf::from_bytes(&buf)?;

    println!("{:#?}", vtf.header);
    Ok(())
}
