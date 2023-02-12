fn main() {
    let example = std::io::stdin().lines().next().unwrap().unwrap();
    let example = example.trim();

    let decoded = puzzleutils::ctc::parse_str(example).unwrap();
    println!("{:#?}", puzzleutils::ctc::metadata(&decoded));
}
