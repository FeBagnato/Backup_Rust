use rpassword;
use verify::{start_verify, VerifyError};
use semaphore::Semaphore;
use std::{thread::{self, JoinHandle}, sync::Arc};

mod backup;
mod ignore;
mod verify;
mod semaphore;

fn main() {
    println!("\x1b[33mCaso tenha algum arquivo ou pasta que você não queira adicionar ao backup, 
coloque o caminho em \"config/ignore_list.conf\"\x1b[0m\n");

    // Verify errors before start
    match start_verify() {
        Ok(_) => {},
        Err(VerifyError::ILFileNotFound(file)) => {
            println!("\x1b[31m\nErro:\x1b[0m ignore_list.conf. Não foi possível encontrar o arquivo \"{file}\"");
            std::process::exit(1);
        },
        Err(VerifyError::ILFileNotAcesseble(file)) => {
            println!("\x1b[31m\nErro:\x1b[0m ignore_list.conf. Não foi possível verificar a existência do arquivo \"{file}\"");
            std::process::exit(1);
        },

        Err(VerifyError::BKPFileExists(file)) => {
            println!("\x1b[31m\nErro:\x1b[0m O arquivo \"{file}\" já existe\nRemova este arquivo antes de continuar");
            std::process::exit(1);
        },

        Err(VerifyError::UserDirConfigNotFound) => {
            let home = std::env::var("HOME").expect("Não foi possível encontrar a variável HOME");
            println!("\x1b[31m\nErro:\x1b[0m Não foi possível encontrar o arquivo \"{}/.config/user-dirs.dirs\"", home);
            std::process::exit(1);
        }
    }

    // Get password
    let password = loop {
        let pass1 = rpassword::prompt_password("Digite a senha: ")
            .expect("\nERRO: Não foi possível pegar a senha\n\n");
        let pass2 = rpassword::prompt_password("Digite a senha novamente: ")
            .expect("\nERRO: Não foi possível pegar a senha\n\n");

        if pass1 == pass2 {
            break pass1;
        }
        println!("\x1b[31m\nSenha incorreta!\n\x1b[0mTente novamente\n");
    };

    let vec_dir = backup::get_directories();

    ignore::start_ignore();

    let max_cpu = num_cpus::get() / 2;
    let semaphore = Arc::new(Semaphore::new(max_cpu));

    // Start backup process
    let arc_password = Arc::new(password);
    let mut count_handle: Vec<JoinHandle<_>> = Vec::new();
    for dir in vec_dir.iter() {
        let dir = dir.clone();
        let arc_password = Arc::clone(&arc_password);
        let arc_semaphore = Arc::clone(&semaphore);

        let handle = thread::spawn(move || {
            let _semaphore_guard = arc_semaphore.acquire();
            backup::init(&dir, arc_password.as_ref());
            // No need to call arc_semaphore.release() since this struct implements Drop
        });

        count_handle.push(handle);
    }

    for handle in count_handle {
        handle.join()
            .expect("\nERRO: Falha durante a execução da thread\n\n");
    }

    ignore::end_ignore();
}
