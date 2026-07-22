use anyhow::Context;

#[derive(Debug, Clone, Copy)]
pub enum DataFormat {
    TextPairs, // Maps u32 -> String (Standard dictionary entries)
    Geometry,  // Maps u32 -> Point (Geometric grids)
}

pub struct Config {
    pub file_path: String,
    pub format: DataFormat,
}

impl Config {
    /// Builds a configuration object out of the system's terminal arguments.
    /// Expects: cargo run <FILE_PATH> <FORMAT_FLAG: "text" | "geo">
    pub fn build(mut args: impl Iterator<Item = String>) -> anyhow::Result<Self> {
        args.next();

        let file_path = args.next().context("Missing filepath")?;

        // Parse your dynamic string argument into your fixed DataFormat enum
        let format = match args
            .next()
            .context("Missing format string")?
            .to_lowercase()
            .as_str()
        {
            "text" => DataFormat::TextPairs,
            "geo" => DataFormat::Geometry,
            other => {
                anyhow::bail!("Unknown format variant flag: '{other}'. Choose 'text' or 'geo'.")
            }
        };

        Ok(Config { file_path, format })
    }
}
