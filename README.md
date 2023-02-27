
# 📣 Subitobot 📣
[![Docker](https://github.com/Sasso0101/subitobot/actions/workflows/docker-publish.yml/badge.svg?branch=master)](https://github.com/Sasso0101/subitobot/actions/workflows/docker-publish.yml)

A Docker container that checks for new listings on [subito.it](https://www.subito.it) and sends a Telegram notification every time something new gets published.

Ready to use images are published on [Github packages](https://github.com/Sasso0101/subitobot/pkgs/container/subitobot).

### Deploy
Download the `docker-compose.yaml` file from the repository:
```
mkdir subitobot
wget https://raw.githubusercontent.com/Sasso0101/subitobot/master/docker-compose.yaml
```
Create a file named `.env` in the project's directory with the following entries (change the relevant parts and remove the quotes):
```
KEYWORDS="A comma separated list of keywords you want to track"
BOT_TOKEN="Your telegram bot token"
CHAT_ID="The telegram ID of the user you want to send the notifications to"
```
Run the following commands:
```
mkdir data
docker compose up
```
The Docker container will use the `./data` folder to store its files (if you want to change this behavior modify the `docker-compose.yaml` file).

Run the container! 🚀
```
docker start subitobot
```

The Docker container checks for new listings just once and then stops. If you want to check periodically for new listings you can use `cron` or similar tools to run the previous command on a schedule.

### Build from source
Clone the repository and run:
```
docker build -t subitobot .
```
