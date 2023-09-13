//use std::error::Error;
//use serde::Deserialize;
use polars::prelude::*;

fn string_length(sorted_magnitudes: [f32; 3]) -> f64{
    for mag in sorted_magnitudes {
        println!("{mag}");
    }
    10.0
}


fn read_gaia_lightcurve(file_path: &str) -> PolarsResult<DataFrame> {
    CsvReader::from_path(file_path)?
        .has_header(true)
        .finish()
}

fn main() {
    string_length([10.0, 20.0, 30.0]);
    let lf = read_gaia_lightcurve("DR3711991473282863616.csv")
        .unwrap().select(["time", "mag"]).unwrap();
    let filtered: DataFrame = lf.drop_nulls::<String>(None).unwrap();
    println!("{:?}", filtered);

}
