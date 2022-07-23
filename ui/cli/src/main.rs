use std::env;

fn main(){
    println!("hello");
    let args: Vec<String> = env::args().collect();
    let mode: Mode = match args[1].as_str() {
        "init" => Mode::Init , 
        "list" => Mode::Listup ,
        "search" => {
            if 3 == args.len() {
                Mode::Search(args[2].to_string())
            } else {
                println!("length error");
                Mode::Nothing
            }
        }
        "create" => {
            if 4 == args.len() {
                Mode::Create(args[2].to_string() , args[3].to_string())
            } else {
                println!("length error");
                Mode::Nothing
            }
        }
        "update" => {
            if 5 == args.len() {
                Mode::Update(args[2].to_string() , args[4].to_string() , args[5].to_string())
            } else {
                Mode::Nothing
            }
        },
        _ => Mode::Nothing
    };
    
    may_error_print(run(mode));
}

enum Mode {
    Nothing, 
    Init,
    Listup,
    Search(String),
    Create(String,String),
    Update(String,String,String)
}

fn run(mode: Mode) -> std::io::Result<()> {
    Ok(())
    /*match mode {
        Mode::Nothing => Ok(()) ,
        Mode::Init => ownlinkmemo::init() ,
        Mode::Listup => ownlinkmemo::listup() ,
        Mode::Search(s) => {
            let obj = ownlinkmemo::search(&s)?;
            println!("found: {:?}" , obj);
            Ok(())
        } ,
        Mode::Create(s1 , s2) => ownlinkmemo::create(&ownlinkmemo::Body::new(s1,s2)) ,
        Mode::Update(s1 , s2 , s3) => {
            let body = ownlinkmemo::Body::new(s2,s3);
            ownlinkmemo::update(&s1 , &body)
        }
    }*/
}

fn may_error_print(res: std::io::Result<()>){
    match res {
        Ok(()) => println!("success") ,
        Err(s) => println!("error: {}" , s)
    }
}