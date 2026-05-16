mod db;
mod services;

use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let conn = db::connection().await?;
    let mut select_option = String::new();
    loop {
        println!("1 - Cadastro de usuário.");
        println!("2 - Acessar com um usuário.");
        println!("3 - Sair.");
        select_option.clear();
        io::stdin().read_line(&mut select_option).expect("Dado informado incorretamente.");
        println!("-------------------------------");
        match select_option.trim() {
            "1" => {
                let mut username = String::new();
                println!("--- Bem vindo(a) ao cadastro ---");
                println!("Informe o seu nome ou o nome do usuário que deseja cadastrar:");
                username.clear();
                io::stdin().read_line(&mut username).expect("Dado não informado corretamente.");
                println!("-----------------------------------------------------------------");

                let cadastro = db::cadastro(&conn, username.trim().to_string()).await?;

                if cadastro {
                    println!("Usuário cadastrado com sucesso!");
                    println!("-----------------------------------");
                    continue;
                }else {
                    println!("Já existe um usuário cadastrado com esse nome, tente novamente...");
                    println!("-----------------------------------");
                }
            },
            "2" => {
                let mut username = String::new();
                println!("----------------- Bem vindo(a) ao login! -----------------");
                println!("--- A partir daqui você irá conseguir acessar o sistema ---");
                println!("------------------------------------------------------------");

                println!("Informe o nome do seu usuário:");
                username.clear();
                io::stdin().read_line(&mut username).expect("Dado não informado corretamente.");

                println!("-------------------------------------------------------------");

                let access = db::access(&conn, username.trim().to_string()).await?;

                if access {
                    println!("Sucesso!!");
                } else if !access {
                    println!("Essa desgraça não funcionou 😤😤😤");
                }
            },
            "3" => {
                println!("Saindo...");
                break Ok(());
            },   
            _ => {}
        }
    }


}