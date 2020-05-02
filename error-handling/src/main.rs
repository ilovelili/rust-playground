use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let f = File::open("i.dont.exist");
    let file = match f {
        Ok(file) => file,
        // The ref in the pattern is needed so error is not moved into the guard condition
        // & matches a reference and gives you its value, but ref matches a value and gives you a reference to it. (in other words & and ref are opposites.)
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("i.dont.exist")
        {
            Ok(fc) => fc,
            Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
        },
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };
}

fn read_username_from_file() -> Result<String, Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// ?
fn read_username_from_file_use_question_mark_operator() -> Result<String, Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
