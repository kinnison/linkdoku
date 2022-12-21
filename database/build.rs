// In order to ensure migration changes rebuild the crate

fn main() {
    println!("cargo:rerun-if-changed=migrations");
}
