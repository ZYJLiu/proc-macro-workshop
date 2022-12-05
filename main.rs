use seq::seq;

seq!(N in 0..4 {
    compile_error!(concat!("error number ", stringify!(N)));
});

fn main() {}
