use std::fs::File;
use std::io::ErrorKind;

fn main() {

    let my_file = File::open("sample2.txt");

    let file = match my_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("Hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("There was a problem creating file {e:?}"),
            },

            other_error => {
                panic!("Problem opening the file {other_error}");
            }
        },
    };

}
