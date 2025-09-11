use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // カメラの追加
    commands.spawn(Camera2d);

    // テキストの追加
    commands.spawn((
        Text::new("Hello, Bevy!"),
        TextFont {
            font_size: 50.0,
            ..Default::default()
        },
    ));
}
