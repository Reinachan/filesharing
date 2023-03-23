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

## Todo

- Account management from client
  - add accounts (admin)
  - remove accounts (admin)
  - change permissions (admin)
  - change password (user)
- Separate between unfinished chunked uploads and finished uploads
  - add job that checks for unfinished uploads and removes remnants of old unfinished uploads

## Screenshots

<table>
<tr>
	<td><img src="https://user-images.githubusercontent.com/16106839/225164029-6762cb87-afd3-4fd8-90f3-bc8c79f564b3.png" />
	<td><img src="https://user-images.githubusercontent.com/16106839/225164012-62eb5ac2-6d67-4d71-ba10-7cb8e10c7b7a.png" />
	<td><img src="https://user-images.githubusercontent.com/16106839/225163994-14a4acbe-8e2e-4481-bd40-c673da914d28.png" />
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
