use jni::objects::{JClass, JString};
use jni::JNIEnv;
mod gaia;
mod torch;
use polars::{lazy::dsl::any_horizontal, prelude::*};

#[no_mangle]
pub extern "system" fn Java_MyFirstRustClass_exposed_1function<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    argument: JString<'local>
) {
    let file_path: String = env.get_string(&argument).expect("Error parsing java argument").into();
    exposed_to_java(file_path);
}

pub fn exposed_to_java(file_path: String) {
    let lf = gaia::read_lightcurve(&file_path).unwrap();
    let folded_magnitude = lf.lazy().select([col("mag").sort_by([col("time") % lit(1.0)], [false])]).collect();
    println!("{:?}", folded_magnitude);
    
    //println!("{:?}", torch::inference().print());
}
