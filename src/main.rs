use serde_json::{Result, Value};
use std::fmt::Error;

mod ffprobing;
mod srt;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use clap::Parser;

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

    /// Planes to extract
    #[clap(long)]
    pub planes: Option<String>,

    /// Output
    #[clap(short, long)]
    pub output: Option<String>,
}

struct Frame {
    entry: u64,
    vmaf: f64,
    time_string: String,
    frame_type: String,
}

impl Frame {
    pub fn new(entry: u64, vmaf: f64, time_start: String, time_end: String) {}
}

fn main() {
    let args = Args::parse();

    let input = args.input_json;
    let js = read_json_file(input);

    // get frames

    let js = js.get("frames").unwrap().as_array().unwrap();

    // Literally hazardous code
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

    let data = ffprobing::run_probe(args.video_input);
    println!("{:#?}", &vmaf);
    println!("{:#?}", &data);
}

fn write_file() {
    ()
}

fn read_json_file(path: PathBuf) -> Value {
    // Open the file in read-only mode with buffer.
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let dt: Value = serde_json::from_reader(reader).unwrap();
    dt
}

fn vmaf_subtitles(counter: u64, vmaf: f64) -> String {
    let vmaf_stamp = format!("{}\n", &vmaf);
    vmaf_stamp
}
