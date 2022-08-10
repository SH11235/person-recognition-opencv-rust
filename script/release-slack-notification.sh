baseDir=$(cd $(dirname $0);pwd)
pythonFile=$baseDir/../slack-notification/send.py
scp $pythonFile rasp4:/home/pi/person-recognition-opencv-rust/slack-notification/
