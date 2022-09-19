# Description
An app that recognizes people with a webcam and sends messages to Slack.

## Environment

### Raspberry Pi 4B

- webcam
- RAM: 8GB
- OS: Raspberry Pi OS(64-bit, released at 2022-04-04)
  ![image](https://user-images.githubusercontent.com/40833633/180515676-a9fde88c-f066-4025-92c8-7189d42661c5.png)


### package install

```
sudo apt update
sudo apt upgrade
sudo apt install -y \
   build-essential \
   ccache \
   cmake \
   clang \
   pkg-config \
   llvm \
   libclang-dev \
   libssl-dev \
   unzip \
   libjpeg-dev libpng-dev libtiff-dev \
   libavcodec-dev libavformat-dev libswscale-dev libv4l-dev\
   libxvidcore-dev libx264-dev \
   libatlas-base-dev gfortran
  ```

### Rust

https://www.rust-lang.org/ja/tools/install

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### OpenCV

- version: 4.6.0
- install document: https://docs.opencv.org/4.6.0/d7/d9f/tutorial_linux_install.html

  ```
  wget -O opencv.zip https://github.com/opencv/opencv/archive/refs/tags/4.6.0.zip
  wget -O opencv_contrib.zip https://github.com/opencv/opencv_contrib/archive/refs/tags/4.6.0.zip

  unzip opencv.zip
  unzip opencv_contrib.zip

  mkdir ~/opencv-build
  cd ~/opencv-build
  cmake -DOPENCV_EXTRA_MODULES_PATH=~/opencv_contrib-4.6.0/modules ~/opencv-4.6.0

  ## Takes about 4 hours
  cmake --build .
  sudo make install
  ```

## .env

```
cp .env.sample .env
```

- HAARCASCADES_FILE

  `frontalface` or `upperbody` or `frontalface_alt` or `fullbody`

- SLEEP_TIME(seconds)

  interval time

- WITH_WINDOW

  Set if you want to launch the app window

## Running the app

- build
  ```
  cargo build
  ```
- run
  ```
  cargo run
  ```
