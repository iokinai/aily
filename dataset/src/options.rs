use clap::Parser;

#[derive(Parser, Debug)]
pub struct Options {
    /// Count of arts to be added to the dataset
    #[arg(short, long, default_value_t = 3000)]
    pub count: u32,
    /// Path to the output file
    #[arg(short, long, default_value_t = String::from("aily_dataset.sqlite"))]
    pub output: String,
    /// Your Danbooru ID
    #[arg(short, long)]
    pub id: u64,
}
