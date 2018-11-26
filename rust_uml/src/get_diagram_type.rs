use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use regex::Regex;
use std::boxed::Box;

pub enum DiagramType {
    CLASS,
    USECASE,
    ACTION,
    SEQUENCE,
    STATE,
    COMPONENT,
    PACKAGE,
    DISTRIBUTION,
    OBJECT,
    NOTFOUND
}

pub fn read_file(filepath: String) -> (Vec<&'static str>, DiagramType) {
    let mut inhalt = String::new();

    let file = File::open(&filepath).expect("Datei kann nicht geöffnet werden!");
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut inhalt).expect("Datei kann nicht gelesen werden!");

    



    //Leere Zeilen aus dem String entfernen, Trennungszeichen
    inhalt = inhalt.replace(" ", "");
    inhalt = inhalt.replace("\"\r\n", "\"|");
    inhalt = inhalt.replace("\r\n", "");


    let split = inhalt.split("|");
    let mut v: Vec<&'static str> = Vec::new();

    for s in split {
        let p: String = s.to_owned();
        let s_slice: &'static str = Box::leak(p.into_boxed_str());

        v.push(s_slice);
    }

    let find = v[0];
    v.remove(0);
	//println!("{}", find);
	
    //Klassendiagramm?
    let mut re = Regex::new("^type\"classdiagram\"").unwrap();
    if re.is_match(find) {
        return (v, DiagramType::CLASS);
    }
    //Anwendungsfalldiagramm?
    re = Regex::new("^type\"usecasediagram\"").unwrap();
    if re.is_match(find) {
        return (v, DiagramType::USECASE);
    }
    //Aktivitätsdiagramm?
    re = Regex::new("^type\"actiondiagram\"").unwrap();
    if re.is_match(find) {
        return (v, DiagramType::ACTION);
    }
    //Sequenzdiagramm?
    re = Regex::new("^type\"sequencediagram\"").unwrap();
    if re.is_match(find) {
        return (v, DiagramType::SEQUENCE);
    }
    //Zustandsdiagramm?
    re = Regex::new("^type\"statediagram\"").unwrap();
    if re.is_match(find) {
        return (v, DiagramType::STATE);
    }
    //Komponentendiagramm?
    re = Regex::new("^type\"componentdiagram\"").unwrap();
    if re.is_match(find) {
        return (v, DiagramType::COMPONENT);
    }
    //Paketdiagramm?
    re = Regex::new("^type\"packagediagram\"").unwrap();
    if re.is_match(find) {
        return (v, DiagramType::PACKAGE);
    }
    //Verteilungsdiagramm?
    re = Regex::new("^type\"distributiondiagram\"").unwrap();
    if re.is_match(find) {
        return (v, DiagramType::DISTRIBUTION);
    }
    //Objektdiagramm?
    re = Regex::new("^type\"objectdiagram\"").unwrap();
    if re.is_match(find) {
        return (v, DiagramType::OBJECT);
    }

    return (v, DiagramType::NOTFOUND);
    

}
