use polars::{lazy::dsl::any_horizontal, prelude::*};

pub fn read_lightcurve(file_path: &str) -> PolarsResult<DataFrame> {
    LazyCsvReader::new(file_path)
        .has_header(true)
        .finish()?
        .select([col("time"), col("mag")])
        .filter(any_horizontal([col("*")]).is_not_null())
        .collect()
}

fn fold(time: f64, period: f64) -> f64 {
    (time % period) / period
}

//fn periodogram(sorted_magnitudes: [f32; 3]) -> Vec<f64> {
//    for mag in sorted_magnitudes {
//        println!("{mag}");
//    }
//    10.0
//}
