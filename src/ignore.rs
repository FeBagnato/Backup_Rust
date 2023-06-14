use std::fs;

pub fn start_ignore(){
    let home_dir = env!("HOME");
    fs::create_dir(format!("{home_dir}/IgnoredFiles")).unwrap();

    for ignored_file in fs::read_to_string("config/ignore_list.conf").unwrap().lines() {
        fs::rename(ignored_file, format!("{home_dir}/IgnoredFiles/{}", 
            ignored_file.replace("/", "_"))).unwrap();
    }
    
}

pub fn end_ignore(){
    let home_dir = env!("HOME");

    for ignored_file in fs::read_to_string("config/ignore_list.conf").unwrap().lines() {
        fs::rename(format!("{home_dir}/IgnoredFiles/{}",
            ignored_file.replace("/", "_")), ignored_file).unwrap();
    }
    fs::remove_dir(format!("{home_dir}/IgnoredFiles")).unwrap();
}