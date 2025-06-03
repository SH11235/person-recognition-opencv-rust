from dotenv import load_dotenv
import os
from pathlib import Path
import logging
from slack_sdk import WebClient


# setting environment variables
load_dotenv()
SLACK_BOT_TOKEN = os.getenv("SLACK_BOT_TOKEN")
SLACK_CHANNEL = os.getenv("SLACK_CHANNEL")
SLACK_CHANNEL_ID = os.getenv("SLACK_CHANNEL_ID")
SLACK_MESSAGE = os.getenv("SLACK_MESSAGE")


logging.basicConfig(level=logging.DEBUG)

image_dir = "../person-recognition/image"
image_file_paths = list(Path(image_dir).glob(r"*.jpg"))
# 最新順にソート
image_file_paths.sort(key=os.path.getmtime, reverse=True)
logging.info(image_file_paths)

if len(image_file_paths) > 0:
    file_path = image_file_paths[0].__str__()

    client = WebClient(token=SLACK_BOT_TOKEN)
    result = client.files_upload_v2(
        channel=SLACK_CHANNEL_ID,
        initial_comment=SLACK_MESSAGE,
        file=file_path,
    )

    # clean up
    import shutil

    for image_file_path in image_file_paths:
        shutil.move(image_file_path.__str__(), image_dir + "/old/")
