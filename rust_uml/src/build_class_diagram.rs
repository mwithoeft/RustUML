use image::{ Rgb, RgbImage};
use std::string::String;
use imageproc::rect::Rect;
use rusttype::{FontCollection, Scale};
use imageproc::drawing::{
    draw_hollow_rect_mut,
    draw_filled_rect_mut,
    draw_line_segment_mut,
    draw_text_mut
    
};

use parsing;

const WIDTH: u32 = 1680;
const HEIGHT: u32 = 720;
const C_NAME_HIGHT: u32 = 60;
const x: i32 = 250; // mindest abstand x-Achse
const y: i32 = 300; // mindest abstand y-Achse


fn breite(name: String, attribute: &Vec<String> , methoden: &Vec<String>)-> usize{
    let mut breite : usize;
    breite = name.chars().count();
    for a in attribute {
        if a.chars().count() > breite {
            breite = a.chars().count();
        }
    }

    for m in methoden{
        if m.chars().count() > breite {
            breite = m.chars().count();
        }
    }
    breite*5
}

fn hoehe(property: &str ,keywords: &str, attribute: &Vec<String> , methoden: &Vec<String>)-> (i32,i32,i32){
    let mut hoehe_kopf:i32 = 60;
    let mut hoehe_att:i32 = 0;
    let mut hoehe_meth:i32 = 0;
    if !property.is_empty(){
        hoehe_kopf += 10;
    }
    if !keywords.is_empty(){
        hoehe_kopf += 10;
    }
    hoehe_att += 10;
    for a in attribute{
        hoehe_att += 10;
    }
    for m in methoden{
        hoehe_meth += 10;
    }
    hoehe_meth+=10;

    (hoehe_kopf as i32 , hoehe_att as i32, hoehe_meth as i32)
}

pub fn build_klassendiagramm(mut klassen: &mut Vec<parsing::parse_class::Klasse>, mut beziehungen : &mut Vec<parsing::parse_class::Beziehung>){
    let mut image = creat_png();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let black = Rgb ([0u8, 0u8, 0u8]);
    
    

    for s in klassen {
        let w_tuple = berechne_werte(&s._name ,&s._property ,&s._keywords , &mut s._attribute, &mut s._methoden);

        //berechnete werte für Klassen namen einsetzten und kasten zeichnen
        draw_hollow_rect_mut(&mut image, Rect::at(20+i*x, 70+j*y                      ).of_size(w_tuple.0 as u32, (w_tuple.1).0 as u32), black);
        //berechnete werte für Attribute namen einsetzten und kasten zeichnen
        draw_hollow_rect_mut(&mut image, Rect::at(20+i*x, 70+j*y+(w_tuple.1).0 as i32).of_size(w_tuple.0 as u32, (w_tuple.1).1 as u32), black);
        //berechnete werte für Methoden namen einsetzten und kasten zeichnen
        draw_hollow_rect_mut(&mut image, Rect::at(20+i*x, 70+j*y+(w_tuple.1).0+(w_tuple.1).1 as i32).of_size(w_tuple.0 as u32, (w_tuple.1).2 as u32), black);

        i=i+1;
        if i> 6 {
            i=0;
            j+=1;
        }
    }
    image.save("UML.png").unwrap();
}

fn berechne_werte(name: &str,property: &str , keywords: &str, attribute: &mut Vec<parsing::parse_class::Attribut>, 
                methoden: &mut Vec<parsing::parse_class::Methode>)->( usize, (i32,i32,i32), Vec<String>, Vec<String>){
    let mut attribute_string: Vec<String> = attribut_to_string(attribute);
    let mut methoden_string: Vec<String> = methode_to_string(methoden);

    println!("{}",breite(name.to_string(), &attribute_string, &methoden_string));
    println!("kopf:{}",hoehe(property, keywords, &attribute_string, &methoden_string).0);
    println!("attribute:{}",hoehe(property, keywords, &attribute_string, &methoden_string).1);
    println!("methoden:{}",hoehe(property, keywords, &attribute_string, &methoden_string).2);

    ( breite(name.to_string(), &attribute_string, &methoden_string), hoehe(property, keywords, &attribute_string, &methoden_string), attribute_string, methoden_string)
}

fn png_bearbeiten(bild: &mut RgbImage){
   
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

fn beispielbild(){
    let white = Rgb ([255u8, 255u8, 255u8]);
    
    let mut image = RgbImage::new(WIDTH, HEIGHT);

    let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();

    let height = 12.4;
    let scale = Scale { x: height * 2.0, y: height };
    let scale_attribut = Scale { x: height , y: height/2.0};
    let black = Rgb ([0u8, 0u8, 0u8]);

    //Weiß machen
    draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(WIDTH, HEIGHT), white);



    //Linkes Klassen Diagramm
    draw_hollow_rect_mut(&mut image, Rect::at(20, 70).of_size(150, 170), black);
    draw_hollow_rect_mut(&mut image, Rect::at(20, 130).of_size(150, 50), black);
    //Attribute
    draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), 70, 90, scale, &font, "Auto");
    draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), 25, 135, scale_attribut, &font, "+ anzahlTüren : int");
    draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), 25, 145, scale_attribut, &font, "+ ps : int");
    draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), 25, 155, scale_attribut, &font, "+ km : int");
    draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), 25, 165, scale_attribut, &font, "+ tüv : boolean");
    //Methoden
    draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), 21, 185, scale_attribut, &font, "+ fahren(strecke: int) : void");
    draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), 21, 195, scale_attribut, &font, "+ tanken(sorte: String) : int");



    //Rechtes Klassen Diagramm
    draw_hollow_rect_mut(&mut image, Rect::at(300, 20).of_size(150, 250), black);
    draw_hollow_rect_mut(&mut image, Rect::at(300, 70).of_size(150, 75), black);
    draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), 330, 40, scale, &font, "Händler");



    //Beziehung
    draw_line_segment_mut(&mut image, (170f32, 135f32), (300f32, 135f32), black);
    draw_line_segment_mut(&mut image, (290f32, 145f32), (300f32, 135f32), black);
    draw_line_segment_mut(&mut image, (290f32, 125f32), (300f32, 135f32), black);


    


    image.save("UML.png").unwrap();
}
