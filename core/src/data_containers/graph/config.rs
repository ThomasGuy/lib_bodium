pub struct Config {
    pub file_path: String,
    pub source_vertex: i32,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let file_path = args.next().ok_or("Missing file path")?;
        let source_vertex = args
            .next()
            .ok_or("Missing source_vertex argument")?
            .parse()
            .map_err(|_| "The source_vertex arg must be an integer")?;

        Ok(Self {
            file_path,
            source_vertex,
        })
    }
}
