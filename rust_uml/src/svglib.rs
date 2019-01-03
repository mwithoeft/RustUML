//use rand::distributions::{Distribution, Uniform};
//use rand::Rng;
//use rand::prelude::*;
use std::string::String;
use parsing;
use svg::Document;
use svg::node::element::Path;
use svg::node::element::Rectangle;
use svg::node::element::Line;
use svg::node::element::Text;
use svg::node::element::path::Data;
use svg::node::element::Ellipse;

const ARROW:i32 = 10;
const ACTOR_LINE:i32 = 30;

pub fn rectangle(mut document:Document,x:i32, y:i32, width:i32, height:i32)->Document{
    let rechteck = Rectangle::new()
                .set("x", x)                            // x-Achse
                .set("y", y)                            // y-Achse
                .set("height", height)                  //höhe
                .set("width", width)                    //breite
                .set("fill", "none")                    //füll farbe
                .set("stroke", "black")                 //Rand farbe
                .set("stroke-width", 2);                //Rand breite

                document=document.add(rechteck);

        document
}
pub fn association(mut document:Document,from:(i32,i32), to:(i32,i32), _f_arrow:bool, _t_arrow:bool, horizontal:bool)->Document{
        document=document.add(line(from, to));
        if horizontal{
                if _f_arrow{
                        document = document.add(dependency_arrow((from.0+ARROW,from.1), from, horizontal));
                }
                if _t_arrow{
                        document = document.add(dependency_arrow((to.0-ARROW,to.1), to, horizontal));
                }
        }else {
                if _f_arrow{
                        document = document.add(dependency_arrow((from.0,from.1+ARROW), from, horizontal));
                }
                if _t_arrow{
                        document = document.add(dependency_arrow((to.0,to.1-ARROW), to, horizontal));
                }
        }

        document
}
pub fn extends(mut document:Document,from:(i32,i32), to:(i32,i32), _f_arrow:bool, _t_arrow:bool, horizontal:bool)->Document{
        document = document.add(line(from, to));
        if horizontal {
                if _f_arrow{
                        document = document.add(extends_arrow((from.0+ARROW,from.1), from, horizontal));
                }
                if _t_arrow{
                        document = document.add(extends_arrow((to.0-ARROW,to.1), to, horizontal));
                }
        }else {
                if _f_arrow{
                        document = document.add(extends_arrow((from.0,from.1+ARROW), from, horizontal));
                }
                if _t_arrow{
                        document = document.add(extends_arrow((to.0,to.1-ARROW), to, horizontal));
                }
        }
        document
}
pub fn implements(mut document:Document, from:(i32,i32), to:(i32,i32), _f_arrow:bool, _t_arrow:bool, horizontal:bool)->Document{
        document = document.add(broken_line(from ,to));
        if horizontal {
                if _f_arrow{
                        document = document.add(extends_arrow((from.0+ARROW,from.1), from, horizontal));
                }
                if _t_arrow{
                        document = document.add(extends_arrow((to.0-ARROW,to.1), to, horizontal));
                }
        }else {
                if _f_arrow{
                        document = document.add(extends_arrow((from.0,from.1+ARROW), from, horizontal));
                }
                if _t_arrow{
                        document = document.add(extends_arrow((to.0,to.1-ARROW), to, horizontal));
                }
        }
        document
}
pub fn aggregation(mut document:Document, from:(i32,i32), to:(i32,i32), _f_arrow:bool, _t_arrow:bool, horizontal:bool)->Document{
        document = document.add(line(from, to));
        if horizontal{
                if _f_arrow{
                        document = document.add(aggregation_arrow((from.0+ARROW,from.1), from, horizontal));
                }
                if _t_arrow{
                        document = document.add(aggregation_arrow((to.0-ARROW,to.1), to, horizontal));
                }
        }else {
                if _f_arrow{
                        document = document.add(aggregation_arrow((from.0,from.1+ARROW), from, horizontal));
                }
                if _t_arrow{
                        document = document.add(aggregation_arrow((to.0,to.1-ARROW), to, horizontal));
                }
        }
        document
}
pub fn composition(mut document:Document, from:(i32,i32), to:(i32,i32), _f_arrow:bool, _t_arrow:bool, horizontal:bool)->Document{
        document = document.add(line(from, to));
        if horizontal{
                if _f_arrow{
                        document = document.add(composition_arrow((from.0+ARROW,from.1), from, horizontal));
                }
                if _t_arrow{
                        document = document.add(composition_arrow((to.0-ARROW,to.1), to, horizontal));
                }
        }else {
                if _f_arrow{
                        document = document.add(composition_arrow((from.0,from.1+ARROW), from, horizontal));
                }
                if _t_arrow{
                        document = document.add(composition_arrow((to.0,to.1-ARROW), to, horizontal));
                }
        }
        document
}
pub fn dependency(mut document:Document, from:(i32,i32), to:(i32,i32), _f_arrow:bool, _t_arrow:bool, horizontal:bool)->Document{
        document = document.add(broken_line(from ,to));
        if horizontal {
                if _f_arrow{
                        document = document.add(dependency_arrow((from.0+ARROW,from.1), from, horizontal));
                }
                if _t_arrow{
                        document = document.add(dependency_arrow((to.0-ARROW,to.1), to, horizontal));
                }
        }else {
                if _f_arrow{
                document = document.add(dependency_arrow((from.0,from.1+ARROW), from, horizontal));
                }
                if _t_arrow{
                document = document.add(dependency_arrow((to.0,to.1-ARROW), to, horizontal));
                }
        }
        
        document 
}
fn line(from:(i32,i32), to:(i32,i32))-> Line{
        let line = Line::new()
                .set("x1", from.0)                      //von x-Achse
                .set("y1", from.1)                      //von y-Achse
                .set("x2", to.0)                        //zu x-Achse
                .set("y2", to.1)                        //zu y-Achse
                .set("stroke", "black")                 //Rand farbe
                .set("stroke-width", 2);               //breite
        line
}
fn broken_line(from:(i32,i32), to:(i32,i32))-> Line {
        let line = Line::new()
                .set("x1", from.0)                      //von x-Achse
                .set("y1", from.1)                      //von y-Achse
                .set("x2", to.0)                        //zu x-Achse
                .set("y2", to.1)                        //zu y-Achse
                .set("stroke", "black")                 //Rand farbe
                .set("stroke-width", 2)               //breite
                .set ("stroke-dasharray", 10);
        line
}
fn ellipse(from:(i32,i32), r:(i32,i32))-> Ellipse{
        let ellipse = Ellipse::new()
                .set("cx",from.0)
                .set("cy",from.1)
                .set("rx",r.0)
                .set("ry",r.1)
                .set("style", "fill:white;stroke:black;stroke-width:2");

        ellipse
}
pub fn actor(mut document:Document,from:(i32,i32), name:String, size:i32)-> Document{
        //Körper
        document = document.add(line(from, (from.0, from.1 - ACTOR_LINE)));
        //Linker Fuß
        document = document.add(line(from, (from.0 - ACTOR_LINE, from.1 + ACTOR_LINE)));
        //Rechter Fuß
        document = document.add(line(from, (from.0 + ACTOR_LINE, from.1 + ACTOR_LINE)));
        //Kopf
        document = document.add(ellipse((from.0, from.1 - (ACTOR_LINE as f32 * 1.5) as i32),
                                        (ACTOR_LINE / 2,ACTOR_LINE / 2)));
        //Arme
        document = document.add(line((from.0 - ACTOR_LINE, from.1 - ACTOR_LINE / 2), (from.0 + ACTOR_LINE, from.1 - ACTOR_LINE / 2)));
        //Namen
        document = write(document, (from.0 - (name.chars().count() as i32) * size / 4
                                , (from.1 + size + ACTOR_LINE)), name, size);
        document
}
pub fn usecase(mut document:Document, from:(i32,i32), name:String, size:i32)-> Document{
        document = document.add(ellipse(from, (name.chars().count() as i32 * size , size * 2)));
        document = write(document, (from.0 - (name.chars().count() as i32) * size / 4 ,from.1), name, size);
        
        document
}
pub fn extpoint(mut document:Document, from:(i32,i32), width:i32, height:i32, case_name:String, point_name:String, size:i32)-> Document{
        document = document.add(ellipse(from, (height / 2, width / 2)));
        document = document.add(line( (from.0 - width / 2, from.1),(from.0 + width,from.1)));
        //write usecase
        document = write(document, (from.0 - (case_name.chars().count() as i32) * size / 4 ,from.1 + ARROW), case_name, size);
        //write expoint
        document = write(document, (from.0 - (point_name.chars().count() as i32) * size / 4 ,from.1 - ARROW), point_name, size);

        document
}
pub fn around_the_corner_arrow(mut document:Document, from:(i32,i32), to:(i32,i32), _f_arrow:bool, _t_arrow:bool, rel:&parsing::parse_class::Beziehungstyp, max_height:i32)->Document{
        //let mut rng = rand::thread_rng();
        //let y: i32 = rng.gen_range(10,50);
        //let mut y:i32;
        
        
        
        match *rel{
            parsing::parse_class::Beziehungstyp::EXTENDS => {
                document = extends(document, from, (from.0,from.1+max_height), _f_arrow, false, false);
                document = document.add(line((from.0,from.1+max_height),(to.0,from.1+max_height)));
                document = extends(document, to, (to.0,from.1+max_height), _t_arrow,false , false);
            }
            parsing::parse_class::Beziehungstyp::IMPLEMENTS=>{
                document = implements(document, from, (from.0,from.1+max_height), _f_arrow, false, false);
                document = document.add(broken_line((from.0,from.1+max_height),(to.0,from.1+max_height)));
                document = implements(document, to, (to.0,from.1+max_height), _t_arrow,false , false);
            }
            parsing::parse_class::Beziehungstyp::ASSOCIATION=>{     
                document = association(document, from, (from.0,from.1+max_height), _f_arrow, false, false);
                document = document.add(line((from.0,from.1+max_height),(to.0,from.1+max_height)));
                document = association(document, to, (to.0,from.1+max_height), _t_arrow,false , false);
            }
            parsing::parse_class::Beziehungstyp::AGGREGATION=>{
                document = aggregation(document, from, (from.0,from.1+max_height), _f_arrow, false, false);
                document = document.add(line((from.0,from.1+max_height),(to.0,from.1+max_height)));
                document = aggregation(document, to, (to.0,from.1+max_height), _t_arrow,false , false);
            }
            parsing::parse_class::Beziehungstyp::COMPOSITION=>{
                document = composition(document, from, (from.0,from.1+max_height), _f_arrow, false, false);
                document = document.add(line((from.0,from.1+max_height),(to.0,from.1+max_height)));
                document = composition(document, to, (to.0,from.1+max_height), _t_arrow,false , false);
            }
            parsing::parse_class::Beziehungstyp::DEPENDENCY=>{
                document = dependency(document, from, (from.0,from.1+max_height), _f_arrow, false, false);
                document = document.add(broken_line((from.0,from.1+max_height),(to.0,from.1+max_height)));
                document = dependency(document, to, (to.0,from.1+max_height), _t_arrow,false , false);
            }
            parsing::parse_class::Beziehungstyp::UNDEFINED=>{}
        }
        
        document
}
fn dependency_arrow(from:(i32,i32), to:(i32,i32), horizontal:bool)-> Path{
        let rel_to = (to.0-from.0,to.1-from.1);
        let extends_arrow_path;
        if horizontal{
                let extends_arrow_data = Data::new()
                        .move_to((from.0, from.1)) 
                        .line_by(rel_to)

                        .line_by((rel_to.0 *(-1), -10))
                        .line_by((rel_to.0 , 10))

                        .line_by((rel_to.0*(-1), 10))
                        .line_by((rel_to.0 , -10))
                        .close();

                extends_arrow_path = Path::new()
                        .set("fill", "none")    //Füllung
                        .set("stroke", "black") //Rand farbe
                        .set("stroke-width", 2) //Rand breite
                        .set("d", extends_arrow_data);
        }else {
                let extends_arrow_data = Data::new()
                        .move_to((from.0, from.1)) 
                        .line_by(rel_to)

                        .line_by((-ARROW, rel_to.1 *(-1)))
                        .line_by(( ARROW, rel_to.1))

                        .line_by((ARROW, rel_to.1*(-1)))
                        .line_by((-ARROW, rel_to.1))
                        .close();

                extends_arrow_path = Path::new()
                        .set("fill", "none")    //Füllung
                        .set("stroke", "black") //Rand farbe
                        .set("stroke-width", 2) //Rand breite
                        .set("d", extends_arrow_data);
        }

        extends_arrow_path
}
fn extends_arrow(from:(i32,i32), to:(i32,i32), horizontal:bool)->Path{
        let rel_to = (to.0-from.0,to.1-from.1);
        let extends_arrow_path;
        if horizontal {
                let extends_arrow_data = Data::new()
                        .move_to(from) 
                        .line_by((0, ARROW))                    //10 nach oben
                        .line_by((rel_to.0, rel_to.1 -ARROW))   //zur Spitze
                        .line_by((rel_to.0 * (-1), -ARROW))     //zum linken flügel 
                        .close();                               //zurück zum Pfeil

                extends_arrow_path = Path::new()
                        .set("fill", "white")    //Füllung
                        .set("stroke", "black") //Rand farbe
                        .set("stroke-width", 2) //Rand breite
                        .set("d", extends_arrow_data);

                
        } else {
               let extends_arrow_data = Data::new()
                        .move_to(from) 
                        .line_by((ARROW, 0))                    //10 nach links
                        .line_by((rel_to.0 -ARROW, rel_to.1 ))   //zur Spitze
                        .line_by((-ARROW, rel_to.1 * (-1)))     //zum rechten flügel 
                        .close();                               //zurück zum Pfeil

                extends_arrow_path = Path::new()
                        .set("fill", "white")    //Füllung
                        .set("stroke", "black") //Rand farbe
                        .set("stroke-width", 2) //Rand breite
                        .set("d", extends_arrow_data); 
        }
        
        extends_arrow_path
        
}
fn aggregation_arrow(from:(i32,i32), to:(i32,i32), horizontal:bool)->Path{
        let rel_to : (i32,i32);
        let tmp_from :i32;
        let aggregation_arrow_path;
        if horizontal {
                //Berchnung der werte für move_to, weil die Pfeile länger sind
                if to.0-from.0 >0 {
                        tmp_from= from.0-10;
                        rel_to = (to.0-from.0,to.1-from.1);
                }else {
                        tmp_from= from.0+10;
                        rel_to = (to.0-from.0,to.1-from.1);
                }
        
                let aggregation_arrow_data = Data::new()
                        .move_to((tmp_from,from.1))
                        .line_by((rel_to.0 , ARROW/2))
                        .line_by((rel_to.0 ,-ARROW/2))
                        .line_by((rel_to.0  * (-1), -ARROW/2))
                        .line_by((rel_to.0  * (-1), ARROW/2))
                        .close();

                aggregation_arrow_path = Path::new()
                        .set("fill", "white")    //Füllung
                        .set("stroke", "black") //Rand farbe
                        .set("stroke-width", 2) //Rand breite
                        .set("d", aggregation_arrow_data);
        }else {
                //Berchnung der werte für move_to, weil die Pfeile länger sind
                if to.1-from.1 >0 {
                        tmp_from= from.1-ARROW;
                        rel_to = (to.0-from.0,to.1-from.1);
                }else {
                        tmp_from= from.1+ARROW;
                        rel_to = (to.0-from.0,to.1-from.1);
                }
        
                let aggregation_arrow_data = Data::new()
                        .move_to((from.0,tmp_from))
                        .line_by(( ARROW/2, rel_to.1))
                        .line_by((-ARROW/2, rel_to.1))
                        .line_by((-ARROW/2, rel_to.1  * (-1)))
                        .line_by((ARROW/2, rel_to.1  * (-1)))
                        .close();

                aggregation_arrow_path = Path::new()
                        .set("fill", "white")    //Füllung
                        .set("stroke", "black") //Rand farbe
                        .set("stroke-width", 2) //Rand breite
                        .set("d", aggregation_arrow_data);
        }

        aggregation_arrow_path
}
fn composition_arrow(from:(i32,i32), to:(i32,i32), horizontal:bool)->Path{
        let rel_to : (i32,i32);
        let tmp_from :i32;
        let composition_arrow_path;
        if horizontal {
                //Berchnung der werte für move_to, weil die Pfeile länger sind
                if to.0-from.0 >0 {
                        tmp_from= from.0-10;
                        rel_to = (to.0-from.0,to.1-from.1);
                }else {
                        tmp_from= from.0+10;
                        rel_to = (to.0-from.0,to.1-from.1);
                }
        
                let composition_arrow_data = Data::new()
                        .move_to((tmp_from,from.1))
                        .line_by((rel_to.0 , ARROW/2))
                        .line_by((rel_to.0 ,-ARROW/2))
                        .line_by((rel_to.0  * (-1), -ARROW/2))
                        .line_by((rel_to.0  * (-1), ARROW/2))
                        .close();

                composition_arrow_path = Path::new()
                        .set("fill", "black")    //Füllung
                        .set("stroke", "black") //Rand farbe
                        .set("stroke-width", 2) //Rand breite
                        .set("d", composition_arrow_data);
        }else {
                //Berchnung der werte für move_to, weil die Pfeile länger sind
                if to.1-from.1 >0 {
                        tmp_from= from.1-ARROW;
                        rel_to = (to.0-from.0,to.1-from.1);
                }else {
                        tmp_from= from.1+ARROW;
                        rel_to = (to.0-from.0,to.1-from.1);
                }
        
                let composition_arrow_data = Data::new()
                        .move_to((from.0,tmp_from))
                        .line_by(( ARROW/2, rel_to.1))
                        .line_by((-ARROW/2, rel_to.1))
                        .line_by((-ARROW/2, rel_to.1  * (-1)))
                        .line_by((ARROW/2, rel_to.1  * (-1)))
                        .close();

                composition_arrow_path = Path::new()
                        .set("fill", "black")    //Füllung
                        .set("stroke", "black") //Rand farbe
                        .set("stroke-width", 2) //Rand breite
                        .set("d", composition_arrow_data);
        }

        composition_arrow_path
}
pub fn write(mut document:Document, from:(i32,i32), _string:String, size:i32)->Document{
        let text = Text::new()
        .set("x",from.0)
        .set("y",from.1)
        .set("fill","black")
        .set("font-size",size)
        .add(svg::node::Text::new(_string));

        document = document.add(text);
        document
}