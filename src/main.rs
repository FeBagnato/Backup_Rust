use rpassword;
use std::{thread::{self, JoinHandle}, sync::Arc};

mod backup;
mod ignore;

fn main() {
    println!("\x1b[33mCaso tenha algum arquivo ou pasta que você não queira adicionar ao backup, 
coloque o caminho em \"config/ignore_list.conf\"\x1b[0m\n");

    //TODO: Add the directories to vector dinamicaly
    let vec_dir = vec!["Desktop", "Documentos", "Downloads", "Imagens", "Música", "Vídeos"];
    let mut pass_error = true;
    while pass_error {
        let password = rpassword::prompt_password("Digite a senha: ").unwrap();

        if password == rpassword::prompt_password("Digite a senha novamente: ").unwrap() {
            //TODO: Run verify code here!
            ignore::start_ignore();

            let arc_password = Arc::new(password);
            let mut count_handle: Vec<JoinHandle<_>> = Vec::new();
            for i in 0..vec_dir.len() {
                let current_directory = vec_dir[i];
                let arc_password = Arc::clone(&arc_password);

                let handle = thread::spawn(move || {
                    backup::init(current_directory, arc_password.as_ref());
                });

                count_handle.push(handle);
            }

            for handle in count_handle {
                handle.join().unwrap();
            }

            ignore::end_ignore();
            pass_error = false;
        }
        else{
            println!("\x1b[31mSenha incorreta!\nTente novamente\n\x1b[0m");
        }
    }
}
