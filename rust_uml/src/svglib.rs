use std::string::String;
use svg::Document;
use svg::node::element::Path;
use svg::node::element::Rectangle;
use svg::node::element::Line;
use svg::node::element::Text;
use svg::node::element::path::Data;

const ARROW:i32 = 10;


pub fn rechteck(mut document:Document,x:i32, y:i32, width:i32, height:i32)->Document{
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
pub fn association(mut document:Document,from:(i32,i32), to:(i32,i32), f_arrow:bool, t_arrow:bool, horizontal:bool)->Document{
        document=document.add(line(from, to));
        if horizontal{
                if f_arrow{
                        document = document.add(dependency_arrow((from.0+ARROW,from.1), from, horizontal));
                }
                if t_arrow{
                        document = document.add(dependency_arrow((to.0-ARROW,to.1), to, horizontal));
                }
        }else {
                if f_arrow{
                        document = document.add(dependency_arrow((from.0,from.1+ARROW), from, horizontal));
                }
                if t_arrow{
                        document = document.add(dependency_arrow((to.0,to.1-ARROW), to, horizontal));
                }
        }

        document
}
pub fn extends(mut document:Document,from:(i32,i32), to:(i32,i32), f_arrow:bool, t_arrow:bool, horizontal:bool)->Document{
        document = document.add(line(from, to));
        if horizontal {
                if f_arrow{
                        document = document.add(extends_arrow((from.0+ARROW,from.1), from, horizontal));
                }
                if t_arrow{
                        document = document.add(extends_arrow((to.0-ARROW,to.1), to, horizontal));
                }
        }else {
                if f_arrow{
                        document = document.add(extends_arrow((from.0,from.1+ARROW), from, horizontal));
                }
                if t_arrow{
                        document = document.add(extends_arrow((to.0,to.1-ARROW), to, horizontal));
                }
        }
        document
}
pub fn implements(mut document:Document, from:(i32,i32), to:(i32,i32), f_arrow:bool, t_arrow:bool, horizontal:bool)->Document{
        document = document.add(broken_line(from ,to));
        if horizontal {
                if f_arrow{
                        document = document.add(extends_arrow((from.0+ARROW,from.1), from, horizontal));
                }
                if t_arrow{
                        document = document.add(extends_arrow((to.0-ARROW,to.1), to, horizontal));
                }
        }else {
                if f_arrow{
                        document = document.add(extends_arrow((from.0,from.1+ARROW), from, horizontal));
                }
                if t_arrow{
                        document = document.add(extends_arrow((to.0,to.1-ARROW), to, horizontal));
                }
        }
        document
}
pub fn aggregation(mut document:Document, from:(i32,i32), to:(i32,i32), f_arrow:bool, t_arrow:bool, horizontal:bool)->Document{
        document = document.add(line(from, to));
        if horizontal{
                if f_arrow{
                        document = document.add(aggregation_arrow((from.0+ARROW,from.1), from, horizontal));
                }
                if t_arrow{
                        document = document.add(aggregation_arrow((to.0-ARROW,to.1), to, horizontal));
                }
        }else {
                if f_arrow{
                        document = document.add(aggregation_arrow((from.0,from.1+ARROW), from, horizontal));
                }
                if t_arrow{
                        document = document.add(aggregation_arrow((to.0,to.1-ARROW), to, horizontal));
                }
        }
        document
}
pub fn composition(mut document:Document, from:(i32,i32), to:(i32,i32), f_arrow:bool, t_arrow:bool, horizontal:bool)->Document{
        document = document.add(line(from, to));
        if horizontal{
                if f_arrow{
                        document = document.add(composition_arrow((from.0+ARROW,from.1), from, horizontal));
                }
                if t_arrow{
                        document = document.add(composition_arrow((to.0-ARROW,to.1), to, horizontal));
                }
        }else {
                if f_arrow{
                        document = document.add(composition_arrow((from.0,from.1+ARROW), from, horizontal));
                }
                if t_arrow{
                        document = document.add(composition_arrow((to.0,to.1-ARROW), to, horizontal));
                }
        }
        document
}
pub fn dependency(mut document:Document, from:(i32,i32), to:(i32,i32), f_arrow:bool, t_arrow:bool, horizontal:bool)->Document{
        document = document.add(broken_line(from ,to));
        if horizontal {
                if f_arrow{
                        document = document.add(dependency_arrow((from.0+ARROW,from.1), from, horizontal));
                }
                if t_arrow{
                        document = document.add(dependency_arrow((to.0-ARROW,to.1), to, horizontal));
                }
        }else {
                if f_arrow{
                document = document.add(dependency_arrow((from.0,from.1+ARROW), from, horizontal));
                }
                if t_arrow{
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