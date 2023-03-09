# biene
elektronische Stockkarte

## Development

### Docker

The `docker-compose.yml` file contains everything to set up for development.

`docker-compose up` run from project root directory will create a postgresql database. The credentials are provided as environmental variable in the `docker-compose.yml`. Once the docker container is created it can be started usinf `docker-compose start beee_db`. The database will be stored in `./data/beee_db/` folder. If you want to start with a fresh container, just delete the content of this folder.
