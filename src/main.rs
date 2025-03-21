use std::env;

use args::{run, Config};

mod model;
mod utils;
mod manga;
mod args;
mod sources;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    println!("blt-dl");
    let args = env::args();
    let new_args = Config::new(args)?;
    run(new_args)?;
    Ok(())
}
