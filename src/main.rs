use rpassword;
use verify::{start_verify, VerifyError};
use std::{thread::{self, JoinHandle}, sync::Arc};

mod backup;
mod ignore;
mod verify;

fn main() {
    println!("\x1b[33mCaso tenha algum arquivo ou pasta que você não queira adicionar ao backup, 
coloque o caminho em \"config/ignore_list.conf\"\x1b[0m\n");

    let mut vec_dir: Vec<String>;
    let mut pass_error = true;
    while pass_error {
        let password = rpassword::prompt_password("Digite a senha: ")
            .expect("\nERRO: Não foi possível pegar a senha\n\n");

        if password == rpassword::prompt_password("Digite a senha novamente: ").expect("\nERRO: Não foi possível pegar a senha\n\n"){
            match start_verify() {
                Ok(_) => {},
                Err(VerifyError::ILFileNotFound(file)) => {
                    println!("Erro ao verificar o ignore_list.conf\nNão foi possível encontrar o arquivo \"{file}\"");
                    break;
                },
                Err(VerifyError::ILFileNotAcesseble(file)) => {
                    println!("Erro ao verificar o ignore_list.conf\nNão foi possível verificar a existência do arquivo \"{file}\"");
                    break;
                },

                Err(VerifyError::BKPFileExists(file)) => {
                    println!("Erro: O arquivo \"{file}\" já existe\nRemova este arquivo antes de continuar");
                    break;
                },

                Err(VerifyError::UserDirConfigNotFound) => {
                    println!("Erro: Não foi possível encontrar o arquivo \"{}/.config/user-dirs.dirs\"", env!("HOME"));
                    break;
                }
            }

            vec_dir = backup::get_directories();

            ignore::start_ignore();

            let arc_password = Arc::new(password);
            let mut count_handle: Vec<JoinHandle<_>> = Vec::new();
            for i in 0..vec_dir.len() {
                let current_directory = String::from(&vec_dir[i]);
                let arc_password = Arc::clone(&arc_password);

                let handle = thread::spawn(move || {
                    backup::init(current_directory.as_str(), arc_password.as_ref());
                });

                count_handle.push(handle);
            }

            for handle in count_handle {
                handle.join()
                    .expect("\nERRO: Falha durante a execução da thread\n\n");
            }

            ignore::end_ignore();
            pass_error = false;
        }
        else{
            println!("\x1b[31mSenha incorreta!\nTente novamente\n\x1b[0m");
        }
    }
}
