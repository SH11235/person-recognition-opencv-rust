# person-recognition-opencv-rust

## person-recognition

WebカメラとOpenCVを使って人物認識→画像出力するアプリ（Rust）

## slack-notification

`person-recognition`で保存した画像をSlackに送信するアプリ（Python）

## リリース

`scp`でRaspberry Piに必要なファイルを送信します。
SSH接続設定は各々の環境に合わせてください。

- person-recognition

  ```sh
  # ファイルを更新
  script/release-person-recognition.sh
  # sshしてアプリのbuild、再起動
  ssh rasp4
  cd person-recognition-opencv-rust/person-recognition
  # 起動しているPIDを調べてkill
  ps aux | grep person
  kill $PID
  # opencvのbuildに5～10分かかる
  cargo build --release
  # アプリをnohupで起動
  nohup ./target/release/person-recognition &         
  ```

- slack-notification

  ```sh
  release-slack-notification.sh
  ```
  crontabで定期実行しているのでファイル更新のみ。

- 全部

  ```sh
  release.sh
  ```

- crontab
  
  ```sh
  # crontabファイルを更新
  scp crontab rasp4:/home/pi/person-recognition-opencv-rust
  # ssh
  ssh rasp4
  # rootユーザーで作業
  sudo su
  cd /etc
  # 過去のcrontabのbackup
  mv crontab crontab$yyyymmdd # yyyymmddは作業日を入れる
  mv /home/pi/person-recognition-opencv-rust/crontab crontab
  # 所有者、権限を適切なものにする
  chown root:root crontab
  chmod 644 crontab
  # 適用
  crontab crontab
  # 確認
  crontab -l
  ```
