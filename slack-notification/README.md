# slack-notification

## Environment

Python3

## package install

```sh
pip install slack-bolt
pip install python-dotenv
```

## .env

```sh
cp .env.sample .env
```

- SLACK_BOT_TOKEN

  Get your token from <https://api.slack.com/apps/>

- SLACK_CHANNEL

  `#your_channel_name`

- SLACK_CHANNEL_ID

  Get from the channel url: `https://xxxxxx.slack.com/archives/SLACK_CHANNEL_ID/pxxxxxxxxx`

- SLACK_MESSAGE

  Any message.
