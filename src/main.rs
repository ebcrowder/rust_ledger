mod cli;
mod error;
mod model;

fn main() -> Result<(), std::io::Error> {
    cli::run()
}
