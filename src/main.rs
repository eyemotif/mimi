mod cli;
mod draw;
mod init;
mod input;
mod state;

fn main() {
    let args = <cli::Args as clap::Parser>::parse();

    let (mut state, mut terminal) = match init::init(&args) {
        Ok(it) => it,
        Err(err) => {
            eprintln!("Error initializing: {err}");
            return;
        }
    };

    loop {
        match terminal.draw(|frame| draw::draw(frame, &mut state)) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error drawing frame: {err}");
                break;
            }
        }

        match input::handle_input(&mut state) {
            Ok(event) => match event {
                input::InputEvent::NoOp => (),
                input::InputEvent::Exit => break,
            },
            Err(err) => {
                eprintln!("Error reading key: {err}");
                break;
            }
        }
    }

    init::deinit(terminal).expect("Deinit should return successfully");
}
