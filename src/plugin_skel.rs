// TODO snippet this

use bevy::prelude::*;
pub struct TestPlug;

impl Plugin for TestPlug {
    fn build(&self, app: &mut App) {
        app.add_system(test_hello);
    }
}

fn test_hello() {
    println!("HELLO");
}
