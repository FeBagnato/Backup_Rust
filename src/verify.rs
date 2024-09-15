use std::{fs, path::Path, result::Result};

use crate::backup::get_directories;

pub enum VerifyError {
    ILFileNotFound(String),
    ILFileNotAcesseble(String),
    BKPFileExists(String)
}

pub fn start_verify<'a>() -> Result<(), VerifyError> {
    match BKP_files_verify() {
        Ok(_) => {},
        Err(err) => { return Err(err) }
    }

    match IL_verify() {
        Ok(_) => {},
        Err(err) => { return Err(err) }
    }

    Ok(())
}

#[allow(non_snake_case)]
fn IL_verify<'a>() -> Result<(), VerifyError> {
    for file_line in fs::read_to_string(Path::new("config/ignore_list.conf"))
        .expect("Não foi possível abrir arquivo config/ignore_list.conf")
        .lines()
    {
        if file_line.starts_with("#") || file_line.is_empty() {
            continue;
        }

        match fs::exists(Path::new(file_line)) {
            Ok(true) => {},
            Ok(false) => {
                return Err(VerifyError::ILFileNotFound(String::from(file_line)));
            },
            Err(_) => {
                return Err(VerifyError::ILFileNotAcesseble(String::from(file_line)));
            }
        }
    }

    Ok(())
}

#[allow(non_snake_case)]
fn BKP_files_verify() -> Result<(), VerifyError> {
    let dir_vec = get_directories();
    let mut backup_file_path: &Path;

    for dir in dir_vec {
        let backup_file_name = format!("Backup {dir}.7z");
        backup_file_path = Path::new(&backup_file_name);

        if backup_file_path.exists() {
            return Err(VerifyError::BKPFileExists(backup_file_name));
        }
    }

    Ok(())
}