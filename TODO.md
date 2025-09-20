- Cargo.toml
  - [x] add supporting crates
  - [x] add app crates

- utils/
  - [x] add boilerplate to tracing.rs
  - [ ] add app specific code to tracing.rs
  - [x] add constants.rs
  - [ ] add load_balancer.rs

- [ ] create config.rs ??? use config loader crate? use constants.rs?

- domain entities
  - error.rs
    - [x] LoadBalancerError
  - load_balancer.rs
    - [x] LoadBalancer struct.

- [ ] create lib.rs

- tests/
  - [ ] helpers.rs
  - [ ] main.rs
  - [ ] root.rs

- main.rs
  - [ ] wire up tracing.

- containerize
  - Containerfile
    - [ ] load balancer
  - compose.yaml
    - [ ] workers
    - [ ] load balancer
