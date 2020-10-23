# Rust EasyAdmin

An admin panel made easy written in rust.
This project is still under development, if you want to contribute, feel free to send a pull request :)

## Development
There is not yet a public crate published, so for development just copy the lib from [rust-easyadmin-framework](https://github.com/edenreich/rust-easyadmin-framework) to the current directory.

## Deploy
Package the application as a container by running:
```
docker build -t easyadmin .
```

When running `docker images | grep easyadmin` you should see your containerized admin application with a size of approx 13.4MB, deploy it anywhere you want :)