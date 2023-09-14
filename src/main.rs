use tch::{Tensor, kind};
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use polars::{prelude::*, lazy::dsl::any_horizontal};

fn string_length(sorted_magnitudes: [f32; 3]) -> f64{
    for mag in sorted_magnitudes {
        println!("{mag}");
    }
    10.0
}

fn read_gaia_lightcurve(file_path: &str) -> PolarsResult<DataFrame> {
    LazyCsvReader::new(file_path)
        .has_header(true)
        .finish()?
        .select([col("time"), col("mag")])
        .filter(any_horizontal([col("*")]).is_not_null())
        .collect()
}

#[no_mangle]
pub extern "system" fn  Java_MyFirstRustClass_example<'local>(mut env: JNIEnv<'local>, class: JClass<'local>) {
    example(1.0);
}

fn fold(time: f64, period: f64) -> f64{
    (time % period)/period
}

fn example(period: f64){
    let lf = read_gaia_lightcurve("DR3711991473282863616.csv").unwrap();
    //let phase = lf.column("time")?.f64()?.apply(|t| fold(t, period));
    let filtered = lf.lazy().select([col("mag").sort_by([col("time")], [false])]).collect();
    println!("{:?}", filtered);
    let t = Tensor::zeros(&[2], kind::FLOAT_CPU);
    //t.print();
    let model = tch::CModule::load("traced_model.pt").unwrap();
    model.forward_ts(&[t]).unwrap().print();

}


fn main() {
    string_length([10.0, 20.0, 30.0]);
    example(1.0);
}
