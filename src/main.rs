use serde_json::Value;

mod ffprobing;

use std::fs::{self, File};
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

    /// Planes to extqract
    #[clap(long)]
    pub planes: Option<String>,

    /// Output
    #[clap(short, long)]
    pub output: Option<String>,
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

    let data = ffprobing::run_probe(args.video_input.clone());

    let mut all_time_codes: Vec<String> = data.iter().map(|(x, _y, _z)| x.clone()).collect();

    let duration = crate::ffprobing::get_duration(args.video_input.clone());
    all_time_codes.push(duration.clone());

    let start_finish: Vec<(String, String)> = all_time_codes[..all_time_codes.len() - 1]
        .iter()
        .zip(all_time_codes[1..].into_iter())
        .map(|(x, y)| (x.clone(), y.clone()))
        .collect();

    let frames: Vec<String> = vmaf
        .iter()
        .zip(start_finish.iter())
        .map(|(x, y)| make_srt_string(x.0, y.0.clone(), y.1.clone(), x.1))
        .collect();

    let srt = frames.join("");

    dbg!(frames);
    let destination = args.video_input.with_extension("srt");
    fs::write(destination, srt).expect("Unable to write file");
}

pub fn make_srt_string(counter: u64, start_time: String, end_time: String, vmaf: f64) -> String {
    let timestamp = format!("{} --> {}", start_time, end_time).replace('.', ",");
    let subtitle = format!("Frame: {}, Vmaf: {}\n\n", counter, vmaf);

    format!("{}\n{}\n{}", counter, timestamp, subtitle)
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
