use clap::Parser;
#[derive(Parser, Clone, Debug)]
pub struct CommandArgument {
    #[clap(short, long)]
    pub day: Option<u32>,

    #[clap(short, long)]
    pub year: Option<i32>,

    pub part: Option<u32>,

    #[clap(long)]
    pub input_filename: Option<String>,

    #[clap(short, long)]
    pub publish: bool,
    // Maybe in the future
    //#[clap(long)]
    //pub all: bool,
}
