use std::io::Write;

mod cli;
mod init;
mod input;
mod state;

fn main() {
    let args = <cli::Args as clap::Parser>::parse();

    let mut state = match init::init(&args) {
        Ok(it) => it,
        Err(err) => {
            eprintln!("Error initializing: {err}");
            return;
        }
    };

    println!("Text");

    loop {
        match input::handle_input(&mut state) {
            Ok(event) => match event {
                input::InputEvent::NoOp => (),
                input::InputEvent::Exit => break,
            },
            Err(err) => eprintln!("Error reading key: {err}"),
        }
    }

    init::deinit(state).expect("Deinit should return successfully");
}
