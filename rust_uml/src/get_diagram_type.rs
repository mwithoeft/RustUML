use std::fs::File;
use std::io::BufReader;
use std::io::Read;


pub fn read_file(filepath: String) -> String {
    let mut inhalt = String::new();

    let file = File::open(&filepath).expect("Datei kann nicht geöffnet werden!");
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut inhalt).expect("Datei kann nicht gelesen werden!");

    get_type(&inhalt);

    return inhalt;
}

fn get_type(content: &str) {

    println!("{}", content);

}