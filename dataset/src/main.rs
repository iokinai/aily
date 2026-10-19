use std::error::Error;

use clap::Parser;

use crate::{
    loader::Loader,
    options::{Mode, Options},
};

mod aily;
mod load_error;
mod loader;
mod options;
mod tags;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::parse();

    let loader = Loader::new(options.id);

    match options.mode {
        Mode::Aily(ref aily_mode_options) => {
            aily::run(&aily_mode_options, &loader, &options.output).await
        }
        Mode::Tags(ref tags_mode_options) => {
            tags::run(&tags_mode_options, &loader, &options.output).await
        }
    }
}
