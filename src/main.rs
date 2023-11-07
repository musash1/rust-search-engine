use std::{fs, path::Path};

fn get_dir(dir_path: &Path) -> Result<(), ()> {
    let dir = fs::read_dir(dir_path).map_err(|err| {
        eprintln!("ERROR: could not open directory {dir_path}: {err}", dir_path = dir_path.display());
    })?; 

    'next_file: for file in dir {
        let file = file.map_err(|err| {
            eprintln!("ERROR: could not read next file in dir {dir_path}: {err}", dir_path = dir_path.display());
        })?; 

        let file_path = file.path();

        let file_type = file.file_type().map_err(|err| {
            eprintln!("ERROR: could not determine type of file {file_path}: {err}", file_path = file_path.display());
        })?;

        if file_type.is_dir() {
            get_dir(&file_path);
            continue 'next_file;
        }

        println!("Name: {}", file_path.display());
    }

    Ok(())
}

fn main() {
    get_dir(Path::new("./docs.gl")); 
}
