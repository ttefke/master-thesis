mkdir -p src
svd2rust -i soc602_reg.svd --config svd2rust-conf.toml --output-dir=src --target riscv
form -i src/lib.rs -o "./src"
cargo fmt

# Wrap no_mangle into unsafe
sed -i "s|#\[no_mangle\]|#\[unsafe(no_mangle)\]|g" src/lib.rs

# Add unsafe for steal function
sed -i "/pub unsafe fn steal() -> Self {/a\
unsafe {" src/lib.rs
echo "}" >> src/lib.rs
cargo fmt