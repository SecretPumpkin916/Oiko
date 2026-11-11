build +FLAGS="":
    cargo build {{FLAGS}}

run +FLAGS="":
    cargo run {{FLAGS}}
ipfsopen:
    ipfs init
    ipfs daemon
