# read .env
from dotenv import load_dotenv
load_dotenv()

# setting environment variables
import os
SLACK_BOT_TOKEN = os.getenv('SLACK_BOT_TOKEN')
SLACK_CHANNEL = os.getenv('SLACK_CHANNEL')
SLACK_CHANNEL_ID = "C03ELV7255Z"
SLACK_MESSAGE = os.getenv('SLACK_MESSAGE')

import logging
logging.basicConfig(level=logging.DEBUG)

image_path ="../person-recognition/image"
files = os.listdir(path=image_path)
files = [f for f in files if f.endswith(".jpg")].sort(key=os.path.getmtime)
logging.info("files: ", files)

file_name = files[0]

from slack_sdk import WebClient
client = WebClient(token=SLACK_BOT_TOKEN)
result = client.files_upload(
    channels=SLACK_CHANNEL_ID,
    initial_comment=SLACK_MESSAGE,
    file=file_name,
)
# Log the result
logging.info(result)

# clean up
import shutil
for file in files:
    shutil.move(image_path + "/" + file, image_path + "/old/")
