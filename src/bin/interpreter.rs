use bevy::prelude::*;
use bevy_mytutorial::interpreter::{CustomHandler, MockInstruction, MockInterpreter};

/// 画面に表示されるテキストを示すためのマーカーコンポーネント。
#[derive(Component)]
struct TextId(usize);

// Resource Traitを実装した構造体
// Resourceは、システム間で共有されるユニークなデータを表す。
// つまり、グローバルなデータはResourceとして管理される。
#[derive(Resource)]
struct ScriptInterpreter(MockInterpreter);

// 今回は結果が観測しやすいように定期実行を秒おきなどにする
#[derive(Resource)]
struct ClockTimer(Timer);

fn main() {
    // ファイルから実行バイナリを読んでくる
    // ここでは簡単のため以下のデータの列だとする
    // これを下の通りインタプリタの初期化時に突っ込む
    let code = vec![
        MockInstruction::ShowText(0, "Hello, bevy world!".to_string()),
        MockInstruction::ShowText(1, "インタプリタの解釈に従って表示しています".to_string()),
        MockInstruction::DeleteText(0),
    ];

    // Bevyアプリケーションを構築します。
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // Resourceの追加
        .insert_resource(ScriptInterpreter(MockInterpreter::new(code)))
        .insert_resource(ClockTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        // 毎フレーム実行されるインタプリタ逐次実行イベントを登録します。
        .add_systems(Update, update_with_interpreting_script)
        .run();
}

/// アプリの起動時にカメラと初期テキストをセットアップするシステム。
fn setup(mut commands: Commands) {
    // 2Dシーンを映すためのカメラをスポーンします。
    commands.spawn(Camera2d);
}

// Res ... 不変なリソースをシステムに注入するために使用される。
// ResMut ... 可変なリソースをシステムに注入するために使用される。
fn update_with_interpreting_script(
    time: Res<Time>,
    mut timer: ResMut<ClockTimer>,
    mut commands: Commands,
    mut intp: ResMut<ScriptInterpreter>,
    // mut query: Query<(&TextId, &mut Text)>,
    // mut query: Query<&mut Text, With<TextId>>,
    query: Query<(Entity, &mut TextId)>,
) {
    // 今回は結果が観測しやすいように定期実行を秒おきなどにする
    if timer.0.tick(time.delta()).just_finished() {
        // インタプリタに次のバイトコードを処理させる
        // プログラムカウンタが内部的に更新される
        // つまり、ここでスクリプト言語の逐次実行を実現している
        if let Some(action) = intp
            .0
            .next_with_custom_handler(CustomHandler::new(Box::new(interpreter_custom_handler)))
        {
            match action {
                UpdateAction::ShowText(id, text) => {
                    commands.spawn((
                        Text::new(text),
                        TextFont {
                            font_size: 50.0,
                            ..Default::default()
                        },
                        id,
                    ));
                }
                UpdateAction::DeleteText(id) => {
                    for (entity, eid) in query.iter() {
                        if eid.0 == id.0 {
                            commands.entity(entity).despawn();
                        }
                    }
                }
            }
        }
    }
}

enum UpdateAction {
    ShowText(TextId, String),
    DeleteText(TextId),
}

// インタプリタに逐次実行をさせているだけだと完全に閉じた状態変化しか起こらず意味がない
// そこで、外部(このコードで言うところのこのモジュールに記述されているbevyをいじっている部分)に影響を与えるため、
// 実際の処理をカスタムハンドラ関数で記述して実行させられるようにした
fn interpreter_custom_handler(intp: &mut MockInterpreter) -> Option<UpdateAction> {
    if let Some(inst) = intp.code.get(intp.pc) {
        intp.pc += 1;

        match inst {
            MockInstruction::ShowText(id, text) => {
                println!("ShowText({id}, {text})");
                Some(UpdateAction::ShowText(TextId(*id), text.clone()))
            }
            MockInstruction::DeleteText(id) => {
                println!("DeleteText({id})");
                Some(UpdateAction::DeleteText(TextId(*id)))
            }
        }
    } else {
        println!("code may already ended");
        None
    }
}
