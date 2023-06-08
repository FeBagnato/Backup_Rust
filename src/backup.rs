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