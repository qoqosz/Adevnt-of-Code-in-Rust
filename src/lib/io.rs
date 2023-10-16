/// Load input data either from a given path or from the 1<sup>st</sup> cmd line arg.
#[macro_export]
macro_rules! load_input {
    () => {
        match std::env::args().nth(1) {
            Some(fname) => load_input!(fname),
            _ => {
                eprint!("No input specified!");
                std::process::exit(1);
            }
        }
    };

    ($p:expr) => {
        std::fs::read_to_string($p).expect("Unable to read input")
    };
}
