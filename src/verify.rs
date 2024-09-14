use std::{fs::{self, File}, path::Path, result::Result};

pub enum VerifyError {
    ILFileNotFound(String),
    ILFileNotAcesseble(String)
}

pub fn start_verify<'a>() -> Result<(), VerifyError> {
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
