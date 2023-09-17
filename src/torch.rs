use tch::{kind, CModule, Tensor};

pub fn inference() -> Tensor {
    let input: Tensor = Tensor::zeros(&[2], kind::FLOAT_CPU);
    let model: CModule = CModule::load("traced_model.pt").unwrap();
    model.forward_ts(&[input]).unwrap()
}
