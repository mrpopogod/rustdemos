// Note: the way the language server works with VS Code if you create a main.rs and a lib.rs in the same src folder
// you'll need to restart the language server once both are up or VS Code won't properly understand how to handle both
// (seen through things like not knowing about the lib module and not getting intellisense in one of the files).
// Fixed by hitting F1 and typing in rust and selecting the "Restart the language server" option
use adder::Rectangle;

fn main() {
    let r = Rectangle {
        width: 8,
        height: 7,
    };

    println!("We made a rectangle {:?}", r);
}
