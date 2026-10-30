// Build a plugin

mod bindings {
    use wit_bindgen::generate;
    use super::Component;

    // wit_bindgen::generate!({
    //     world: "greeter",
    //     path: "wit",
    // });

    generate!();

    export!(Component);
}

/// Component off of which implementation will hang (this can be named anything)
struct Component;

use crate::bindings::exports::liminal::plugin::greet::Guest;

impl Guest for Component {
    fn greet(name: String) -> String {
        format!("Hello {name}!")
    }

    fn hello() -> String { 
        format!("THIS IS NOT A COMPUTER")
    }
}
