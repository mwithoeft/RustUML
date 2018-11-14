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

fn breite(name: String, attribut: String, methode: String)-> usize{
    let bname:usize = name.chars().count();
    let aname:usize = attribut.chars().count();
    let mname:usize = methode.chars().count();
    if bname > aname && bname > mname{
        return bname;
    }else if aname > mname{
        return aname;
    }else{
        return mname;
    }

    


}

/*fn höhe()-> (usize,usize){





}*/

fn test_breite(){
    let name = String::from("Auto");
    let attribut = String::from("- anzahlRäder:int");
    let methode = String::from("+fahren():void ");
    println!("This is {} neat", breite(name,attribut ,methode ));
}

pub fn build_klassendiagramm(mut klassen: &mut Vec<parsing::parse_class::Klasse>, mut beziehungen : &mut Vec<parsing::parse_class::Beziehung>){

    for s in klassen {
        println!("{}", s._name);
    }



}

fn beispielbild(){
    let white = Rgb ([255u8, 255u8, 255u8]);
    let black = Rgb ([0u8, 0u8, 0u8]);
    let mut image = RgbImage::new(WIDTH, HEIGHT);

    let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();

    let height = 12.4;
    let scale = Scale { x: height * 2.0, y: height };
    let scale_attribut = Scale { x: height , y: height/2.0};
    

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

fn build_png() {


}