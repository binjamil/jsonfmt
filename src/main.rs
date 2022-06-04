fn main() {
    if let Err(e) = jsonfmt::get_args().and_then(|s| jsonfmt::run(&s)) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    // match jsonfmt::get_args() {
    //     Ok(s) => {
    //         if let Err(e) = jsonfmt::run(&s) {
    //             eprintln!("{}", e);
    //             std::process::exit(1);
    //         }
    //     }
    //     Err(e) => {
    //         eprintln!("{}", e);
    //         std::process::exit(1);
    //     }
    // }
}
