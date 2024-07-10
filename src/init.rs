use crate::cli::Args;
use crate::state::State;
use std::io::Read;

pub fn init(args: &Args) -> std::io::Result<State> {
    let (file, is_readonly) = read_file(&args.file)?;

    Ok(State {
        file,
        is_readonly: is_readonly || args.readonly,
    })
}

fn read_file(path: &std::path::Path) -> std::io::Result<(String, bool)> {
    let mut file = std::fs::File::open(path)?;

    let meta = file.metadata()?;
    let readonly = meta.permissions().readonly();

    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    Ok((buf, readonly))
}
