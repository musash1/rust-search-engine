use std::fs::File;
use std::io:: {
    Result,
    prelude::*,
    BufReader
};


fn main() -> Result<()> {
    let file = File::open("./docs.gl/gl4/glGetCompressedTexImage.xhtml")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    println!("{contents}");
    Ok(())
}
