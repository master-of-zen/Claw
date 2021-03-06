use serde_json::Value;
use std::{path::PathBuf, process::Command, str::FromStr};

pub fn run_probe(path: PathBuf) -> Vec<(String, String, u64)> {
    let mut ffprobe = Command::new("ffprobe");
    ffprobe.args(&[
        "-threads",
        "24",
        "-show_frames",
        "-select_streams",
        "v",
        "-sexagesimal",
        "-print_format",
        "json",
    ]);

    ffprobe.arg(path.as_os_str());

    let out = ffprobe.output().expect("Failed to execute ffprobe");

    let stdout = String::from_utf8_lossy(&out.stdout);

    let data = parse_ffprobe_json(stdout.as_ref().to_string());

    data
}

pub fn get_duration(path: PathBuf) -> String {
    let mut ffprobe = Command::new("ffprobe");
    ffprobe.args(&[
        "-v",
        "error",
        "-show_entries",
        "format=duration",
        "-sexagesimal",
        "-of",
        "default=noprint_wrappers=1:nokey=1",
    ]);

    ffprobe.arg(path.as_os_str());

    let out = ffprobe.output().unwrap();
    let mut data = String::from_utf8_lossy(&out.stdout).to_string();

    data.truncate(data.len() - 4);

    data
}

fn parse_ffprobe_json(stdout: String) -> Vec<(String, String, u64)> {
    let js = Value::from_str(&stdout).unwrap();

    let js = js.get("frames").unwrap();

    let js = js.as_array().unwrap();

    let dt = js.clone();

    let data = dt
        .clone()
        .iter()
        .map(|x| {
            let start_time = x
                .get("best_effort_timestamp_time")
                .unwrap()
                .as_str()
                .unwrap();

            let start_time = &start_time[..start_time.len() - 3];
            let start_time = start_time.to_owned();

            let is_keyframe = x.get("key_frame").unwrap().as_u64().unwrap();

            let pict_type = x.get("pict_type").unwrap().as_str().unwrap().to_string();

            (start_time, pict_type, is_keyframe)
        })
        .collect::<Vec<(String, String, u64)>>();

    data
}
