use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, assets_server: Res<AssetServer>) {
    // カメラの追加
    commands.spawn(Camera2d);

    // 画像の追加
    // デフォルトで、assetsディレクトリが参照される
    commands.spawn(Sprite::from_image(assets_server.load("sample.png")));
}
