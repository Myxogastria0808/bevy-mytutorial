use bevy::prelude::*;

// ECS (Entity Component System)

fn main() {
    App::new()
        // Start ... アプリケーションの開始時に一度だけ実行されるシステム
        .add_systems(Startup, add_people)
        // Update ... 毎フレーム実行されるシステム
        // 基本的には、タプルはできる限り並列で処理されるが、
        // chain() メソッドを使用すると、タプルの順に実行されるように強制できる。
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}

// Component
// コンポーネントは、できるだけシンプルで小さく保つべきである。
// 複数のコンポーネントを組み合わせることで、具体的なオブジェクトを作成する。
/*
- 良くない例
まだ、分割できる余地がある。
#[derive(Component)]
Person {
    name: String,
    age: u32
};

- 良い例
細かく分割されている。
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Age(u32);
*/
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

//* Startup System *//
// アプリケーションの開始時に一度だけ実行されるシステム
// command.spawn() の分だけ並列に実行される。
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Alice".to_string())));
    commands.spawn((Person, Name("Bob".to_string())));
    commands.spawn((Person, Name("Tom".to_string())));
}

//* Update System *//
// 毎フレーム実行されるシステム
// シンプルなパターン
fn hello_world() {
    println!("Hello, world!");
}

// Queryを使ったパターン
fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("position: {}", name.0);
    }
}

// 可変なQueryを使ったパターン
// 以下のようにして、Queryの中身を変更できる。
fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Alice" {
            name.0 = "Elena".to_string();
            break;
        }
    }
}
