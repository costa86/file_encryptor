use clap::Parser;
use fernet::Fernet;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process,
};

const ENCRYPTION: &str = "e";
const DECRYPTION: &str = "d";

/// Simple program to encrypt/decrypt files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Comma-separated files to be encrypted/decrypted
    #[arg(short, long)]
    files: String,

    /// Action to perform (E - encrypt / D - decrypt)
    #[arg(short, long)]
    action: String,

    /// Encryption key file
    #[arg(short, long)]
    key_file: String,
}

fn file_is_already_encrypted(content: &str, key: &str) -> bool {
    let fernet = Fernet::new(key).unwrap();

    let decrypted_text = fernet.decrypt(content);

    if decrypted_text.is_ok() {
        return true;
    }
    false
}

fn encrypt_file(contents: &Vec<u8>, fernet: &Fernet, path: &PathBuf) {
    let encypted_contents = fernet.encrypt(&contents);
    let mut file = fs::File::create(&path).unwrap();
    file.write_all(&encypted_contents.as_bytes()).unwrap();

    println!("{:?} -> Encrypted", &path.as_os_str());
}

fn decrypt_file(contents: &String, fernet: &Fernet, path: &PathBuf) {
    let decrypted_contents = fernet.decrypt(contents);
    let mut message = format!(
        "{:?} -> Either is already decrypted or key is invalid",
        &path.as_os_str()
    );

    if decrypted_contents.is_ok() {
        let mut file = fs::File::create(&path).unwrap();
        file.write_all(&decrypted_contents.unwrap()).unwrap();
        message = format!("{:?} -> Decrypted", &path.as_os_str());
    }
    println!("{}", message);
}

fn check_file_exists(file_path: &PathBuf) {
    if !file_path.is_file() {
        println!("ERROR: {:?} is not a valid file", file_path);
        process::exit(0);
    }
}

fn main() {
    let args = Args::parse();

    let action = args.action.to_ascii_lowercase();

    if action != ENCRYPTION && action != DECRYPTION {
        println!("Invalid action");
        process::exit(0);
    }

    let key_file_path = PathBuf::from(args.key_file);
    check_file_exists(&key_file_path);

    let key = fs::read(Path::new(&key_file_path)).unwrap();
    let key = String::from_utf8(key.clone()).unwrap();

    let intro = if action == ENCRYPTION {
        "Encrypting files..."
    } else {
        "Decrypting files..."
    };

    println!("{intro}");
    let files: Vec<&str> = args.files.split(',').collect();
    let fernet = Fernet::new(&key).unwrap();

    for i in files {
        let path = PathBuf::from(&i);

        check_file_exists(&path);

        let contents = fs::read(Path::new(&path)).unwrap();
        let contents_str = String::from_utf8(contents.clone()).unwrap();

        if action == ENCRYPTION {
            if file_is_already_encrypted(&contents_str, &key) {
                println!("{:?} -> Already encrypted", &i);
                continue;
            }
            encrypt_file(&contents, &fernet, &path);
        } else {
            decrypt_file(&contents_str, &fernet, &path);
        }
    }
}
