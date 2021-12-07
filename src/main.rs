use serde_json::{Result, Value};
use std::fmt::Error;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[clap(short, parse(from_os_str), required = true)]
    pub input: PathBuf,

    /// Mode of operation
    #[clap(long)]
    mode: String,

    /// Planes to extract
    #[clap(long)]
    planes: Option<String>,

    /// Output
    #[clap(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

    let input = args.input;
    let js = read_json_file(input);

    // get frames

    let js = js.get("frames").unwrap().as_array().unwrap();

    let vmaf: Vec<(u64, f64)> = js
        .into_iter()
        .map(|x| {
            let frame_num = x.get("frameNum").unwrap().as_u64().unwrap();
            let vmaf = x
                .get("metrics")
                .unwrap()
                .get("vmaf")
                .unwrap()
                .as_f64()
                .unwrap();
            (frame_num, vmaf)
        })
        .collect::<Vec<(u64, f64)>>();

    println!("{:#?}", vmaf);
}

fn read_json_file(path: PathBuf) -> Value {
    // Open the file in read-only mode with buffer.
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let dt: Value = serde_json::from_reader(reader).unwrap();
    dt
}

fn vmaf_subtitles() {
    ()
}
