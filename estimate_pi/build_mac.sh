cd estimate_pi/
cargo build --release
cd ..
mv estimate_pi/target/release/libestimate_pi.dylib estimate_pi.so
