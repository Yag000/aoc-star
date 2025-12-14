use clap::Parser;
#[derive(Parser, Clone, Debug)]
pub struct CommandArgument {
    #[clap(short, long)]
    pub day: u32,

    #[clap(short, long)]
    pub part: Option<u32>,

    #[clap(short, long)]
    pub year: Option<i32>,

    #[clap(long)]
    pub input_file: Option<String>,

    #[clap(long)]
    pub publish: bool,
    // Maybe in the future
    //#[clap(long)]
    //pub all: bool,
}
