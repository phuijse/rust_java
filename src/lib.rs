use jni::objects::{JClass, JString};
use jni::JNIEnv;
mod gaia;
mod torch;
use polars::{lazy::dsl::any_horizontal, prelude::*};

#[no_mangle]
pub extern "system" fn Java_MyFirstRustClass_example<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
) {
    exposed_to_java();
}

pub fn exposed_to_java() {
    let lf = gaia::read_lightcurve("DR3711991473282863616.csv").unwrap();
    let folded_magnitude = lf.lazy().select([col("mag").sort_by([col("time") % lit(1.0)], [false])]).collect();
    println!("{:?}", folded_magnitude);
    
    //println!("{:?}", torch::inference().print());
}
