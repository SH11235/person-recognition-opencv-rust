ps aux | grep [p]erson-recognition | awk '{ print "kill -9", $2 }' | sh
