use std::collections::HashMap;

fn main() -> Result<(), csv::Error> {
    let mut rdr = csv::Reader::from_path("data.csv")?;
    for result in rdr.deserialize() {
        let record: HashMap<String, String> = result?;
        println!(
            "msgid: {:?}", record["msgid"],
        );
    }
    Ok(())
}