mod cli;
mod error;
mod model;

fn main() -> Result<(), error::Error> {
    cli::run()
}
