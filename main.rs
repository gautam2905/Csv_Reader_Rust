use std::error::Error;
use csv;

fn read_from_csv(path: &str) -> Result<(),Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    for result in reader.records(){
        let record = result;
        println!("{:?}",record);
    }
    Ok(())
}

fn main() {
    if let Err(e) = read_from_csv("./submission.csv"){
        eprint!("{}",e);
    }
    // println!("Hello, world!");
}
