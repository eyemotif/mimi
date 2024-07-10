use std::io::Read;

mod cli;

fn main() {
    let args = <cli::Args as clap::Parser>::parse();

    let (file, readonly) = match read_file(&args.file) {
        Ok(it) => it,
        Err(err) => {
            eprintln!(
                "Error opening file \"{}\": {err}",
                args.file.to_string_lossy()
            );
            return;
        }
    };

    let readonly = readonly || args.readonly;

    println!("{}{file:?}", if readonly { "readonly: " } else { "" });
}

fn read_file(path: &std::path::Path) -> std::io::Result<(String, bool)> {
    let mut file = std::fs::File::open(path)?;

    let meta = file.metadata()?;
    let readonly = meta.permissions().readonly();

    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    Ok((buf, readonly))
}
