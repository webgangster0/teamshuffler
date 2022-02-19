use std::process;

use teamshuffler::app::App;

fn main() {
    let app = match App::new() {
        Ok(app) => app,
        Err(message) => {
            println!("{}", message);
            process::exit(1);
        }
    };

    app.process_args().unwrap_or_else(|error| {
        println!("{}", error);
        process::exit(1);
    });
}
