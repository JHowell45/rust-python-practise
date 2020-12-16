cd template/
cargo build --release
cd ..
mv template/target/release/libtemplate.dylib template.so
