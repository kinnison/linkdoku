use std::path::Path;

use css_minify::optimizations::{Level, Minifier};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for base in ["dark.sass", "light.sass"] {
        let css = grass::from_path(base, &grass::Options::default())?;
        let mincss = Minifier::default()
            .minify(&css, Level::One)
            .map_err(|e| format!("{:?}", e))?;
        let outname = Path::new(base)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .replace(".sass", ".min.css");
        std::fs::write(outname, mincss)?;
    }

    Ok(())
}
