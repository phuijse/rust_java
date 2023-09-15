use jni::JNIEnv;
use jni::objects::{JClass, JString};
use polars::{prelude::*, lazy::dsl::any_horizontal};
mod gaia;
mod torch;

#[no_mangle]
pub extern "system" fn  Java_MyFirstRustClass_example<'local>(mut env: JNIEnv<'local>, class: JClass<'local>) {
    exposed_to_java();
}

pub fn exposed_to_java(){
    let lf: DataFrame = gaia::read_lightcurve("DR3711991473282863616.csv").unwrap();
    //let phase = lf.column("time")?.f64()?.apply(|t| fold(t, period));
    let filtered = lf.lazy().select([col("mag").sort_by([col("time")], [false])]).collect();
    println!("{:?}", filtered);
    println!("{:?}", torch::inference().print());
}

