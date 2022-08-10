baseDir=$(cd $(dirname $0);pwd)
rustDir=$baseDir/../person-recognition

scp $rustDir/Cargo.toml rasp4:/home/pi/person-recognition-opencv-rust/person-recognition/Cargo.toml
scp $rustDir/Cargo.lock rasp4:/home/pi/person-recognition-opencv-rust/person-recognition/Cargo.lock
scp $rustDir/haarcascades/* rasp4:/home/pi/person-recognition-opencv-rust/person-recognition/haarcascades/
scp $rustDir/src/main.rs rasp4:/home/pi/person-recognition-opencv-rust/person-recognition/src/
