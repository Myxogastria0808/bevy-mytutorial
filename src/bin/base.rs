use bevy::prelude::*;

fn main() {
    // このApp structを実行することで、Bevyアプリケーションが開始される。
    // ビルダーパターンを使用して、アプリケーションの設定を行う。
    App::new().add_systems(Startup, hello_world).run();
}

fn hello_world() {
    print!("Hello, world!");
}
