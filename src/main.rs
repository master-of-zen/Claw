mod ffprobing;
mod vmaf_sub;

use clap::Parser;
use std::path::PathBuf;
use vmaf_sub::make_vmaf_subttiles;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[clap(short, parse(from_os_str), required = true)]
    pub input_json: PathBuf,

    /// Input video
    #[clap(short, parse(from_os_str), required = true)]
    pub video_input: PathBuf,

    /// Mode of operation
    #[clap(long)]
    pub mode: Option<String>,

    /// Planes to extqract
    #[clap(long)]
    pub planes: Option<String>,

    /// Output
    #[clap(short, long)]
    pub output: Option<String>,
}

fn main() {
    let args = Args::parse();

    let input_js = args.input_json;
    let input_video = args.video_input;

    make_vmaf_subttiles(input_js, input_video);
}
