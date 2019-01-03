use regex::Regex;

pub struct RawDiagram {
    pub _typ: DiagramType,
    pub _name: String,
    pub _input: Vec<String>
}

pub enum DiagramType {
    CLASS,
    USECASE,
    ACTION,
    SEQUENCE,
    STATE,
    COMPONENT,
    PACKAGE,
    DEPLOYMENT,
    OBJECT,
    NOTFOUND
}

fn build_raw_diagram(_typ: DiagramType, _name: String) -> RawDiagram {
    RawDiagram {
        _typ,
        _name,
        _input: Vec::new()
    }
}

pub fn get_diagram(diagrams: &mut Vec<RawDiagram>, input: &str) {

    //Leere Zeilen aus dem String entfernen, Trennungszeichen
    let mut inhalt = String::from(input);
    inhalt = inhalt.replace(" ", "");
    inhalt = inhalt.replace("\"\n", "\"|");
    inhalt = inhalt.replace("\n", "");


    let split : Vec<&str> = inhalt.split("|").collect();

    let mut diagram = build_raw_diagram(DiagramType::NOTFOUND, "".to_string());

	for item in split.iter() {
        //Klassendiagramm?
        let mut re = Regex::new("(^type\"classdiagram\")|(^type\"classdiagram:.+?\")").unwrap();
        if re.is_match(item) {
            if !diagram._input.is_empty() {
                diagrams.push(diagram);
            }
            let mut re = Regex::new("^type\"classdiagram:(.+?)\"").unwrap();
            if re.is_match(item) {
                let caps = re.captures(item).unwrap();
                let u = caps.get(1).map_or(String::from(""), |m| String::from(m.as_str()));
                diagram = build_raw_diagram(DiagramType::CLASS, u);
            } else {
                diagram = build_raw_diagram(DiagramType::CLASS, "Klassendiagramm".to_string());
            }
            continue;
        }
        //Anwendungsfalldiagramm?
        let mut re = Regex::new("(^type\"usecasediagram\")|(^type\"usecasediagram:.+?\")").unwrap();
        if re.is_match(item) {
            if !diagram._input.is_empty() {
                diagrams.push(diagram);
            }
            let mut re = Regex::new("^type\"usecasediagram:(.+?)\"").unwrap();
            if re.is_match(item) {
                let caps = re.captures(item).unwrap();
                let u = caps.get(1).map_or(String::from(""), |m| String::from(m.as_str()));
                diagram = build_raw_diagram(DiagramType::USECASE, u);
            } else {
                diagram = build_raw_diagram(DiagramType::USECASE, "Anwendungsfalldiagramm".to_string());
            }
            continue;
        }
        //Aktivitätsdiagramm?
        let mut re = Regex::new("(^type\"actiondiagram\")|(^type\"actiondiagram:.+?\")").unwrap();
        if re.is_match(item) {
            if !diagram._input.is_empty() {
                diagrams.push(diagram);
            }
            let mut re = Regex::new("^type\"actiondiagram:(.+?)\"").unwrap();
            if re.is_match(item) {
                let caps = re.captures(item).unwrap();
                let u = caps.get(1).map_or(String::from(""), |m| String::from(m.as_str()));
                diagram = build_raw_diagram(DiagramType::ACTION, u);
            } else {
                diagram = build_raw_diagram(DiagramType::ACTION, "Aktivitätsdiagramm".to_string());
            }
            continue;
        }
        //Sequenzdiagramm?
        re = Regex::new("(^type\"sequencediagram\")|(^type\"sequencediagram:.+?\")").unwrap();
        if re.is_match(item) {
            if !diagram._input.is_empty() {
                diagrams.push(diagram);
            }
            let mut re = Regex::new("^type\"sequencediagram:(.+?)\"").unwrap();
            if re.is_match(item) {
                let caps = re.captures(item).unwrap();
                let u = caps.get(1).map_or(String::from(""), |m| String::from(m.as_str()));
                diagram = build_raw_diagram(DiagramType::SEQUENCE, u);
            } else {
                diagram = build_raw_diagram(DiagramType::SEQUENCE, "Sequenzdiagramm".to_string());
            }
            continue;
        }
        //Zustandsdiagramm?
        re = Regex::new("(^type\"statediagram\")|(^type\"statediagram:.+?\")").unwrap();
        if re.is_match(item) {
            if !diagram._input.is_empty() {
                diagrams.push(diagram);
            }
            let mut re = Regex::new("^type\"statediagram:(.+?)\"").unwrap();
            if re.is_match(item) {
                let caps = re.captures(item).unwrap();
                let u = caps.get(1).map_or(String::from(""), |m| String::from(m.as_str()));
                diagram = build_raw_diagram(DiagramType::STATE, u);
            } else {
                diagram = build_raw_diagram(DiagramType::STATE, "Zustandsdiagramm".to_string());
            }
            continue;
        }
        //Komponentendiagramm?
        re = Regex::new("(^type\"componentdiagram\")|(^type\"componentdiagram:.+?\")").unwrap();
        if re.is_match(item) {
            if !diagram._input.is_empty() {
                diagrams.push(diagram);
            }
            let mut re = Regex::new("^type\"componentdiagram:(.+?)\"").unwrap();
            if re.is_match(item) {
                let caps = re.captures(item).unwrap();
                let u = caps.get(1).map_or(String::from(""), |m| String::from(m.as_str()));
                diagram = build_raw_diagram(DiagramType::COMPONENT, u);
            } else {
                diagram = build_raw_diagram(DiagramType::COMPONENT, "Komponentendiagramm".to_string());
            }
            continue;
        }
        //Paketdiagramm?
        re = Regex::new("(^type\"packagediagram\")|(^type\"packagediagram:.+?\")").unwrap();
        if re.is_match(item) {
            if !diagram._input.is_empty() {
                diagrams.push(diagram);
            }
            let mut re = Regex::new("^type\"packagediagram:(.+?)\"").unwrap();
            if re.is_match(item) {
                let caps = re.captures(item).unwrap();
                let u = caps.get(1).map_or(String::from(""), |m| String::from(m.as_str()));
                diagram = build_raw_diagram(DiagramType::PACKAGE, u);
            } else {
                diagram = build_raw_diagram(DiagramType::PACKAGE, "Paketdiagramm".to_string());
            }
            continue;
        }
        //Verteilungsdiagramm?
        re = Regex::new("(^type\"deploymentdiagram\")|(^type\"deploymentdiagram:.+?\")").unwrap();
        if re.is_match(item) {
            if !diagram._input.is_empty() {
                diagrams.push(diagram);
            }
            let mut re = Regex::new("^type\"deploymentdiagram:(.+?)\"").unwrap();
            if re.is_match(item) {
                let caps = re.captures(item).unwrap();
                let u = caps.get(1).map_or(String::from(""), |m| String::from(m.as_str()));
                diagram = build_raw_diagram(DiagramType::DEPLOYMENT, u);
            } else {
                diagram = build_raw_diagram(DiagramType::DEPLOYMENT, "Verteilungsdiagramm".to_string());
            }
            continue;
        }
        //Objektdiagramm?
        re = Regex::new("(^type\"objectdiagram\")|(^type\"objectdiagram:.+?\")").unwrap();
        if re.is_match(item) {
            if !diagram._input.is_empty() {
                diagrams.push(diagram);
            }
            let mut re = Regex::new("^type\"objectdiagram:(.+?)\"").unwrap();
            if re.is_match(item) {
                let caps = re.captures(item).unwrap();
                let u = caps.get(1).map_or(String::from(""), |m| String::from(m.as_str()));
                diagram = build_raw_diagram(DiagramType::OBJECT, u);
            } else {
                diagram = build_raw_diagram(DiagramType::OBJECT, "Objektdiagramm".to_string());
            }
            continue;
        }

        diagram._input.push(item.to_string());
        
    }

    if !diagram._input.is_empty() {
        diagrams.push(diagram);
    }
    

}