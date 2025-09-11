use bevy::prelude::*;
use std::sync::{
    Mutex,
    mpsc::{self, Receiver, Sender},
};
use std::thread;
use std::time::Duration;

/// 画面に表示され、更新対象となるテキストを示すためのマーカーコンポーネント。
#[derive(Component)]
struct UpdatableText;

/// MPSCチャンネルの受信側(`Receiver`)を保持するためのリソース。
/// これにより、Bevyのシステム内からチャンネルにアクセスできるようになります。
#[derive(Resource)]
struct TextUpdateReceiver(Mutex<Receiver<String>>);

fn main() {
    // 文字列を送信・受信するためのMPSCチャンネルを作成します。
    // tx (Sender) は別のスレッドに渡され、rx (Receiver) はBevyアプリのリソースとして登録されます。
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    // テキストを定期的に送信するための新しいスレッドを起動します。
    thread::spawn(move || {
        let mut count = 0;
        loop {
            // 1秒ごとにカウンターを含んだメッセージを送信します。
            let message = format!("Message count: {count}");
            println!("Sending: {message}");
            if tx.send(message).is_err() {
                // 受信側が破棄された場合（例: アプリが終了した場合）、ループを抜けます。
                println!("Receiver has been dropped. Exiting thread.");
                break;
            }
            count += 1;
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Bevyアプリケーションを構築します。
    App::new()
        .add_plugins(DefaultPlugins)
        // 作成したチャンネルの受信側(rx)をリソースとしてアプリに挿入します。
        .insert_resource(TextUpdateReceiver(Mutex::new(rx)))
        // アプリ起動時に一度だけ実行されるセットアップシステムを登録します。
        .add_systems(Startup, setup)
        // 毎フレーム実行されるテキスト更新システムを登録します。
        .add_systems(Update, update_text_from_channel)
        .run();
}

/// アプリの起動時にカメラと初期テキストをセットアップするシステム。
fn setup(mut commands: Commands) {
    // 2Dシーンを映すためのカメラをスポーンします。
    commands.spawn(Camera2d);
    commands.spawn((
        // 初期テキストを設定します。
        Text::new("Waiting for messages..."),
        TextFont {
            font_size: 50.0,
            ..Default::default()
        },
        // このエンティティが更新対象のテキストであることを示すマーカーコンポーネントを追加します。
        UpdatableText,
    ));
}

/// MPSCチャンネルからメッセージを受信し、画面のテキストを更新するシステム。
fn update_text_from_channel(
    // リソースに登録されたTextUpdateReceiverを取得します。
    receiver: Res<TextUpdateReceiver>,
    // UpdatableTextマーカーを持つエンティティからTextコンポーネントをクエリします。
    mut query: Query<&mut Text, With<UpdatableText>>,
) {
    // MutexでラップされたReceiverをロックしてからtry_recvを呼び出します。
    if let Ok(rx) = receiver.0.lock() {
        if let Ok(new_message) = rx.try_recv() {
            println!("Received: {new_message}");
            // 新しいメッセージを受信した場合、クエリで取得したすべてのテキストコンポーネントを更新します。
            for mut text in &mut query {
                text.0 = new_message.clone();
            }
        }
    }
}
