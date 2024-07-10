mod cli;
mod init;
mod state;

fn main() {
    let args = <cli::Args as clap::Parser>::parse();

    let state = match init::init(&args) {
        Ok(it) => it,
        Err(err) => {
            eprintln!("Error initializing: {err}");
            return;
        }
    };
}
