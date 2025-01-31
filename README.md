This is the `algebraic-client` module from our blog post: [A full-fledged Rust architecture based on workspaces and gRPC](https://blog.nullnet.ai/blog/workspaces-grpc).

This repository contains a simple client that receives inputs from a file monitored for changes, parses such inputs,
and invokes the `algebraic-server` to compute operations such as factorials and powers.


<div align="center">
  <picture>
    <img src="./diagram.png" alt="Architecture diagram" width="80%"/>
  </picture>
</div>