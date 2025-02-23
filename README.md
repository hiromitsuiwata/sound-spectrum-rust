# sound-spectrum-rust

## サンプルコードのコピー元

- https://qiita.com/kamiro/items/5493dd109b7cc5043814
- https://github.com/RustAudio/cpal
- https://zenn.dev/yutannihilation/articles/bb89785ceaad77
- https://github.com/RustAudio/cpal/tree/master/examples
- https://github.com/RustAudio/dasp
- https://github.com/Joylei/plotters-iced/
- https://github.com/iced-rs/iced
- https://qiita.com/GreasySlug/items/1678c4b1e4b6e0721bf3


## Linux環境構築

```bash
sudo apt -y install alsa pulseaudio libasound2-dev pkg-config
export PULSE_SERVER=unix:$(sed 's/unix://g' <<< "$PULSE_SERVER")
```

## CPAL examples実行

```bash
cargo run --bin beep
cargo run --bin feedback
cargo run --bin record_wav
cargo run --bin record_wav_windows
cargo run --bin synth_tones
```
