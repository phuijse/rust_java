use polars::{lazy::dsl::any_horizontal, prelude::*};

pub struct LightCurve {
    time: Vec<f64>,
    mag: Vec<f64>
}

pub fn read_lightcurve(file_path: &str) -> LightCurve {
    let df = LazyCsvReader::new(file_path)
        .has_header(true)
        .finish().unwrap()
        .filter(col("band").eq(lit("G")))
        .select([col("time"), col("mag")])
        .filter(any_horizontal([col("*")]).is_not_null())
        .collect().unwrap();
    let mag: Vec<f64> = df["mag"].f64().unwrap().into_no_null_iter().collect();
    let time: Vec<f64> = df["time"].f64().unwrap().into_no_null_iter().collect();
    LightCurve {time, mag}
}


fn argsort(data: &Vec<f64>) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by(|&i, &j| data[i].partial_cmp(&data[j]).unwrap());
    indices
}

pub fn periodogram(lc: &LightCurve) {
        let n = lc.mag.len() as f64;
    let average: f64 = lc.mag.iter().sum::<f64>()/n;
    let variance: f64 = lc.mag.iter().map(|m| (m-average).powf(2.0)).sum::<f64>()/(n-1.0);
    let mut trial_frequency: f64 = 0.001;
    let nf = 1_000_000;
    for _i in 0..nf {
        let phase: Vec<f64> = lc.time.iter().map(|t| (t % trial_frequency.powf(-1.0))).collect();
        let folded_indices: Vec<usize> = argsort(&phase);
    
        let mut string_length: f64 = 0.0;
        for i in 1..lc.mag.len() {
            string_length += (&lc.mag[folded_indices[i]]-&lc.mag[folded_indices[i-1]]).powf(2.0);
        }
        string_length = string_length/(2.0*n*variance);
        println!("{trial_frequency},{string_length}");
        trial_frequency += 1e-5;
    }
     

}
