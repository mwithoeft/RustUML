extern crate regex;

mod check_file;
mod get_diagram_type;
mod parsing;

fn main() {
    //Deklaration der Parser-Stukturen
    //Für Klassendiagramme
    let mut klassen: Vec<parsing::parse_class::Klasse> = Vec::new();
    let mut beziehungen: Vec<parsing::parse_class::Beziehung> = Vec::new();


    //Eingabe des Dateinamens:
    let filepath = check_file::get_file_path();

    //Auslese der Datei, wenn Diagrammtyp angegeben
    let mut vektor : Vec<&'static str>;
    let tupel = get_diagram_type::read_file(filepath);

    //Aufrufen des Diagramm entsprechenden Parsers
    vektor = tupel.0;
    let typ : get_diagram_type::DiagramType = tupel.1;
    match typ {
	    get_diagram_type::DiagramType::CLASS => { 
            parsing::parse_class::parse(&mut vektor, &mut klassen, &mut beziehungen);

            for i in klassen {
                println!("{}", i._name);
                println!("{}", i._property);
                println!("{}", i._keywords);
                for s in i._attribute {
                    println!("{}", s._modifikator);
                    println!("{}", s._static);
                    println!("{}", s._final);
                    println!("{}", s._name);
                    println!("{}", s._datentyp);
                    println!("{}", s._wert);
                }
                for s in i._methoden {
                    println!("{}", s._modifikator);
                    println!("{}", s._static);
                    println!("{}", s._final);
                    println!("{}", s._name);
                    println!("{}", s._returntyp);
                    for t in s._parameter {
                        println!("{}", t);
                    }
                }
                println!("");
            }
            for i in beziehungen {
                println!("{}", i._von_klasse_name);
                println!("{}", i._von_klasse_pfeil);
                println!("{}", i._von_klasse_mult);
                println!("{}", i._zu_klasse_name);
                println!("{}", i._zu_klasse_pfeil);
                println!("{}", i._zu_klasse_mult);
            }
        }

        
	    get_diagram_type::DiagramType::USECASE => { println!("USECASE!"); }
        get_diagram_type::DiagramType::ACTION => { println!("ACTION!"); }
        get_diagram_type::DiagramType::SEQUENCE => { println!("SEQUENCE!"); }
        get_diagram_type::DiagramType::STATE => { println!("STATE!"); }
        get_diagram_type::DiagramType::COMPONENT => { println!("COMPONENT!"); }
        get_diagram_type::DiagramType::PACKAGE => { println!("PACKAGE!"); }
        get_diagram_type::DiagramType::DISTRIBUTION => { println!("DISTRIBUTION!"); }
        get_diagram_type::DiagramType::OBJECT => { println!("OBJECT!"); }
        get_diagram_type::DiagramType::NOTFOUND => { println!("NOTFOUND!"); }
    }


}


