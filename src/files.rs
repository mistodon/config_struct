use std::path::Path;

pub fn ensure_destination(path: &Path, create_dirs: bool) -> Result<(), std::io::Error> {
    if create_dirs {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
    }

    Ok(())
}

pub fn write_destination(
    destination: &Path,
    output: String,
    write_only_if_changed: bool,
) -> Result<(), std::io::Error> {
    let should_write = if write_only_if_changed {
        let existing = std::fs::read_to_string(destination);
        match existing {
            Ok(existing) => existing != output,
            Err(_) => true,
        }
    } else {
        true
    };

    if should_write {
        std::fs::write(destination, output)
    } else {
        Ok(())
    }
}

