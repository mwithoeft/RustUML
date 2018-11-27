use image::{ Rgb, RgbImage};
use std::string::String;
use imageproc::rect::Rect;
use rusttype::{FontCollection, Scale};
use imageproc::drawing::{
    draw_hollow_rect_mut,
    draw_filled_rect_mut,
    draw_line_segment_mut,
    draw_text_mut,
    draw_hollow_circle_mut
    
};

use parsing;

const WIDTH: u32 = 1680;
const HEIGHT: u32 = 720;
const X_MIN: i32 = 300; // mindest abstand x-Achse
const Y_MIN: i32 = 200; // mindest abstand y-Achse
const FONT_SIZE_TITLE: f32 = 20.0;
const FONT_SIZE: f32 = 6.0;
const TITEL_MIN: u32 = 30;
const MIN_RAND_LINKS: u32 = 2;
const FONT_SPACE: u32 = 6;




struct Pngclass{
    id:i32,
    breite: u32,
    hoehe_kopf: u32,
    hoehe_att: u32,
    hoehe_meth: u32,
    rel_point_LORU: ((f32,f32),(f32,f32),(f32,f32),(f32,f32)),
    attribute: Vec<String>,
    methoden: Vec<String>,
    name: String,
    property: String,
    keywords: String
}

fn buildclass(id: i32)->Pngclass{
    Pngclass {
        id,
        breite: 1,
        hoehe_kopf: 0,
        hoehe_att: 0,
        hoehe_meth: 0,
        rel_point_LORU:((0.0,0.0),(0.0,0.0),(0.0,0.0),(0.0,0.0)),
        attribute: Vec::new(),
        methoden: Vec::new(),
        name: String::new(),
        property: String::new(),
        keywords: String::new()
    }
}

fn breite(name: String, attribute: &Vec<String> , methoden: &Vec<String>)-> u32{
    let mut breite : usize;
    breite = name.chars().count() * FONT_SIZE_TITLE as usize;
    for a in attribute {
        if a.chars().count()*FONT_SIZE as usize > breite {
            breite = a.chars().count()*FONT_SIZE as usize;
        }
    }
    for m in methoden{
        if m.chars().count()*FONT_SIZE as usize > breite {
            breite = m.chars().count()*FONT_SIZE as usize;
        }
    }
   
    breite as u32
}

fn hoehe(property: &str ,keywords: &str, attribute: &Vec<String> , methoden: &Vec<String>)-> (u32,u32,u32){
    //Default werte
    let mut hoehe_kopf:u32 = TITEL_MIN;
    let mut hoehe_att:u32 = 0;
    let mut hoehe_meth:u32 = 0;

    if !property.is_empty(){
        hoehe_kopf += FONT_SPACE;
    }
    if !keywords.is_empty(){
        hoehe_kopf += FONT_SPACE;
    }
    hoehe_att += FONT_SPACE;
    for _a in attribute{
        hoehe_att += FONT_SPACE;
    }
    hoehe_meth += FONT_SPACE;
    for _m in methoden{
        hoehe_meth += FONT_SPACE;
    }

    (hoehe_kopf , hoehe_att, hoehe_meth)
}



pub fn build_klassendiagramm( klassen: &mut Vec<parsing::parse_class::Klasse>, mut beziehungen : &mut Vec<parsing::parse_class::Beziehung>){
    let mut image = creat_png();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut id: i32= 0;
    let _black = Rgb ([0u8, 0u8, 0u8]);
    let mut class: Vec<Pngclass>= Vec::new();

    
    

    for s in klassen {
        class.push(buildclass(id));
        berechne_werte(&s._name ,&s._property ,&s._keywords , &mut s._attribute, &mut s._methoden,  id, &mut class );
        image = draw_class(image, &mut class,id, i, j);
        i=i+1;
        id = id + 1;
        if i> 4 {
            i=0;
            j+=1;
        }
    }

    image = draw_relation(image, &mut beziehungen, class);
    image.save("UML.png").unwrap();
    print!("PNG erstellt");
}

fn berechne_werte(name: &str,property: &str , keywords: &str, attribute: &mut Vec<parsing::parse_class::Attribut>, 
                methoden: &mut Vec<parsing::parse_class::Methode>, id: i32, pngclass: &mut Vec<Pngclass>){
   
    for class in pngclass{
        if class.id == id{
            class.attribute = attribut_to_string(attribute);
            class.methoden = methode_to_string(methoden);
            class.breite = breite(name.to_string(), &class.attribute, &class.methoden);
            class.hoehe_kopf = hoehe(property, keywords, &class.attribute, &class.methoden).0;
            class.hoehe_att = hoehe(property, keywords, &class.attribute, &class.methoden).1;
            class.hoehe_meth = hoehe(property, keywords, &class.attribute, &class.methoden).2;
            class.name =name.to_string();
            class.property = property.to_string();
            class.keywords = keywords.to_string();
        }
    }  
}

fn draw_class(image: RgbImage, class: &mut Vec<Pngclass>, id:i32, i:i32, j:i32)-> RgbImage{
    let mut bild = image;
    let x: i32 = 20+i*X_MIN;
    let y: i32 = 70+j*Y_MIN;
    let _black = Rgb ([0u8, 0u8, 0u8]);
    let _red = Rgb([255u8, 0u8, 0u8]);
    let title_scale = Scale { x: FONT_SIZE_TITLE*2.0 , y: FONT_SIZE_TITLE};
    let scale = Scale { x: FONT_SIZE * 2.0 , y: FONT_SIZE};
    let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
    let mut counter: u32 = 2;
    for class in class {
        if class.id == id{
            //berechnete werte für Klassen namen einsetzten und kasten zeichnen
             draw_hollow_rect_mut( &mut bild, Rect::at(x, y                      ).of_size(class.breite, class.hoehe_kopf), _black);
            //berechnete werte für Attribute namen einsetzten und kasten zeichnen
            draw_hollow_rect_mut( &mut bild, Rect::at(x, y+class.hoehe_kopf as i32).of_size(class.breite, class.hoehe_att), _black);
            //berechnete werte für Methoden namen einsetzten und kasten zeichnen
            draw_hollow_rect_mut( &mut bild, Rect::at(x, y+class.hoehe_kopf as i32+class.hoehe_att as i32).of_size(class.breite, class.hoehe_meth), _black);

            class.rel_point_LORU = ((x as f32 ,(y + ((class.hoehe_kopf+class.hoehe_att+class.hoehe_meth)/2)as i32) as f32),
                                    (x as f32 +(class.breite/2) as f32,y as f32),
                                    (x as f32 +class.breite as f32,(y+((class.hoehe_kopf+class.hoehe_att+class.hoehe_meth)/2) as i32) as f32),
                                    (x as f32 +(class.breite/2) as f32,(y +(class.hoehe_kopf+class.hoehe_att+class.hoehe_meth) as i32) as f32));
                                    
           
            //draw_hollow_circle_mut(&mut bild, class.rel_point_LORU.0, 2, _red);
            //draw_hollow_circle_mut(&mut bild, class.rel_point_LORU.1, 2, _red);
            //draw_hollow_circle_mut(&mut bild, class.rel_point_LORU.2, 2, _red);
            //draw_hollow_circle_mut(&mut bild, class.rel_point_LORU.3, 2, _red);
            


            //Klassen namen schreiben x wert = x des linken randes + die Differenz aus der hälfte der breite und die hälft der Wortlänge
            // y wert = y wert des oberen randes + die Differenz aus der gesamten kopf höhe und der konstanten Titel_min
            draw_text_mut(&mut bild, _black, (x + class.breite as i32 / 2 - (class.property.chars().count()*FONT_SIZE as usize /2) as i32) as u32, y as u32+ 3, scale, &font, &class.property);
           
            draw_text_mut(&mut bild, _black, (x + class.breite as i32 / 2 - (class.name.chars().count()*FONT_SIZE_TITLE as usize /2) as i32) as u32, 
                            y as u32+ class.hoehe_kopf - TITEL_MIN, title_scale, &font, &class.name);
            
            draw_text_mut(&mut bild, _black, (x + class.breite as i32 / 2 - (class.keywords.chars().count()*FONT_SIZE as usize /2) as i32) as u32, y as u32+ class.hoehe_kopf - 10 , scale, &font, &class.keywords);
           
            //Attribute schreiben
            for att in &class.attribute {
                draw_text_mut(&mut bild, _black, (x as u32 + MIN_RAND_LINKS ) as u32, 
                                y as u32+ class.hoehe_kopf + counter, scale, &font, &att);
                counter+= FONT_SPACE;

            }
            counter = 2;
            for meth in &class.methoden {
                draw_text_mut(&mut bild, _black, (x as u32 + MIN_RAND_LINKS ) as u32, 
                                y as u32+ class.hoehe_kopf + class.hoehe_att + counter, scale, &font, &meth);
                counter+= FONT_SPACE;

            }


        }
    }
    bild
}

fn draw_relation(image: RgbImage, relation: &mut Vec<parsing::parse_class::Beziehung>, class:  Vec<Pngclass> )-> RgbImage{
    let mut bild = image;
    let mut from: (i32, String, (f32,f32)) = (-1,"".to_string(),(-1.0,-1.0));
    let mut to: (i32, String, (f32,f32)) = (-1,"".to_string(),(-1.0,-1.0));
    let mut temp_point_from: ((f32,f32),(f32,f32),(f32,f32),(f32,f32)) = ((1.0,1.0),(1.0,1.0),(1.0,1.0),(1.0,1.0));
    let mut temp_point_to: ((f32,f32),(f32,f32),(f32,f32),(f32,f32)) = ((1.0,1.0),(1.0,1.0),(1.0,1.0),(1.0,1.0));
    let start_point: i32;
    let end_point: i32;
    let _black = Rgb ([0u8, 0u8, 0u8]);
    let white = Rgb ([255u8, 255u8, 255u8]);
    let scale = Scale { x: FONT_SIZE * 2.0 , y: FONT_SIZE};
    let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();

    for r in relation{
        for c in &class{
            if r._von_klasse_name == c.name{
                 from.0 = c.id;
                 from.1 = c.name.to_string();
                 temp_point_from = c.rel_point_LORU;
            }
            if r._zu_klasse_name == c.name{
                 to.0 = c.id;
                 to.1 = c.name.to_string();
                 temp_point_to = c.rel_point_LORU;
            }
            if !(from.0 == -1 && to.0 == -1){

                if from.0 < to.0{
                    from.2 = temp_point_from.2;
                    from.2 = temp_point_from.2;
                    to.2 = temp_point_to.0;
                    to.2 = temp_point_to.0;
                }else {
                    from.2 = temp_point_from.0;
                    from.2 = temp_point_from.0;
                    to.2 = temp_point_to.2;
                    to.2 = temp_point_to.2;
                }
            }
        }
        draw_line_segment_mut(&mut bild, from.2, to.2, _black);
        

        match r._beziehungstyp{
            parsing::parse_class::Beziehungstyp::EXTENDS => {
                let mut spitze: (f32,f32) = (0.0,0.0);
                let mut lfluegel: (f32,f32) = (0.0,0.0);
                let mut rfluegel: (f32,f32) = (0.0,0.0);

                if(r._von_klasse_pfeil){
                    spitze = from.2;
                    lfluegel = ((from.2).0+10.0,(from.2).1+10.0);
                    rfluegel = ((from.2).0+10.0,(from.2).1-10.0);
                    draw_line_segment_mut(&mut bild, spitze, (((from.2).0)+9.0,(from.2).1), white);
                    draw_line_segment_mut(&mut bild, spitze, lfluegel, _black);
                    draw_line_segment_mut(&mut bild, spitze, rfluegel, _black);
                    draw_line_segment_mut(&mut bild, lfluegel, rfluegel, _black);
                }
                
                if(r._zu_klasse_pfeil){
                    spitze = to.2;
                    lfluegel = ((to.2).0-10.0,(to.2).1+10.0);
                    rfluegel = ((to.2).0-10.0,(to.2).1-10.0);

                    draw_line_segment_mut(&mut bild, to.2, (((to.2).0)-9.0,(to.2).1), white);
                    draw_line_segment_mut(&mut bild, spitze, lfluegel, _black);
                    draw_line_segment_mut(&mut bild, spitze, rfluegel, _black);
                    draw_line_segment_mut(&mut bild, lfluegel, rfluegel, _black);
                }
            }
            parsing::parse_class::Beziehungstyp::IMPLEMENTS=>{
                draw_line_segment_mut(&mut bild, from.2, (((from.2).0)+9.0,(from.2).1), white);
                draw_line_segment_mut(&mut bild, ((from.2).0, (from.2).1), (((from.2).0)+10.0, ((from.2).1)-10.0), _black);
                draw_line_segment_mut(&mut bild, ((from.2).0, (from.2).1), (((from.2).0)+10.0, ((from.2).1)+10.0), _black);
                draw_line_segment_mut(&mut bild, (((from.2).0)+10.0, ((from.2).1)+10.0), (((from.2).0)+10.0, ((from.2).1)-10.0), _black);

                for i in 1..9{
                    draw_line_segment_mut(&mut bild, (((from.2).0)+i as f32, ((from.2).1)), (((from.2).0)+i as f32, ((from.2).1)), white);
                }
            }
            parsing::parse_class::Beziehungstyp::ASSOCIATION=>{
                let mut spitze: (f32,f32) = (0.0,0.0);
                let mut lfluegel: (f32,f32) = (0.0,0.0);
                let mut rfluegel: (f32,f32) = (0.0,0.0);

                if(r._von_klasse_pfeil){
                    spitze = from.2;
                    lfluegel = ((from.2).0+10.0,(from.2).1+10.0);
                    rfluegel = ((from.2).0+10.0,(from.2).1-10.0);
                    draw_line_segment_mut(&mut bild, spitze, lfluegel, _black);
                    draw_line_segment_mut(&mut bild, spitze, rfluegel, _black);
                }
                
                if(r._zu_klasse_pfeil){
                    spitze = to.2;
                    lfluegel = ((to.2).0-10.0,(to.2).1+10.0);
                    rfluegel = ((to.2).0-10.0,(to.2).1-10.0);

                    draw_line_segment_mut(&mut bild, spitze, lfluegel, _black);
                    draw_line_segment_mut(&mut bild, spitze, rfluegel, _black);
                }
            }
            parsing::parse_class::Beziehungstyp::AGGREGATION=>{
                draw_line_segment_mut(&mut bild, from.2, (((from.2).0)+19.0,(from.2).1), white);
                draw_line_segment_mut(&mut bild, ((from.2).0, (from.2).1), (((from.2).0)+9.0, ((from.2).1)-5.0), _black);
                draw_line_segment_mut(&mut bild, ((from.2).0, (from.2).1), (((from.2).0)+9.0, ((from.2).1)+5.0), _black);
                draw_line_segment_mut(&mut bild, (((from.2).0)+10.0, ((from.2).1)+5.0), (((from.2).0)+20.0, (from.2).1), _black);
                draw_line_segment_mut(&mut bild, (((from.2).0)+10.0, ((from.2).1)-5.0), (((from.2).0)+20.0, (from.2).1), _black);
            }
            parsing::parse_class::Beziehungstyp::COMPOSITION=>{
                draw_line_segment_mut(&mut bild, ((from.2).0, (from.2).1), (((from.2).0)+9.0, ((from.2).1)-5.0), _black);
                draw_line_segment_mut(&mut bild, ((from.2).0, (from.2).1), (((from.2).0)+9.0, ((from.2).1)+5.0), _black);
               
                draw_line_segment_mut(&mut bild, (((from.2).0)+10.0, ((from.2).1)+5.0), (((from.2).0)+20.0, (from.2).1), _black);
                draw_line_segment_mut(&mut bild, (((from.2).0)+10.0, ((from.2).1)-5.0), (((from.2).0)+20.0, (from.2).1), _black);

                for i in 1..5{

                    draw_line_segment_mut(&mut bild, (((from.2).0)+i as f32 *2.0,((from.2).1)+i as f32), (((from.2).0)+20.0-i as f32*2.0, ((from.2).1)+i as f32), _black);
                    draw_line_segment_mut(&mut bild, (((from.2).0)+i as f32 *2.0,((from.2).1)-i as f32), (((from.2).0)+20.0-i as f32*2.0, ((from.2).1)-i as f32), _black);

                  // draw_line_segment_mut(&mut bild, (((from.2).0)+20.0-i as f32, (from.2).1), (((from.2).0)+20.0-i as f32, ((from.2).1)+i as f32), _black);
                   // draw_line_segment_mut(&mut bild, (((from.2).0)+20.0-i as f32, (from.2).1), (((from.2).0)+20.0-i as f32, ((from.2).1)-i as f32), _black);
                }
                
            }
            parsing::parse_class::Beziehungstyp::DEPENDENCY=>{
                 let mut spitze: (f32,f32) = (0.0,0.0);
                let mut lfluegel: (f32,f32) = (0.0,0.0);
                let mut rfluegel: (f32,f32) = (0.0,0.0);

                if(r._von_klasse_pfeil){
                    spitze = from.2;
                    lfluegel = ((from.2).0+10.0,(from.2).1+10.0);
                    rfluegel = ((from.2).0+10.0,(from.2).1-10.0);
                    draw_line_segment_mut(&mut bild, spitze, lfluegel, _black);
                    draw_line_segment_mut(&mut bild, spitze, rfluegel, _black);
                }
                
                if(r._zu_klasse_pfeil){
                    spitze = to.2;
                    lfluegel = ((to.2).0-10.0,(to.2).1+10.0);
                    rfluegel = ((to.2).0-10.0,(to.2).1-10.0);

                    draw_line_segment_mut(&mut bild, spitze, lfluegel, _black);
                    draw_line_segment_mut(&mut bild, spitze, rfluegel, _black);
                }
            }
            parsing::parse_class::Beziehungstyp::UNDEFINED=>{}
        }


        if !(r._von_klasse_mult.is_empty()) {
            let mut p: (u32,u32) = (0,0);

            p.0 = (from.2).0 as u32 +20;
            p.1 = (from.2).1 as u32 -10;

            draw_text_mut(&mut bild, _black, p.0, p.1, scale, &font, &r._von_klasse_mult);;

        }
        if !(r._zu_klasse_mult.is_empty()){
            let mut p: (u32,u32) = (0,0);

            p.0 = (to.2).0 as u32 -(FONT_SIZE as u32*r._zu_klasse_mult.chars().count()as u32 ) ;
            p.1 = (to.2).1 as u32 -10;

            draw_text_mut(&mut bild, _black, p.0, p.1, scale, &font, &r._zu_klasse_mult);;
        }

        
    }



    bild
}



fn attribut_to_string(attribute: &mut Vec<parsing::parse_class::Attribut>)-> Vec<String>{
    let mut attribute_string: Vec<String> = Vec::new();
    for a in attribute {
        let mut string = String::from("");
        string.push_str(&a._name);
        if a._final {
            string = string.to_uppercase();
        }
        string.insert_str(0, &a._modifikator.to_string());
        string.push_str(":");
        string.push_str(&a._datentyp);
        if !a._wert.is_empty(){
            string.push_str(" = ");
            string.push_str(&a._wert);
        }
        attribute_string.push(string);
    }
    attribute_string
}

fn methode_to_string(methode: &mut Vec<parsing::parse_class::Methode>)-> Vec<String>{
    let mut methoden_string: Vec<String> = Vec::new();
    for a in methode {
        let mut string = String::from("");
        string.push_str(&a._name);
        if a._final {
            string = string.to_uppercase();
        }
        string.insert_str(0, &a._modifikator.to_string());
        string.push_str("(");
        if !a._parameter.is_empty(){
            for p in &a._parameter {
                string.push_str(&p.to_string());
                string.push_str(",");

            }
        }
        string.push_str("):");
        string.push_str(&a._returntyp);
        
        methoden_string.push(string);
    }
    methoden_string
}

fn creat_png() ->RgbImage{
    let white = Rgb ([255u8, 255u8, 255u8]);
    let mut image = RgbImage::new(WIDTH, HEIGHT);
    draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(WIDTH, HEIGHT), white);  
    image
}