use std::{sync::{Arc, Mutex}}; // 複数のスレッドで安全にデータを共有するために Arc と Mutex を使用
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait}; // CPAL のデバイス制御用トレイトをインポート
use hound; // WAV ファイルの入出力を行うためのクレート

fn main() {
    // オーディオホストを取得（OS に依存するデフォルトのオーディオ管理）
    let host = cpal::default_host();
    
    // デフォルトの入力デバイス（マイクなど）を取得
    let device = host.default_input_device().expect("Failed to get default input device");
    println!("Using input device: {}", device.name().unwrap());
    
    // 入力デバイスのデフォルト設定を取得（サンプルレートやチャンネル数など）
    let config = device.default_input_config().expect("Failed to get default input config");
    let sample_rate = config.sample_rate().0; // サンプリングレート（Hz）
    let channels = config.channels() as u16; // チャンネル数（1=モノラル, 2=ステレオ）
    println!("Default input format: {:?}", config);
    
    // 音声データを格納するための共有バッファ（スレッド間で安全にアクセス可能）
    let audio_data = Arc::new(Mutex::new(Vec::<f32>::new()));
    // let audio_data = Arc::new(Mutex::new(Vec::<i16>::new())); // i16の場合
    let audio_data_clone = Arc::clone(&audio_data);
    
    // 音声入力ストリームを作成
    let stream = device.build_input_stream(
        &config.into(), // 設定を適用
        // 音声データを受け取るコールバック。
        // move |data: &[i16], _: &cpal::InputCallbackInfo| { // i16の場合
        move |data: &[f32], _: &cpal::InputCallbackInfo| { // f32の場合
            // println!("Received sample: {:?}", &data[0..10]); // 先頭10個を表示
            let mut buffer = audio_data_clone.lock().unwrap(); // ミューテックスで保護
            buffer.extend_from_slice(data); // バッファにデータを追加
        },
        move |err| { // エラーハンドリング用のコールバック
            eprintln!("Error occurred: {}", err);
        },
        None,
    ).expect("Failed to build input stream");
    
    // ストリームを開始（録音を開始）
    stream.play().expect("Failed to start stream");
    println!("Recording... Press Enter to stop.");
    
    // ユーザーが Enter を押すまで待機（録音を継続）
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    // 録音したデータを WAV ファイルとして保存
    save_to_wav(&audio_data.lock().unwrap(), sample_rate, channels);
    println!("Recording saved to output.wav");
}

// 録音データを WAV ファイルとして保存する関数
fn save_to_wav(audio_data: &[f32], sample_rate: u32, channels: u16) { // f32の場合
// fn save_to_wav(audio_data: &[i16], sample_rate: u32, channels: u16) { // i16の場合
    // WAV ファイルは通常 PCM（Pulse Code Modulation） 形式で音声データを保存します。この PCM 形式には、以下のような一般的なビット深度（量子化ビット数）があります：
    // 16-bit PCM（標準的な CD 音質）
    // 24-bit PCM（高音質オーディオ向け）
    // 32-bit PCM（高精度オーディオ向け、あまり一般的ではない）
    // 32-bit Floating Point（プロ仕様 DAW では使用されることも）
    // ほとんどの 一般的なオーディオプレーヤーやソフトウェア（Windows Media Player、Audacity など）は 16-bit PCM を標準とする ため、 i16 に変換することで広い互換性を持たせています。 
    let spec = hound::WavSpec {
        channels, // チャンネル数（モノラル or ステレオ）
        sample_rate, // サンプリングレート（Hz）
        bits_per_sample: 16, // 16ビット PCM
        sample_format: hound::SampleFormat::Int, // 整数型の PCM データ
    };
    
    // WAV ファイルを作成
    let mut writer = hound::WavWriter::create("output.wav", spec).expect("Failed to create WAV file");
    
    // 録音データを WAV に書き込む（f32 -> i16 に変換）
    for &sample in audio_data {
        // cpal は標準で f32 のフォーマットを使用し、値の範囲は -1.0 から 1.0 です。
        // i16 の範囲は -32768 から 32767 なので、変換時に f32 * 32767 を行うことで適切なスケーリングを行います。
        let sample_i16 = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16; // f32の場合
        writer.write_sample(sample_i16).unwrap();

        // writer.write_sample(sample).unwrap(); // i16の場合
    }
}

