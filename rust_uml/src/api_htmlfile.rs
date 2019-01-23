use yew::prelude::*;
use stdweb::web::{IElement, INode, IParentNode, document};
use stdweb::*;
use std::boxed::Box;

use build_class_svg;
use build_usecase_svg;
use parsing;
use get_diagram_type;


pub fn run_yew() {
    
    yew::initialize();

    let body = document().query_selector("body").unwrap().unwrap();

    let mount_class = "rust_uml";
    let mount_point = document().create_element("div").unwrap();
    mount_point.class_list().add(mount_class).unwrap();
    body.append_child(&mount_point);

    App::<Model>::new().mount(mount_point);
    yew::run_loop();
}

pub struct Model {
    text_area: String,
    dltype: DownloadType
}

pub enum Msg {
    Parse,
    SelectDownloadType,
    Load,
    LoadAll
}

pub enum DownloadType {
    SVG,
    PNG
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            text_area: "".into(),
            dltype: DownloadType::SVG
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {

            Msg::Parse => {
                self.text_area = (js!{var input = document.getElementById("uml_diagramm");return String(input.innerHTML)}).into_string().unwrap();
                work_on_input(&self.text_area);
            }

            Msg::SelectDownloadType => {
                let s : String = get_dl_type();
                if s == "svg" {self.dltype = DownloadType::SVG;}
                else if s == "png" {self.dltype = DownloadType::PNG;}
            }

            Msg::Load => {
                match self.dltype {
                    DownloadType::SVG => {
                        download_svg((js!{return document.getElementById("bild").dataset.counter}).into_string().unwrap());
                    }
                    DownloadType::PNG => {
                        download_png((js!{return document.getElementById("bild").dataset.counter}).into_string().unwrap());
                    }
                }
            }

            Msg::LoadAll => {
                let anzahl_str = (js!{var tabcontent = document.getElementsByClassName("tabcontent");return String(tabcontent.length)}).into_string().unwrap();
                let anzahl: i32 = anzahl_str.parse().unwrap();
                for x in 0..anzahl {
                    match self.dltype {
                        DownloadType::SVG => {
                            download_svg(x.to_string());
                        }
                        DownloadType::PNG => {
                            download_png(x.to_string());
                        }
                    }
                }
            }

        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div id="mantel",>
                <div id="bild", data-counter=0, ondoubleclick=|_| Msg::Parse,>{"Klick mich doppelt!"}</div>
                <select id="selectDLBox", onchange=|_| Msg::SelectDownloadType,>
                    <option value="svg",>{"SVG"}</option>
                    <option value="png",>{"PNG"}</option>
                </select>
                <div id="dlelements",>
                    <button class="dlbutton", onclick=|_| Msg::Load,>{"Ausgewähltes downloaden"}</button>
                    <button class="dlbutton", onclick=|_| Msg::LoadAll,>{"Alle downloaden"}</button>
                </div>
            </div>
        }
    }
}

fn get_dl_type() -> String {
    (js!{
        var selectBox = document.getElementById("selectDLBox");
        var selectedValue = selectBox.options[selectBox.selectedIndex].value;
        return selectedValue}
    ).into_string().unwrap()
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

        document.getElementById("selectDLBox").style.display = "inline";
        var dlbuttons = document.getElementsByClassName("dlbutton");
        for (var i = 0; i < dlbuttons.length; i++) {
            dlbuttons[i].style.display = "inline";
        }
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
                make_tabs(number, counter, svg, &diagramm._name);
            }

            get_diagram_type::DiagramType::USECASE => {
                js!(console.log("Parse Usecase!")); 
                let mut typen: Vec<parsing::parse_usecase::Typ> = Vec::new();
                let mut beziehungen_usecase: Vec<parsing::parse_usecase::Beziehung> = Vec::new();
                parsing::parse_usecase::parse(&diagramm._input, &mut typen, &mut beziehungen_usecase);
                let mut svg : String = build_usecase_svg::build_usecase_diagramm(&mut typen, &mut beziehungen_usecase).to_string();
                make_tabs(number, counter, svg, &diagramm._name);
            }

            get_diagram_type::DiagramType::ACTION => { js!(console.log("Parse Action!"));  }
            get_diagram_type::DiagramType::SEQUENCE => { js!(console.log("Parse Sequence!"));  }
            get_diagram_type::DiagramType::STATE => { js!(console.log("Parse State!"));  }
            get_diagram_type::DiagramType::COMPONENT => { js!(console.log("Parse Component!"));  }
            get_diagram_type::DiagramType::PACKAGE => { js!(console.log("Parse Package!"));  }
            get_diagram_type::DiagramType::DEPLOYMENT => { js!(console.log("Parse Deployment!"));  }
            get_diagram_type::DiagramType::OBJECT => { js!(console.log("Parse Object!"));  }
            get_diagram_type::DiagramType::NOTFOUND => { 
                js!{
                    console.log("NOTFOUND!");
                    document.getElementById("bild").dataset.counter = 0;
                    document.getElementById("selectDLBox").style.display = "none";
                    var dlbuttons = document.getElementsByClassName("dlbutton");
                    for (var i = 0; i < dlbuttons.length; i++) {
                        dlbuttons[i].style.display = "none";
                    }
                }   
            }
        }

    }
}

fn make_tabs(number: usize, counter: String, svg: String, diagrammname: &str) {
    js!{
        var panOptions = {
            viewportSelector: ".svg-pan-zoom_viewport",
            panEnabled: true, 
            controlIconsEnabled: false, 
            zoomEnabled: true, 
            dblClickZoomEnabled: false, 
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
        button.innerHTML = @{diagrammname.to_string()};
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

fn download_png(number: String){
    js!{
        var svg = document.getElementsByClassName("tabcontent")[@{number}].cloneNode(true);

        var newParent = svg.firstChild;
        var oldParent = svg.firstChild.firstChild;
        while (oldParent.childNodes.length > 0) {
            newParent.appendChild(oldParent.childNodes[0]);
        }
        oldParent.remove();
        svg = svg.innerHTML;

        var canvas = document.createElement("canvas");
        canvas.height = 1080;
	    canvas.width = 1920;
        var ctx = canvas.getContext("2d");
        var image = new Image();
        image.src = "data:image/svg+xml," + svg;


        image.onload = function() {
            ctx.drawImage(image, 0, 0);
            var a = document.createElement("a");
            a.download = "diagramm.png";
            a.href = canvas.toDataURL("image/png");
            a.click();
        };

    }
}


fn download_svg(number: String){
    js!{
        var svg = document.getElementsByClassName("tabcontent")[@{number}].cloneNode(true);

        var newParent = svg.firstChild;
        var oldParent = svg.firstChild.firstChild;
        while (oldParent.childNodes.length > 0) {
            newParent.appendChild(oldParent.childNodes[0]);
        }
        oldParent.remove();

        svg = svg.firstChild;

        var serializer = new XMLSerializer();
        var source = serializer.serializeToString(svg);

        source = "<?xml version='1.0' standalone='no'?>\r\n" + source;
        var url = "data:image/svg+xml;charset=utf-8,"+encodeURIComponent(source);

        var a = document.createElement("a");
        a.download = "diagramm.svg";
        a.href = url;
        a.click();

    }
}