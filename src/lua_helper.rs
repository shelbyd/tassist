use std::path::Path;

pub fn write_file_when_done<P: AsRef<Path>>(path: P) -> String {
    format!(r##"
        local file = io.open("{}", "w")
        file:write("done")
    "##, path.as_ref().to_string_lossy())
}