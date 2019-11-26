extern crate csv;
use csv::Error;

fn main() -> Result<(), Error> {
    let csv = "year,make,model,description
1948,Porsche,356,Luxury sports
1967,Ford,Mustang fastback 1967,American
1999,Toyota,Corolla sedan,Japanese";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for record in reader.records() {
        let record = record?;
        println!(
            "In {}, {} built the {} model. It is a {} car.",
            &record[0],
            &record[1],
            &record[2],
            &record[3]
        );
    }

    Ok(())
}

