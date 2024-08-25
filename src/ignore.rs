use std::fs;

pub fn start_ignore(){
    let home_dir = env!("HOME");
    fs::create_dir(format!("{home_dir}/IgnoredFiles"))
        .expect(format!("\nERRO: Não foi possível criar o diretório \"{home_dir}/IgnoredFiles\"\n\n")
        .as_str());

    // TODO: Verificar se o caminho abaixo existe (verify.rs)
    for ignored_file in fs::read_to_string("config/ignore_list.conf").unwrap().lines() {
        if ignored_file.is_empty() || ignored_file.starts_with("#") {
            continue;
        }

        fs::rename(ignored_file, format!("{home_dir}/IgnoredFiles/{}", 
            ignored_file.replace("/", "_")))
                .expect(format!("\nERRO: Não foi possível mover \"{ignored_file}\" para \"{home_dir}/IgnoredFiles\"n\n")
                .as_str());
    }
    
}

pub fn end_ignore(){
    let home_dir = env!("HOME");

    for ignored_file in fs::read_to_string("config/ignore_list.conf").unwrap().lines() {
        if ignored_file.is_empty() || ignored_file.starts_with("#") {
            continue;
        }

        fs::rename(format!("{home_dir}/IgnoredFiles/{}",
            ignored_file.replace("/", "_")), ignored_file)
                .expect(format!("\nERRO: Não foi possível mover arquivo de volta para \"{ignored_file}\"n\n")
                .as_str());
    }
    fs::remove_dir(format!("{home_dir}/IgnoredFiles"))
        .expect(format!("\nERRO: Não foi possível remover diretório {home_dir}/IgnoredFiles\n\n")
        .as_str());
}