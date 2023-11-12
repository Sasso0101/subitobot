
# 📣 Subitobot 📣
[![Docker](https://github.com/Sasso0101/subitobot/actions/workflows/docker-publish.yml/badge.svg?branch=master)](https://github.com/Sasso0101/subitobot/actions/workflows/docker-publish.yml)

Subitobot is a Telegram bot designed to notify a user about new listings posted on [subito.it](https://www.subito.it).

The bot is written in Rust and distributed as a Docker image published on [Github packages](https://github.com/Sasso0101/subitobot/pkgs/container/subitobot).

### Installation
Download the `docker-compose.yaml` file from the repository:
```
mkdir subitobot
wget https://raw.githubusercontent.com/Sasso0101/subitobot/master/docker-compose.yaml
```
Edit the `config.toml` file located in the `data` directory with your telegram user id (you can obtain it using [myidbot](https://t.me/myidbot)) and the bot token generated by [BotFather](https://t.me/BotFather). To configure the items to be tracked check the examples in the `config.toml` file. All blocks must start with `[[item]]`. All fields are optional except for `keyword`.
#### Description of the fields
- `keyword` (string): name of the item
- `region` (array of integers): restrict the search to specific regions. The regions must be encoded using [these codes](docs/regions.md). Arrays are specified as follows: `[x, y, z]`.
- `province` (integer): restrict the search to a province. The province must be encoded using [these codes](docs/provinces.md). If this field is used, then also `region` must be specified.
- `city` (integer): restrict the search to a city. The city must be encoded using its ISTAT code (a list can be found [here](https://dait.interno.gov.it/territorio-e-autonomie-locali/sut/elenco_codici_comuni.php)).
- `category` (integer): restrict the search to a category. The category must be encoded using [these codes](docs/categories.md).
- `search_only_title` (boolean): restrict the search to the title of the postings only. `false` by default.
- `min_price` (integer): restrict the search to items priced more or equal to this value.
- `max_price` (integer): restrict the search to items priced less or equal to this value.

Run the following commands:
```
mkdir data
docker compose up -d
```
Start the container! 🚀
```
docker start subitobot
```

The Docker container will check if new listings have been posted, send the notifications and quit. If you want to check periodically for new listings you can use `cron` or similar tools to run the container on a schedule.

### Build from source
Clone the repository and run:
```
docker build -t subitobot .
```
