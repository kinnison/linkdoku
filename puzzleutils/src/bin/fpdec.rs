use puzzleutils::fpuzzles;

fn main() {
    let mut puzzles: Vec<_> = std::env::args().skip(1).collect();
    if puzzles.is_empty() {
        puzzles.extend(std::io::stdin().lines().map(|l| l.unwrap()));
    }
    for puzzle in puzzles {
        if let Some(data) = fpuzzles::extract(&puzzle) {
            println!("{}", serde_json::to_string(&data).unwrap());
        } else {
            eprintln!("BAD: {}", puzzle);
        }
    }
}
