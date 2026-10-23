use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Options {
    #[command(subcommand)]
    pub mode: Mode,
    /// Path to the output file
    #[arg(short, long)]
    pub output: String,
    /// Your Danbooru ID
    #[arg(short, long)]
    pub id: u64,
}

#[derive(Subcommand, Debug)]
pub enum Mode {
    /// Loading artworks for default mode
    Aily(AilyModeOptions),

    /// Loading tags for aily-tags mode
    Tags(TagsModeOptions),
}

#[derive(Args, Debug)]
pub struct AilyModeOptions {
    /// Count of arts to be added to the dataset
    #[arg(short, long, default_value_t = 3000)]
    pub count: u32,
}

#[derive(Args, Debug)]
pub struct TagsModeOptions {
    /// Database connection string
    #[arg(short, long)]
    pub database: String,
}
