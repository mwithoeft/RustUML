#![recursion_limit="2048"]
#[macro_use]
extern crate yew;
extern crate stdweb;
extern crate regex;
extern crate rusttype;
extern crate svg;

use yew::prelude::*;
use stdweb::*;
use std::boxed::Box;

mod build_class_svg;
mod parsing;
mod svglib;
mod get_diagram_type;
mod examples;


pub struct Model {
    text_area: String
}

pub enum Msg {
    GotInput(String),
    SelectType,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            text_area: "".into()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotInput(new_value) => {
                self.text_area = new_value;
                work_on_input(&self.text_area);
            }

            Msg::SelectType => {
                let s : String = get_type();
                if s == "class" {insert_at_caret("eingabefeld", examples::CLASS_EXAMPLE);}
                else if s == "usecase" {insert_at_caret("eingabefeld", examples::USECASE_EXAMPLE);}
                else if s == "action" {}
                else if s == "sequence" {}
                else if s == "state" {}
                else if s == "component" {}
                else if s == "package" {}
                else if s == "deployment" {}
                else if s == "object" {}
                js!(document.getElementById("selectBox").selectedIndex = "0";);
                self.text_area = (js!{return document.getElementById("eingabefeld").value}).into_string().unwrap();
                work_on_input(&self.text_area);
            }

        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div id="mantel",>
                <div id="bild", data-counter=0,></div>
                <div id="textfeld",>
                    <textarea rows=5,
                        value=&self.text_area,
                        oninput=|e| Msg::GotInput(e.value),
                        placeholder="Hier können Sie ihre Spezifikation eintragen",
                        id="eingabefeld",>
                    </textarea>
                </div>
                <div id="buttons",>
                    <select id="selectBox", onchange=|_| Msg::SelectType,>
                        <option value="", disabled=true, required=true,>{"Beispieleingaben:"}</option>
                        <option value="class",>{"Klassendiagramm"}</option>
                        <option value="usecase",>{"Anwendungsfalldiagramm"}</option>
                        <option value="action",>{"Aktivitätsdiagramm"}</option>
                        <option value="sequence",>{"Sequenzdiagramm"}</option>
                        <option value="state",>{"Zustandsdiagramm"}</option>
                        <option value="component",>{"Komponentendiagramm"}</option>
                        <option value="package",>{"Paketdiagramm"}</option>
                        <option value="deployment",>{"Verteilungsdiagramm"}</option>
                        <option value="object",>{"Objektdiagramm"}</option>
                    </select>
                </div>
            </div>
        }
    }
}

fn get_type() -> String {
    (js!{
        var selectBox = document.getElementById("selectBox");
        var selectedValue = selectBox.options[selectBox.selectedIndex].value;
        return selectedValue}).into_string().unwrap()
}

fn work_on_input(input: &str) {

    let mut diagramme: Vec<get_diagram_type::RawDiagram> = Vec::new();
    get_diagram_type::get_diagram(&mut diagramme, &input);


    js!{
        var bild = document.getElementById("bild");
        while (bild.firstChild) {
            bild.removeChild(bild.firstChild);
        }
        var tab = document.createElement("div");
        tab.classList.add("tab");
        bild.appendChild(tab);

    }

    for (number, diagramm) in diagramme.iter().enumerate() {

        let counter = (js!{var tmp = document.getElementById("bild").dataset.counter;return tmp}).into_string().unwrap();

        match diagramm._typ {
            get_diagram_type::DiagramType::CLASS => {
                js!(console.log("Parse Class!"));     
                let mut klassen: Vec<parsing::parse_class::Klasse> = Vec::new();
                let mut beziehungen_class: Vec<parsing::parse_class::Beziehung> = Vec::new();
                parsing::parse_class::parse(&diagramm._input, &mut klassen, &mut beziehungen_class);
                let mut svg : String = build_class_svg::build_klassendiagramm(&mut klassen, &mut beziehungen_class).to_string();
                make_tabs(number, counter, svg);
            }

            get_diagram_type::DiagramType::USECASE => {

                //let mut typen: Vec<parsing::parse_usecase::Typ> = Vec::new();
                //let mut beziehungen_usecase: Vec<parsing::parse_usecase::Beziehung> = Vec::new();

                js!(console.log("Parse Usecase!")); 
                
                //DATEIEN NICHT MEHR VORHANDEN - NUR ZUR KENNZEICHNUNG //parsing::parse_usecase::parse(&mut vektor, &mut typen, &mut beziehungen_usecase);
                //DATEIEN NICHT MEHR VORHANDEN - NUR ZUR KENNZEICHNUNG //build_usecase_diagram::build_usecase_diagramm(&mut typen, &mut beziehungen_usecase);
            }

            get_diagram_type::DiagramType::ACTION => { js!(console.log("Parse Action!"));  }
            get_diagram_type::DiagramType::SEQUENCE => { js!(console.log("Parse Sequence!"));  }
            get_diagram_type::DiagramType::STATE => { js!(console.log("Parse State!"));  }
            get_diagram_type::DiagramType::COMPONENT => { js!(console.log("Parse Component!"));  }
            get_diagram_type::DiagramType::PACKAGE => { js!(console.log("Parse Package!"));  }
            get_diagram_type::DiagramType::DEPLOYMENT => { js!(console.log("Parse Deployment!"));  }
            get_diagram_type::DiagramType::OBJECT => { js!(console.log("Parse Object!"));  }
            get_diagram_type::DiagramType::NOTFOUND => { js!(console.log("NOTFOUND!"));  }
        }

    }
}

fn make_tabs(number: usize, counter: String, svg: String) {
    js!{
        var panOptions = {
            viewportSelector: ".svg-pan-zoom_viewport",
            panEnabled: true, 
            controlIconsEnabled: false, 
            zoomEnabled: true, 
            dblClickZoomEnabled: true, 
            mouseWheelZoomEnabled: true, 
            preventMouseEventsDefault: true, 
            zoomScaleSensitivity: 0.2, 
            minZoom: 0.5, 
            maxZoom: 10,
            fit: false, 
            contain: true,
            center: false, 
            refreshRate: "auto"
        };

        var button = document.createElement("button");
        button.classList.add("tablinks");
        if (@{number.to_string()} == @{&counter.to_string()}) {
            button.classList.add("active");
        }

        button.onclick = function(){

            var i, tabcontent, tablinks;
            // Hole alle Elemente mit der Klasse "tabcontent" and verstecke sie, entferne panzoom
            tabcontent = document.getElementsByClassName("tabcontent");
            for (i = 0; i < tabcontent.length; i++) {
                if (i == document.getElementById("bild").dataset.counter) {
                    svgPanZoom(tabcontent[i].firstChild).destroy();
                    tabcontent[i].style.display = "none";
                    var newParent = tabcontent[i].firstChild;
                    var oldParent = newParent.firstChild;
                    while (oldParent.childNodes.length > 0) {
                        newParent.appendChild(oldParent.childNodes[0]);
                    }
                }

            }

            /*Entferne den alten pan-viewport*/
            var vp = document.getElementsByClassName("svg-pan-zoom_viewport");
            while(vp[0]) {
                vp[0].parentNode.removeChild(vp[0]);
            }

            //Hole alle Elemente mit der Klasse "tablinks" und entferne die Klasse "active"
            var p = 0;
            tablinks = document.getElementsByClassName("tablinks");
            for (i = 0; i < tablinks.length; i++) {
                tablinks[i].className = tablinks[i].className.replace(" active", "");
                if (tablinks[i] == this) {
                    this.classList.add("active");
                    p = i;
                    var tmp = document.getElementById("bild");
                    tmp.dataset.counter = i;
                }
            }
            var tc = document.getElementsByClassName("tabcontent");
            for (i = 0; i <= p; i++) {
                if (i == p) {
                    tc[i].style.display = "block";
                    tc[i].firstChild.setAttribute("viewBox", "0 0 1680 720");
                    svgPanZoom(tc[i].firstChild, panOptions);
                }
            }
        };

        button.value = "d_" + @{(number+1).to_string()};
        button.innerHTML = "Klassendiagramm " + @{(number+1).to_string()};
        var tab = document.getElementsByClassName("tab");
        for (var i = 0; i < tab.length; i++) {
            tab[i].appendChild(button);
        }

        var tabcontent = document.createElement("div");
        tabcontent.classList.add("tabcontent");
        tabcontent.id = "d_" + @{(number+1).to_string()};
        tabcontent.innerHTML = @{svg};
        document.getElementById("bild").appendChild(tabcontent);

        if (@{number.to_string()} == @{&counter.to_string()}) {
            tabcontent.style.display = "block";
            svgPanZoom(tabcontent.firstChild, panOptions);
        }
    }
}

fn insert_at_caret(area_id: &str, text: &str) {
    js!{
        var txtarea = document.getElementById(@{area_id});
		var scrollPos = txtarea.scrollTop;
		var strPos = 0;
		var br = ((txtarea.selectionStart || txtarea.selectionStart == '0') ? 
			"ff" : (document.selection ? "ie" : false ) );
		if (br == "ie") { 
			txtarea.focus();
			var range = document.selection.createRange();
			range.moveStart ("character", -txtarea.value.length);
			strPos = range.text.length;
		}
		else if (br == "ff") strPos = txtarea.selectionStart;
	
		var front = (txtarea.value).substring(0,strPos);  
		var back = (txtarea.value).substring(strPos,txtarea.value.length); 
		txtarea.value=front+@{text}+back;
		strPos = strPos + @{text}.length;
		if (br == "ie") { 
			txtarea.focus();
			var range = document.selection.createRange();
			range.moveStart ("character", -txtarea.value.length);
			range.moveStart ("character", strPos);
			range.moveEnd ("character", 0);
			range.select();
		}
		else if (br == "ff") {
			txtarea.selectionStart = strPos;
			txtarea.selectionEnd = strPos;
			txtarea.focus();
		}
		txtarea.scrollTop = scrollPos;
    }
}