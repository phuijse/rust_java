use jni::objects::{JClass, JString};
use jni::JNIEnv;
mod gaia;
mod torch;

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
    let lc: gaia::LightCurve = gaia::read_lightcurve(&file_path);
    gaia::periodogram(lc);
    //println!("{:?}", torch::inference().print());
}
