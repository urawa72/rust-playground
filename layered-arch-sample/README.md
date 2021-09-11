# layered-arch-sample

ref. https://github.com/foresta/rust-api-architecture-sample

## diesel with postgres

```zsh
# start postgres with docker
docker-compose up -d

# install diesel
sudo apt install -y libpq-dev
cargo install diesel_cli --no-default-features --features postgres

# setup, migration
diesel setup
diesel migration generate create_documents
diesel migration run
```
