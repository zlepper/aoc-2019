use std::{fs, env};
use http::{Request, Uri};
use isahc;
use isahc::ResponseExt;
use std::path::Path;

pub trait AocImplementation<T> {

    fn start(&self, day: i32) {
        download_input_file(day);
        let contents = fs::read_to_string(get_day_filename(day)).expect("Failed to read input file");

        let parsed = self.process_input(&contents);

        let answer = self.execute(parsed);

        match answer {
            Some(a) => println!("Puzzle answer: {}", a),
            None => eprintln!("Failed to calculate answer")
        }
    }

    fn process_input(&self, input: &str) -> Vec<T>;
    fn execute(&self, input: Vec<T>) -> Option<i32>;
}

fn get_day_filename(day: i32) -> String {
    format!("day{}/input.txt", day)
}

// Ensures the input file exists, and downloads it if not
fn download_input_file(day: i32) {
    let filename = get_day_filename(day);

    if Path::new(&filename).exists() {
        return
    }

    println!("Input file does not exist, downloading before running");

    let cookie = env::var("AOC_SESSION_COOKIE").expect("AOC_SESSION_COOKIE env variable was not set");

    let url: Uri = format!("https://adventofcode.com/2019/day/{}/input", day).parse().unwrap();


    let request = Request::builder()
        .uri(url)
        .method("GET")
        .header("cookie", format!("session={}", cookie))
        .body(()).unwrap();

    let content = isahc::send(request).unwrap().text().unwrap();

    fs::write(&filename, content.trim()).unwrap();
}