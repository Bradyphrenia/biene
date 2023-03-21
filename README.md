# biene

`biene` represents an electronic hive card

## Structure

`biene` is a Rust/Typescript application which connects to an existing postgresql database and handels organisation of bee hives. The desktop application uses `tauri` and inside `tauri` `vue` is used.

## Development

To build the application a current node.js and Rust version is required.

Once both is installed and prepared. You should use the following steps to run the app in development mode:

1. `npm install` to install all required node modules.
2. `npm run tauri dev` to run the application and to install potential missing rust crates.

That's it :)
