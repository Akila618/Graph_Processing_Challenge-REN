use std::{error::Error};

//===================== read csv and return a values as string vector ============================
pub fn read_csv() -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    let input = match args.get(1) {
        Some(path) => path,
        None => {
            println!("Error: Missing file path argument.");
            return Err("Missing required file path argument".into());
        }
    }; 

    // let input = "../test_files/graph2.csv";
    
    let file = std::fs::File::open(input)?; 

    //[skip headers if there is one exists]
    let mut rdr = csv::ReaderBuilder::new().has_headers(false) .from_reader(file);

    let mut csv_results:Vec<Vec<i32>> = Vec::new();

    let records = rdr.records();

    csv_results.extend(records.map(|r| {

        r.unwrap().iter().map(|s| {
            s.to_string().parse::<i32>().expect("failed to convert String to i32")
        }).collect()

    }));
    
    println!("retrieved csv successfully!");
    
    Ok(csv_results)
}

