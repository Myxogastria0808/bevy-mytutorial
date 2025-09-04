use bevy::prelude::*;

fn main() {
    App::new()
        // DefaultPlugins ... Bevyのデフォルトのプラグインを追加する。
        // これには、ウィンドウ管理、レンダリング、入力処理などが含まれる。
        .add_plugins(DefaultPlugins)
        // HelloPlugin ... 自作のプラグインを追加する。
        .add_plugins(HelloPlugin)
        .run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Alice".to_string())));
    commands.spawn((Person, Name("Bob".to_string())));
    commands.spawn((Person, Name("Tom".to_string())));
}

fn hello_world() {
    println!("Hello, world!");
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("position: {}", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Alice" {
            name.0 = "Elena".to_string();
        }
    }
}

// Pluginsの基本の型
// pub struct HelloPlugin;

// impl Plugin for HelloPlugin {
//     fn build(&self, app: &mut App) {
//         ここにプラグインのシステムやリソースを追加するコードを書く
//         引数のappは、Bevyアプリケーションのインスタンスである。
//     }
// }

// Pluginsの使用例
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people)
            .add_systems(Update, (hello_world, (update_people, greet_people).chain()));
    }
}
