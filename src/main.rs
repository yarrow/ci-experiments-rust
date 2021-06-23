fn main() -> std::io::Result<()> {
    let mut args = std::env::args();
    let _ = args.next();
    match args.next() {
        None => std::process::exit(1),
        Some(path) => {
            let contents = std::fs::read(path)?;
            print!("{}", String::from_utf8(contents).unwrap());
        }
    };
    Ok(())
}
