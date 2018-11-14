use std::io::{self, BufRead};
use std::env;
use std::string::String;

pub fn get_file_path() -> String {
    let mut line = String::new();
    let stdin = io::stdin();
    println!("Bitte den Dateinamen ohne Endung eingeben:\nDie Endung muss .txt sein!\n");
    stdin.lock().read_line(&mut line).expect("Konnte nicht gelesen werden!\n");

    let tupel = check_file(&line);
    if tupel.0 {
        println!("Datei gefunden!\n");
        return tupel.1;
    } else {
        println!("Datei existiert nicht!\n");
        return get_file_path();
    }

}

fn check_file(input: &str) -> (bool, String) {
    let mut path = env::current_dir().unwrap();
    let mut s = String::from(input);
    s.remove(s.len()-1);
    s.remove(s.len()-1);
    path.push(s);
    path.set_extension("txt");

    //Gibt den Dateipfad zurück und ob der Pfad gefunden wurde
    (path.exists(), String::from(path.to_string_lossy()))
}