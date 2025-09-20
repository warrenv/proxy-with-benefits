This project will explore rust's memory layout and foreign function interface (ffi)
by impementing a reverse proxy server.

The focus will be on writing a load balancer in rust with a plugin system used for:
  - Balancing algorithms written in lua (and potentially rust).
  - A decision engine for dynamically switching the balancing algorithms.

The final result will be the code and throughput benchmarking results.
