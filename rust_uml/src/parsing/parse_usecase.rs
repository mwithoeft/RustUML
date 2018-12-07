use regex::Regex;

pub struct Typ {
    pub _elementtyp: TypEnum,
    pub _elementname: String,
    pub _behaelter: String
}
pub enum TypEnum {
    SUBJECT,
    ACTOR,
    USECASE,
    EXTPOINT,
    UNDEFINED
}
pub struct Beziehung {
    pub _beziehungstyp: Beziehungstyp,
    pub _von_element_name: Vec<String>,
    pub _von_element_mult: String,
    pub _zu_element_name: String,
    pub _zu_element_mult: String,
    pub _notiz: String
}
pub enum Beziehungstyp {
    ASSOCIATION,
    GENERALIZATION,
    INCLUDE,
    EXTEND,
    EXTENDS,
    UNDEFINED
}
fn build_typ(_elementtyp: TypEnum, _elementname: String, _behaelter:String) -> Typ {
    Typ {
        _elementtyp,
        _elementname,
        _behaelter
    }
}
fn build_beziehung(_beziehungstyp: Beziehungstyp, _von_element_mult: String, _zu_element_name: String,_zu_element_mult: String,_notiz: String) -> Beziehung {
    Beziehung {
        _beziehungstyp,
        _von_element_name: Vec::new(),
        _von_element_mult,
        _zu_element_name,
        _zu_element_mult,
        _notiz
    }
}








pub fn parse(string_vec: &mut Vec<&'static str>, mut elementtypen: &mut Vec<Typ>, beziehungen : &mut Vec<Beziehung>) {


    //Parse die Elementtypen
    for s in string_vec.iter() {
        match parse_element(s) {
            Some(x) => {elementtypen.push(x)}
            None    => {}
        }
    }

    //Parse die Beziehungen
    for s in string_vec.iter() {
        match parse_relationship(s, &mut elementtypen) {
            Some(x) => {beziehungen.push(x)}
            None    => {}
        }
    }
}

fn parse_element(s: &str) -> Option<Typ> {
    let re = Regex::new("^elementtype\"([^\"]+?)\"$").unwrap();
    if re.is_match(s) {
        let mut elementtype: Typ = build_typ(TypEnum::UNDEFINED, String::from(""), String::from(""));
        let caps = re.captures(s).unwrap();
        let u = caps.get(1).map_or(String::from(""), |m| String::from(m.as_str()));
        if u.contains(","){
            let v: Vec<&str> = u.split(',').collect();
            for (i, item) in v.iter().enumerate() {
                if i == 0 {
                    match get_enum(&item) {
                        TypEnum::UNDEFINED => {return None;} //Fehler in der Eingabe
                        _ => {elementtype._elementtyp = get_enum(&item);}
                    }
                } else if i == 1 {
                    if item.contains("-") {
                        let s = String::from(item.replace("-"," "));
                        elementtype._elementname = s;
                    } else {
                        elementtype._elementname = item.to_string();
                    }
                } else if i == 2 {
                    if item.contains("-") {
                        let s = String::from(item.replace("-"," "));
                        elementtype._behaelter = s;
                    } else {
                        elementtype._behaelter = item.to_string();
                    }
                } else {
                    return None; //Zu viele Argumente
                }
            }
            match elementtype._elementtyp { //Prüfen, ob extpoint einem usecase zugeordnet ist
                        TypEnum::EXTPOINT=> {
                            if v.len() != 3 {
                                return None;
                            }
                        }
                        _ => {}
            }
            return Some(elementtype);
        } else {
            //Falsche Syntax, kein Komma
            return None;
        }
    }
    //Falsch geschrieben oder es handelt sich einfach nicht um ein Elementtype
    None
}

fn parse_relationship(s: &str, mut typen: &mut Vec<Typ>) -> Option<Beziehung> {
     let re = Regex::new("^relationship\"([^\"]+?)\"$").unwrap();
    if re.is_match(s) {
        let mut beziehung: Beziehung = build_beziehung(Beziehungstyp::UNDEFINED, String::from("-"), String::from(""), String::from("-"), String::from(""));
        let caps = re.captures(s).unwrap();
        let u = caps.get(1).map_or(String::from(""), |m| String::from(m.as_str()));
        if u.contains(","){
            let v: Vec<&str> = u.split(',').collect();
            for (i, item) in v.iter().enumerate() {
                if i == 0 {
                    match get_beziehungstyp(&item) {
                        Beziehungstyp::UNDEFINED => {return None;} //Fehler in der Eingabe
                        _ => {beziehung._beziehungstyp = get_beziehungstyp(&item);}
                    }
                } else {
                    match beziehung._beziehungstyp {
                        Beziehungstyp::GENERALIZATION => {
                            if item.contains("-") {
                                let s = String::from(item.replace("-"," "));
                                if i+1 < v.len() {
                                    if !check_type(&s, &mut typen) {return None;} //Typ kommt nicht in deklaration vor
                                    beziehung._von_element_name.push(s);
                                } else {
                                    beziehung._zu_element_name = s;
                                }
                            } else {
                                if i+1 < v.len() {
                                    if !check_type(&item, &mut typen) {return None;} //Typ kommt nicht in deklaration vor
                                    beziehung._von_element_name.push(item.to_string());
                                } else {
                                    beziehung._zu_element_name = item.to_string();
                                }
                            }
                        }
                        _ => {
                            if i == 1 {
                                if item.contains("-") {
                                    let s = String::from(item.replace("-"," "));
                                    if !check_type(&s, &mut typen) {return None;} //Typ kommt nicht in deklaration vor
                                    beziehung._von_element_name.push(s);
                                } else {
                                    if !check_type(&item, &mut typen) {return None;} //Typ kommt nicht in deklaration vor
                                    beziehung._von_element_name.push(item.to_string());
                                }
                            } else if i == 2 {
                                if item.to_string() != "-" {
                                    beziehung._von_element_mult = item.to_string();
                                }
                            } else if i == 3 {
                                if item.contains("-") {
                                    let s = String::from(item.replace("-"," "));
                                    if !check_type(&s, &mut typen) {return None;} //Typ kommt nicht in deklaration vor
                                    beziehung._zu_element_name = s;
                                } else {
                                    if !check_type(&item, &mut typen) {return None;} //Typ kommt nicht in deklaration vor
                                    beziehung._zu_element_name = item.to_string();
                                }
                            } else if i == 4 {
                                if item.to_string() != "-" {
                                    beziehung._zu_element_mult = item.to_string();
                                }
                            }
                        }
                    }
                }
                
            }
            return Some(beziehung);
        }  else {
            //Falsche Syntax, kein Komma
            return None;
        }

    }
    None
}

fn get_enum(s: &str) -> TypEnum {
    if s == "subject" {
        return TypEnum::SUBJECT;
    } else if s == "actor" {
        return TypEnum::ACTOR;
    } else if s == "usecase" {
        return TypEnum::USECASE;
    } else if s == "extpoint" {
        return TypEnum::EXTPOINT;
    }
    TypEnum::UNDEFINED
}

fn get_beziehungstyp(s: &str) -> Beziehungstyp {
    if s == "association" {
        return Beziehungstyp::ASSOCIATION;
    } else if s == "generalization" {
        return Beziehungstyp::GENERALIZATION;
    } else if s == "include" {
        return Beziehungstyp::INCLUDE;
    } else if s == "extend" {
        return Beziehungstyp::EXTEND;
    } else if s == "extends" {
        return Beziehungstyp::EXTENDS;
    }
    Beziehungstyp::UNDEFINED
}
fn check_type(s: &str, typen: &mut Vec<Typ>) -> bool {
    for typ in typen {
        if s == typ._elementname {
            return true;
        }
    }
    //Typ gibt es in der bisherigen Deklaration nicht
    println!("\nFEHLER, Beziehung mit ungültiger Klasse!\n");
    false
}





