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

// Resource Traitを実装した構造体
// Resourceは、システム間で共有されるユニークなデータを表す。
// つまり、グローバルなデータはResourceとして管理される。
#[derive(Resource)]
struct GreetTimer(Timer);

// Res ... 不変なリソースをシステムに注入するために使用される。
// ResMut ... 可変なリソースをシステムに注入するために使用される。
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("position: {}", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resourceの追加
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            // systemでResourceを使用できるようになる
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
}
