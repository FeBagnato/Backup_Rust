use std::env;
use std::{fs, path};
use copy_dir::copy_dir;
use sevenz_rust;

pub fn init(dir_name: &str, pass: &String){
    //TODO: VERIFICAR SE TODOS OS UNWRAPS SAO SAFE
    let home_dir = env!("HOME");
    let current_dir = format!("{home_dir}/{dir_name}");
    
    fs::create_dir(format!("{current_dir}/Backup {dir_name}")).unwrap();
    fs::create_dir(format!("{current_dir}/Backup {dir_name}/Backup {dir_name}")).unwrap();
    println!("Copiando os itens de {dir_name}");

    let list_itens = fs::read_dir(format!("{current_dir}")).unwrap();
    for iten in list_itens {
        let iten = iten.unwrap().path();

        if iten.as_path() == path::Path::new(format!("{current_dir}/Backup {dir_name}").as_str())
            { continue; }

        if iten.as_path().is_dir() {
            if let Some(iten_name) = iten.file_name() {
                let iten_name = iten_name.to_os_string().into_string().unwrap();
                
                copy_dir(&iten, format!("{current_dir}/Backup {dir_name}/Backup {dir_name}/{iten_name}"))
                    .unwrap();
                println!("Copiando \x1b[32m{iten_name}\x1b[0m");
            }
        }

        else if let Some(iten_name) = iten.file_name() {
            let iten_name = iten_name.to_os_string().into_string().unwrap();
            fs::copy(iten, format!("{current_dir}/Backup {dir_name}/Backup {dir_name}/{}", 
                iten_name)).unwrap();
            
            println!("Copiando \x1b[32m{iten_name}\x1b[0m");
        }
    }

    sevenz_rust::compress_to_path_encrypted(format!("{current_dir}/Backup {dir_name}"),
        format!("{current_dir}/Backup {dir_name}.7z"), 
        pass.as_str().into())
    .unwrap();

    fs::remove_dir_all(format!("{current_dir}/Backup {dir_name}")).unwrap();
}

pub fn get_directories() -> Vec<String> {
    let file_config_dirs = format!("{}/.config/user-dirs.dirs", env!("HOME").to_string());
    let mut vec_dir: Vec<String> = vec![];

    for line in fs::read_to_string(file_config_dirs).unwrap().lines() {
        // Ignore line if starts with "#"
        if line.starts_with("#") { continue; }

        if line.contains("DESKTOP") || line.contains("DOWNLOAD") || line.contains("DOCUMENTS") ||
        line.contains("MUSIC") || line.contains("PICTURES") || line.contains("VIDEOS") {
            let directory = match line.rsplit("/").next() {
                Some(i) => i,
                None => panic!("Something is wrong with the file .config/user-dirs.dirs")
            };

            // Remove the last quote from the string and add the result to the vector
            vec_dir.push(
                match directory.split("\"").next() {
                    Some(i) => i.to_string(),
                    None => panic!("Something is wrong with the file .config/user-dirs.dirs")
                }
            );
        }
    }

    return vec_dir;
}