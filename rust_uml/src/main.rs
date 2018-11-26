extern crate regex;
extern crate image;
extern crate imageproc;
extern crate rusttype;


mod check_file;
mod get_diagram_type;
mod parsing;
mod build_class_diagram;

fn main() {
    //Deklaration der Parser-Stukturen
    //Für Klassendiagramme
    let mut klassen: Vec<parsing::parse_class::Klasse> = Vec::new();
    let mut beziehungen_class: Vec<parsing::parse_class::Beziehung> = Vec::new();
    //Für Usecasediagramme
    let mut typen: Vec<parsing::parse_usecase::Typ> = Vec::new();
    let mut beziehungen_usecase: Vec<parsing::parse_usecase::Beziehung> = Vec::new();



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
            parsing::parse_class::parse(&mut vektor, &mut klassen, &mut beziehungen_class);
            build_class_diagram::build_klassendiagramm(&mut klassen, &mut beziehungen_class);
        }

	    get_diagram_type::DiagramType::USECASE => {
            parsing::parse_usecase::parse(&mut vektor, &mut typen, &mut beziehungen_usecase);
            
            println!("Elementtypen:");
            for s in typen {
                println!("{}", s._elementname);
            }
            println!("Beziehungen:");
            for s in beziehungen_usecase {
                for t in s._von_element_name {
                    println!("{}", t);
                }
                println!("{}", s._von_element_mult);
                println!("{}", s._zu_element_name);
                println!("{}", s._zu_element_mult);
                println!("");
            }

        }
        
        get_diagram_type::DiagramType::ACTION => { println!("ACTION!"); }
        get_diagram_type::DiagramType::SEQUENCE => { println!("SEQUENCE!"); }
        get_diagram_type::DiagramType::STATE => { println!("STATE!"); }
        get_diagram_type::DiagramType::COMPONENT => { println!("COMPONENT!"); }
        get_diagram_type::DiagramType::PACKAGE => { println!("PACKAGE!"); }
        get_diagram_type::DiagramType::DISTRIBUTION => { println!("Deployment!"); }
        get_diagram_type::DiagramType::OBJECT => { println!("OBJECT!"); }
        get_diagram_type::DiagramType::NOTFOUND => { println!("NOTFOUND!"); 
            parsing::parse_class::parse(&mut vektor, &mut klassen, &mut beziehungen);

            build_class_diagram::build_klassendiagramm(&mut klassen, &mut beziehungen);
        }
    }


}



