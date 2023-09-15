# Interfaces between Java Rust and Torchscript

Rust code can be called from java through the Java Native Interface (JNI) by exploiting the [jni crate](https://crates.io/crates/jni). 

The first half of the example shows how to do some compute in Rust and exposes it in Java through a class.

Then there is [torchscript](https://pytorch.org/docs/stable/jit.html) which can serialize a Pytorch model (trained in regular Python). This model can be then called using the TorchScript C++ API. C/C++ code can be called from Rust and indeed there is already a [crate](https://github.com/LaurentMazare/tch-rs) with Rust bindings for the Torchscript API. 

The script `model.py` creates a simple PyTorch model and serialize it. To run it I suggest:

    conda create -n torch_rust pytorch==2.0.0 cpuonly -c pytorch
    conda activate torch_rust
    python model.py

A file `traced_model.py` should appear in the root of this repo.

After that add the following to your `LD_LIBRARY_PATH` env variable:

    $CONDA_PREFIX/lib/python3.10/site-packages/torch/lib/

Then do compile the crate with `cargo build` or `cargo run` if you want to execute the binary (`main.rs`).

Now to call this from java

    javac main.java
    java -Djava.library.path=target/debug/ MyFirstRustClass

Change to `/target/release` if you passed `--release` to `cargo build`.
