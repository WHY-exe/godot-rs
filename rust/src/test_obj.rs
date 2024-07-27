use godot::{classes::Node2D, prelude::*};

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Test {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Test {
    fn init(base: Base<Node2D>) -> Self {
        Test { base }
    }
}
