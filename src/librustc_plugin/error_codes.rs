<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
use syntax::{register_diagnostics, register_long_diagnostics};

register_long_diagnostics! {

}

register_diagnostics! {
    E0498  // malformed plugin attribute
=======
syntax::register_diagnostics! {
;
    E0498,  // malformed plugin attribute
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}
