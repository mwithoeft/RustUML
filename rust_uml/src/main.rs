
mod check_file;
mod get_diagram_type;

fn main() {
    //Eingabe des Dateinamens:
    let filepath = check_file::get_file_path();

    get_diagram_type::read_file(filepath);
}



