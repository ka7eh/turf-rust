use geojson::GeoJson;
use std::env::current_dir;
use std::ffi::OsStr;
use std::fs::{read_dir, File};
use std::path::PathBuf;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_file(file_path: &PathBuf) -> String {
    let file = File::open(file_path).unwrap();
    let mut buff_reader = BufReader::new(file);
    let mut content = String::new();
    buff_reader.read_to_string(&mut content).unwrap();
    content
}

pub fn get_geojsons() -> Vec<(GeoJson, GeoJson)> {
    let cwd = current_dir().unwrap();
    let fixtures_path = cwd.join("src").join("fixtures");
    let in_path = fixtures_path.join("in");
    let out_path = fixtures_path.join("out");

    let mut collection = Vec::new();

    for entry in read_dir(in_path).unwrap() {
        let entry = entry.unwrap();
        let input_path = entry.path();
        if input_path.extension().and_then(OsStr::to_str) == Some("geojson") {
            let input_content = read_file(&input_path);
            let input_geojson = input_content.parse::<GeoJson>().unwrap();

            let output_path = out_path.join(input_path.file_name().unwrap());
            let output_content = read_file(&output_path);
            let output_geojson = output_content.parse::<GeoJson>().unwrap();

            collection.push((input_geojson, output_geojson));
        }
    }

    collection
}
