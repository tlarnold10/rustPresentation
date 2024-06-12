# rustPresentation
Game of fill in the blank stories. Choose a story and fill in the blanks when prompted.

# Deployment
1. Build docker image
    - Navigate to `./client/storiesclient` and run `docker build -t rust-example .`

# Development
You will need to either have Docker or Rust installed. 
- Docker to build an image and application.
    1. Navigate to `./client/storiesclient` and run `docker build -t rust-example .`
    2. Run `docker run -i rust-example` to build a container using the newly created image.
    3. Within the container, navigate to the `storiesclient` directory.
    4. Run `cargo build`
    5. You will likely need to copy the `data` directory from the root directory to the `target/debug` directory.
    6. Navigate to the `target/debug` directory and run the application using `./storiesclient`

- Rust if you want to run and edit the application locally.
    1. Start the client cli by navigating to the `client/storiesclient/` directory and running `cargo run`.

# Architecture
- I was going to use a server and client model for this, but currently all logic just lives in the client.
    - At some point it would be great to create an http server and have the client make http requests to the server. Then the server would handle all the logic, store the templates, and even log all stories that were told. Future me's problem!
