extern crate csv;
use csv::Error;
extern crate chrono;
//use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;


// Rust's error handling revolves around returning Result<T, E> and using ? to propagate errors. 
 fn main() -> Result<(), Error> {
	 // https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/question-mark-in-main-and-tests.html
//fn main()  {
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

