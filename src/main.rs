mod base64;
use base64::Base64Codec;
use std::fs::read_to_string;
use std::env::args;

fn show_help(){
    println!("Base64_v0.1.2 [Option] [Text]");
    println!("Option:");
    println!("    e|-e      to encode");
    println!("    d|-d      to decode");
    println!("    fe|-fe    read from file and encode");
    println!("    fd|-fd    read from file and decode");
}

fn main() {
    let args: Vec<String> = args().collect();
    match args.len() {
        3 =>{
            let ct = Base64Codec::default();
            match &args[1][..] {
                "e"|"-e" =>{
                    println!("{}",ct.encode(&args[2])); 
                },
                "d"|"-d" =>{
                    println!("{}",ct.decode(&args[2]));
                },
                "fe"|"-fe" =>{
                    if let Ok(s) = read_to_string(&args[2]){
                        println!("{}",ct.encode(&s));
                    }
                    else {
                        println!("[Error] Failed to read file");
                    }
                },
                "fd"|"-fd" =>{
                    if let Ok(s) = read_to_string(&args[2]){
                        println!("{}",ct.decode(&s));
                    }
                    else {
                        println!("[Error] Failed to read file");
                    }
                },
                _ => show_help(),
            }
              
        },
        _ => show_help(),
    };
}
