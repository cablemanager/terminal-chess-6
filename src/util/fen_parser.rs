use std::fs;

pub struct Fen {
    unparsed_fen_vector: Vec<String>,
    board_fen: String,
}   

impl Fen {
    pub fn read_fen(&mut self) {
    let contents = fs::read_to_string("./board_layouts/fen_strings/default_fen.txt")
            .expect("File is read.");

        let parts = contents.split(" ");
        let collection = parts.collect::<Vec<&str>>();
        let strings: Vec<String> = collection.iter().map(|&s|s.into()).collect();

        self.unparsed_fen_vector = strings;
        }

    pub fn parse_board_layout(&mut self)
    {
        let unparsed_layout: &String = &self.unparsed_fen_vector[0];
        println!("{}", unparsed_layout);
    }
}

fn main() {
}
