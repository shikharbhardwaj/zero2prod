# Zero to production in Rust

My attempt at following the book [Zero To Production In Rust](https://www.zero2prod.com).

# Dev setup

 - Install pre-commit hooks

    ```
    pip install pre-commit
    pre-commit install
    ```

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


## Local deployment

Instructions for standing up a new env using a local K8s cluster provided by Docker Dekstop.

Prerequisites: Docker dekstop, Helm

1. Enable Kubernetes in Docker desktop > Settings > Kubernetes

2. Install nginx ingress controller

   ```
   helm upgrade --install ingress-nginx ingress-nginx \
   --repo https://kubernetes.github.io/ingress-nginx \
   --namespace ingress-nginx --create-namespace
   ```

3. Create namespace

   ```
   kubectl create ns zero2prod
   ```

4. Helm install

   ```
   cd deployment
   helm install zero2prod --namespace zero2prod zero2prod -f local/values.yaml
   ```
5. (Optional) Apply the ingress or access the application via portforward.

## Credits

Uses the [Hyperspace](https://github.com/lufevida/html5up-hyperspace) theme from HTML5UP.
