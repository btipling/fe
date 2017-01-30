use display;

pub fn v (msg: String, options: &super::Options) {
    if options.verbose { display::print_log_message(&msg[..]); }
}

pub fn vv (msg: String, options: &super::Options) {
    if options.very_verbose { display::print_debug_message(&msg[..]); }
}