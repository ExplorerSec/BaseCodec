mod base64;
use base64::Base64Codec;
use std::fs::{read,read_to_string,write};
use std::env::args;

fn show_help(){
    println!("Base64_v0.1.4 [Option] [Text]");
    println!("Option:");
    println!("    e|-e      to encode");
    println!("    d|-d      to decode");
    println!("    fe|-fe    read from file and encode");
    println!("    fd|-fd    read from file and decode");
    println!("    fef|-fef  read from file, encode, and write to file");
    println!("    fdf|-fdf  read from file, decode, and write to file");
}

fn main() {
    let args: Vec<String> = args().collect();
    match args.len() {
        3 =>{
            let ct = Base64Codec::default();
            match &args[1][..] {
                "e"|"-e" =>{
                    println!("{}",ct.encode_str(&args[2])); 
                },
                "d"|"-d" =>{
                    println!("{}",ct.decode_str(&args[2]));
                },
                "fe"|"-fe" =>{
                    if let Ok(vec) = read(&args[2]){
                        println!("{}",ct.encode(vec));
                    }
                    else {
                        println!("[Error] Failed to read file");
                    }
                },
                "fd"|"-fd" =>{
                    if let Ok(s) = read_to_string(&args[2]){
                        println!("{}",ct.decode_str(&s));
                    }
                    else {
                        println!("[Error] Failed to read file");
                    }
                },
                "fef"|"-fef" =>{
                    if let Ok(vec) = read(&args[2]){
                        let mut path = args[2].clone();
                        path.push_str(".enc");
                        let _ = write(path,ct.encode(vec));
                    }
                    else {
                        println!("[Error] Failed to read file");
                    }
                },
                "fdf"|"-fdf" =>{
                    if let Ok(s) = read_to_string(&args[2]){
                        let vec = ct.decode(&s);
                        let mut path = args[2].clone();
                        path.push_str(".dec");
                        let _ = write(path,vec);
                    }
                    else {
                        println!("[Error] Failed to read file");
                    }
                }
                _ => show_help(),
            }
              
        },
        _ => show_help(),
    };
}
