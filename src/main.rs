use rpassword;

mod backup;

fn main() {
    println!("\x1b[33mCaso tenha algum arquivo ou pasta que você não queira adicionar ao backup, 
coloque o caminho em \"config/ignore_list.conf\"\x1b[0m\n");

    let mut pass_error = true;
    while pass_error {
        let password = rpassword::prompt_password("Digite a senha: ").unwrap();

        if password == rpassword::prompt_password("Digite a senha novamente: ").unwrap() {
            //TODO: Run verify code here!
            //TODO: Run ignore_list code here!
            backup::init("Desktop", &password);
            backup::init("Documentos", &password);
            backup::init("Downloads", &password);
            backup::init("Imagens", &password);
            backup::init("Música", &password);
            backup::init("Vídeos", &password);
            pass_error = false;
        }
        else{
            println!("\x1b[31mSenha incorreta!\nTente novamente\n\x1b[0m");
        }
    }
}
