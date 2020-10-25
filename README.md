# Rust EasyAdmin

An admin panel made easy written in rust.
This project is still under development, if you want to contribute, feel free to send a pull request :)

## Development
1. There is not yet a public crate published, so for development just copy the lib and the cli from [rust-easyadmin-framework](https://github.com/edenreich/rust-easyadmin-framework) to the current directory.

2. Copy .env.dist file to .env: `cp .env.dist .env`

3. Populate the .env file with a random key `easyadmin key:generate`

## Deploy
Package the application as a container by running:
```
docker build -t easyadmin --target production .
```

When running `docker images | grep easyadmin` you should see your containerized admin application.

To run this application use the following:
```
docker run -d \
    -e DATABASE_URL=mysql://root:secret@mysql:3306/easyadmin \
    -e ROCKET_SECRET_KEY=$(openssl rand -base64 32) \
    easyadmin
```

Note: replace DATABASE_URL with your own configurations.