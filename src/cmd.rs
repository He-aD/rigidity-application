pub fn interpret_args(mut args: Vec<String>) -> () {
    let interpreter: &dyn Fn(Vec<String>) -> Result<(), String>;

    match args[0].to_lowercase().as_str() {
        "db"  => {
            interpreter = &db::interpret;
        },
        _ => {
            println!("Unknown module: {}", args[0]);
            return;
        },
    }

    args.drain(0..1);
    if let Some(e) = interpreter(args).err() {
        println!("{}", e);
    }
}

mod db {
    use crate::app_conf;

    pub fn interpret(args: Vec<String>) -> Result<(), String> {
        let mut functors: Vec<&dyn Fn() -> ()> = vec![];
        let mut cmd = "--";

        // store functions to execute and stop if unknown command/args
        for arg in &args {
            if arg.contains("--") {
                cmd = &arg[..];
            } else {
                match cmd {
                    "--insert" => {
                        functors.push(get_insert_fn(&arg)?);
                    }
                    _ => {
                        return Err(format!("Unknown command {} of db module", arg));
                    }
                }
            }
        }

        // execute stored function
        for func in functors {
            func();
        }

        Ok(())
    }

    fn get_insert_fn(arg: &str) -> Result<&dyn Fn() -> (), String> {
        match arg {
            "users" => {
                Ok(&insert_test_users)
            },
            _ => {
                Err(format!("Unknown arg {} of command db --insert", arg))
            }
        }
    }

    fn insert_test_users() {
        use crate::services::auth;
        use crate::models::user;
        use crate::models::forms::user::UserForm;
        use crate::chrono::NaiveDateTime;

        let nb_users = 10;
        let pass_hash = auth::hash_password("spike").unwrap();
        let conn = &app_conf::connect_database().get().unwrap();
        let email_confirmation_hash = "toto";

        for i in 0..nb_users {
            user::create(UserForm {
                email: format!("{}@spikegames.eu", i).as_str(),
                nickname: format!("Spike{}", i).as_str(),
                steam_id: format!("{}", i).as_str(),
                first_name: "Spike",
                last_name: format!("{}", i).as_str(),
                hash: &pass_hash,
                birth_date: NaiveDateTime::default(),
            }, email_confirmation_hash, conn).unwrap();
            
            user::confirm_email(email_confirmation_hash, conn).unwrap();
        }        
    }
}
