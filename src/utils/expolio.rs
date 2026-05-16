/*

// let mut username = String::new();
        // println!("--- Seja bem vindo ---");
        // println!("1 - Ver usuarios");
        // println!("2 - Cadastrar usuario");
        // io::stdout().flush()?;
        // // println!("");
        // let mut option = String::new().to_lowercase();
        // option.clear();
        // io::stdin().read_line(&mut option).expect("Campo não preenchido");
        // match option.trim() {
        //     "1" => {
        //         println!("--- Ver usuários ---");
        //         db::select_users(&conn).await?;
        //     },
        //     "2" => {
        //         println!("Digite o nome do usuário que deseja cadastrar:");
        //         let mut username = String::new();
        //         io::stdin().read_line(&mut username).expect("Campo obrigatório.");
        //         let name = username.trim();
        //         if name.is_empty() {
        //             println!("Não preencheu corretamente. Inicia o app novamente");
        //             continue;
        //         }

        //         db::cadastro(&conn, username.trim().to_string()).await?;
                
        //     },
        //     _ => {
        //         println!("Preenche as coisa certas.");
        //     }
        // }


                    println!("-----------------------------------");
loop {
        println!("----- Seja bem vindo! -----");
        println!("Digite uma das opções abaixo para seguir:");
        println!("1 - Cadastro de usuário.");
        println!("2 - Acessar com um usuário.");
        println!("3 - Acessar TO-DO list.");
        println!("4 - Converter um arquivo excel em json.");
        println!("5 - Verificar estado de redes.");
        io::stdout().flush()?;
        
        println!("--------------------------------------");
        select_option.clear();
        io::stdin().read_line(&mut select_option).expect("Dado não preenchido corretamente");
        println!("--------------------------------------");

        match select_option.trim() {
            "1" => {
                let mut username = String::new();
                println!("Informe seu nome de usuário:");
                username.clear();
                io::stdin().read_line(&mut username).expect("Preencha o campo corretamente...");

                let cadastro = db::cadastro(&conn, username.trim().to_string()).await?;

                if cadastro == true {
                    println!("Usuário cadastrado com sucesso");
                    continue;
                } else {
                    println!("Usuário já cadastrado, tente novamente.");
                    println!("---------------------------------------");
                }

            },
            _ => {
                println!("Selecione uma opção valida...");
            }
        }
    }


*/