mod base64;
use base64::Base64Codec;
use std::env::args;

fn show_help(){
    println!("Base64_v0.1.1 [Option] [Text]");
    println!("Option:");
    println!("    e|-e      to encode");
    println!("    d|-d      to decode");
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
                _ => show_help(),
            }
              
        },
        _ => show_help(),
    };
}
