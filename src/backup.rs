use std::env;
use std::path::PathBuf;
use std::{fs, io::Seek};
use sevenz_rust::{self, SevenZWriter, SevenZArchiveEntry, lzma};

pub fn init(dir_name: &str, pass: &String){
    //TODO: VERIFICAR SE TODOS OS UNWRAPS SAO SAFE
    let current_dir = format!("{}/{dir_name}", env!("HOME"));
    
    println!("Adicionando arquivos de {dir_name}");

    let mut sz_writer = SevenZWriter::create(format!("Backup {dir_name}.7z")).unwrap();
    sz_writer.set_content_methods(vec![
        sevenz_rust::AesEncoderOptions::new(pass.as_str().into()).into(),
        lzma::LZMA2Options::with_preset(9).into()
    ]);

    let list_itens = fs::read_dir(format!("{current_dir}")).unwrap();
    for iten in list_itens {
        let iten = iten.unwrap().path();

        add_recursive_files(&mut sz_writer, iten, &dir_name);
    }
    sz_writer.finish().unwrap();
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
                    Some(i) => String::from(i),
                    None => panic!("Something is wrong with the file .config/user-dirs.dirs")
                }
            );
        }
    }

    return vec_dir;
}

fn add_recursive_files <W: std::io::Write>(sz: &mut SevenZWriter<W>, iten: PathBuf, dir_name: &str) where W: Seek {
    let current_dir = format!("{}/{dir_name}", env!("HOME"));

    // Verify if iten is the .7z backup file
    if iten.to_str().unwrap().contains(".7z") {
        let vec_bkp_dir = get_directories();
        for bkp_dir in vec_bkp_dir {
            if String::from(iten.to_str().unwrap()).contains(format!("Backup {bkp_dir}.7z").as_str()) {
                return;
            }
        }
    }

    // Ignore if symbolic link
    if iten.as_path().is_symlink() {
        return;
    }

    // Add file to .7z
    if iten.as_path().is_dir() {
        let subdir_itens = fs::read_dir(iten.as_path()).unwrap();
        for iten in subdir_itens {
            let iten = iten.unwrap().path();

            add_recursive_files(sz, iten, &dir_name);
        }
    }
    else {
        let iten_name = String::from(iten.to_str().unwrap()
            .replace(format!("{current_dir}/").as_str(), ""));

        println!("Adicionando \x1b[32m{iten_name}\x1b[0m");
        sz.push_archive_entry(
            SevenZArchiveEntry::from_path(iten.as_path(), format!("Backup {dir_name}/{iten_name}")),
            Some(fs::File::open(iten.as_path()).unwrap())
        ).unwrap();
    }
}