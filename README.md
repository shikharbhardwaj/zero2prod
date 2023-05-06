# Zero to production in Rust

My attempt at following the book [Zero To Production In Rust](https://www.zero2prod.com).

# Dev setup

 - Dev cycle (lint, test and run)

    ```
    # Spin up a local DB
    ./scripts/init_db.sh

    # Start watch to hot
    cargo watch -x check -x test -x run | bunyan
    ``` 

## Deployment

 - CI configured via GitHub actions.
 - CD configured using Argo CD pointing to a K8s cluster.
