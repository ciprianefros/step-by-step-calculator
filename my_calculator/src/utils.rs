use std::fs::File;
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

