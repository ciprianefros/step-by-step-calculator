use std::fs::File;
use std::fs::{read_dir, remove_file};
use std::io::Write;

pub fn save_to_file(file_name: &str, steps: &[String]) -> Result<(), std::io::Error> {
    let mut path = String::from("evaluations/");
    path.push_str(file_name);
    let mut file = File::create(path)?;
    for step in steps {
        writeln!(file, "{}", step)?;
    }
    Ok(())
}

pub fn delete_saved_evaluations() -> Result<(), std::io::Error> {
    let evaluations_directory = "evaluations/";

    if let Ok(entries) = read_dir(evaluations_directory) {
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                remove_file(path)?;
            }
        }
    }
    Ok(())
}
