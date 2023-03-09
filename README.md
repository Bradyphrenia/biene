# biene
elektronische Stockkarte

## Development

### Docker

The `docker-compose.yml` file contains everything to set up for development:

- a postgresql database
- python django backend

`docker-compose up` run from project root directory will create both: a postgresql database and a running django instance. The volume is mounted, thus changes in the code base will have an effect on the running instance (if hot reload is enabled).

The credentials and environmental variables are provided as environmental variable in the `docker-compose.yml`.

Once the docker container is created it can be started usinf `docker-compose start` to run all container at once or `docker-compose start <CONTAINER_NAME>` to run a specific container.

The database will be stored in `./data/beee_db/` folder. If you want to start with a fresh container, just delete the content of this folder. Otherwise the database from previous sessions will be used.
