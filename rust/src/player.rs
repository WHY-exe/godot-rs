use godot::{
    engine::{ISprite2D, Sprite2D},
    obj::{Base, WithBaseField},
    register::{godot_api, GodotClass},
};

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    #[export]
    speed: f64,
    angular_speed: f64,
    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }
}

#[godot_api]
impl Player {
    #[func]
    fn do_process(&mut self, delta: f64) {
        let radian = (delta * self.angular_speed) as f32;
        self.base_mut().rotate(radian);
    }
}
