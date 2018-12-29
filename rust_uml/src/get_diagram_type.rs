use regex::Regex;

pub struct RawDiagram {
    pub _typ: DiagramType,
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

fn build_raw_diagram(_typ: DiagramType) -> RawDiagram {
    RawDiagram {
        _typ,
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

    let mut diagram = build_raw_diagram(DiagramType::NOTFOUND);

	for item in split.iter() {
        //Klassendiagramm?
        let mut re = Regex::new("^type\"classdiagram\"").unwrap();
        if re.is_match(item) {
            if !diagram._input.is_empty() {
                diagrams.push(diagram);
            }
            diagram = build_raw_diagram(DiagramType::CLASS);
            continue;
        }
        //Anwendungsfalldiagramm?
        let mut re = Regex::new("^type\"usecasediagram\"").unwrap();
        if re.is_match(item) {
            if !diagram._input.is_empty() {
                diagrams.push(diagram);
            }
            diagram = build_raw_diagram(DiagramType::USECASE);
            continue;
        }
        //Aktivitätsdiagramm?
        let mut re = Regex::new("^type\"actiondiagram\"").unwrap();
        if re.is_match(item) {
            if !diagram._input.is_empty() {
                diagrams.push(diagram);
            }
            diagram = build_raw_diagram(DiagramType::ACTION);
            continue;
        }
        //Sequenzdiagramm?
        re = Regex::new("^type\"sequencediagram\"").unwrap();
        if re.is_match(item) {
            if !diagram._input.is_empty() {
                diagrams.push(diagram);
            }
            diagram = build_raw_diagram(DiagramType::SEQUENCE);
            continue;
        }
        //Zustandsdiagramm?
        re = Regex::new("^type\"statediagram\"").unwrap();
        if re.is_match(item) {
            if !diagram._input.is_empty() {
                diagrams.push(diagram);
            }
            diagram = build_raw_diagram(DiagramType::STATE);
            continue;
        }
        //Komponentendiagramm?
        re = Regex::new("^type\"componentdiagram\"").unwrap();
        if re.is_match(item) {
            if !diagram._input.is_empty() {
                diagrams.push(diagram);
            }
            diagram = build_raw_diagram(DiagramType::COMPONENT);
            continue;
        }
        //Paketdiagramm?
        re = Regex::new("^type\"packagediagram\"").unwrap();
        if re.is_match(item) {
            if !diagram._input.is_empty() {
                diagrams.push(diagram);
            }
            diagram = build_raw_diagram(DiagramType::PACKAGE);
            continue;
        }
        //Verteilungsdiagramm?
        re = Regex::new("^type\"deploymentdiagram\"").unwrap();
        if re.is_match(item) {
            if !diagram._input.is_empty() {
                diagrams.push(diagram);
            }
            diagram = build_raw_diagram(DiagramType::DEPLOYMENT);
            continue;
        }
        //Objektdiagramm?
        re = Regex::new("^type\"objectdiagram\"").unwrap();
        if re.is_match(item) {
            if !diagram._input.is_empty() {
                diagrams.push(diagram);
            }
            diagram = build_raw_diagram(DiagramType::OBJECT);
            continue;
        }

        diagram._input.push(item.to_string());
        
    }

    if !diagram._input.is_empty() {
        diagrams.push(diagram);
    }
    

}