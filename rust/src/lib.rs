use godot::prelude::*;
mod player;
mod test_obj;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
