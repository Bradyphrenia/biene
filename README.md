# biene

`biene` represents an electronic hive card

## Structure

`biene` is a Rust/Typescript application which connects to an existing postgresql database and handels organisation of bee hives. The desktop application uses `tauri` and inside `tauri` `vue` is used.

## Development

`biene` uses `diesel` for database handling (supports multiple database types). Thus, `diesel_cli` must be available. A detailed description can be found in `diesel` [Getting Started](https://diesel.rs/guides/getting-started).

In addition, the `DATABASE_URL` environmental variable has to be set either using `export DATABASE_URL=postgres://username:password@localhost:port/database_name` or create a `.env` file in the project root folder with the variable:

```sh
echo DATABASE_URL=postgres://username:password@localhost:port/database_name > .env
```

Be aware that diesel might apply changes on your database so you should consider using a sample database and load a dump of your data within this isolated database.

To build the application a current node.js and Rust version is required.

Once both is installed and prepared. You should use the following steps to run the app in development mode:

1. `npm install` to install all required node modules.
2. `npm run tauri dev` to run the application and to install potential missing rust crates.

That's it :)

### Using Docker

A `Dockerfile` is included to run the application from within a Docker container. Therefore the following two steps are required:

1. Build the container with `docker build -t tauri .` from root directory
2. Run the container with `docker run -it --rm --env="DISPLAY=host.docker.internal:0" tauri`. On MacOS xQuartz is required and the environmental variable should be available. To enable XQuartz,
    1. `ip=$(ifconfig en0 | grep inet | awk '$1=="inet" {print $2}')`
    2. `export DISPLAY="$ip:0.0"` (export local monitor by setting environmental `DISPLAY` variable)
    3. `xhost +` (grant access for all)

The `docker-compose.yml` also creates some testing database.

When running `docker-compose up`, be aware to have XQuartz running (`xhost +`)
