# Filesharing Tool

Current state is beta. It has most of the functionality in place. You should expect bugs and breaking changes (requiring rebuilding the DB) until this leaves beta.

## Features

- Accounts with permission system
  - Upload files
  - List files
  - Delete files
  - Manage accounts
- Upload files
  - Optional password lock
  - Optional expiery date
- Works without JavaScript enabled
- Files get uploaded in chunks (requires JavaScript)
  - Retries up to 5 times if errors occur
  - Current progress indicator

## Project progress

You can view the current progress towards 1.0 full release [here](https://github.com/users/Reinachan/projects/3/views/1)

## Screenshots


<table>
<tr>
	<td>Homepage
	<td>Upload files page
<tr>
	<td><img src="https://user-images.githubusercontent.com/16106839/227304219-c2cc31e8-224c-4646-890e-22fce481ced7.png" alt="download files page" />
	<td><img src="https://user-images.githubusercontent.com/16106839/227304323-5b19b8ca-9fc5-4e26-a4f0-633729e5ba7f.png" alt="upload files page" />
<tr>
	<td>Files list page
	<td>Profile page (WIP)
<tr>
	<td><img src="https://user-images.githubusercontent.com/16106839/227304421-7249438d-5709-4050-8038-dbe93df403c6.png" alt="files list page" />
	<td><img src="https://user-images.githubusercontent.com/16106839/227304464-d32f5608-4462-4149-b70a-946cc5053599.png" alt="profile page" />
</table>

## Build

In development

```sh
cargo run
```

Initialise database (this may change in the future)

```sh
cargo install sqlx
sqlx database create
sqlx migrate run
```

Compile typescript

```sh
npm i -g typescript # first time only
tsc --watch # compiles on save
```

Compile for production

```sh
tsc
cargo build --production # optimises performance
```

Run in production

```sh
./target/release/filesharing
```

Alternatively you can use Docker. To set that up, create a `compose.yaml` file in the parent directory of this project with this content:

```yaml
version: '3.9'

services:
  filehost:
    container_name: fileshare
    build: ./Fileshare
    restart: unless-stopped
    env_file:
      - .env
    volumes:
      - ./Fileshare/files:/usr/src/fileshare/files
      - ./Fileshare/db:/usr/src/fileshare/db
    user: '1000:1000'
    ports:
      - '3000:3000' # remove this if you're using Caddy/Nginx
```

Put the `.env` file next to the `.compose.yaml` file and run

```sh
# Prepare the binary
cd Fileshare
cargo build --release
tsc # assumes you've already installed typescript globally with npm
cd ..
# Build and run the docker container
docker compose build
docker compose up
```

## License

The code is licensed under the AGPLv3 license. The icons in the `assets` folder are all public domain icons, some of which I've modified. All icons should be treated as being public domain.
