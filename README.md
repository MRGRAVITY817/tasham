# TASHAM stack

TASHAM is my preferred stack for developing web apps with Rust.

I'm still doing experiments with this template, so please use it at your own risk.

## What does "TASHAM" stand for?

- [**T**ailwindCSS](https://tailwindcss.com/), for easy styling
- [**A**lpine](https://alpinejs.dev/), to get a bit of power to manipulate clientside state
- [**S**urrealDB](https://surrealdb.com/), as a multi-purpose database
- [**H**tmx](https://htmx.org/), for building a hypermedia-driven app
- [**A**xum](https://docs.rs/axum/latest/axum/), as a backend library
- [**M**aud](https://maud.lambda.xyz/), for easy templating within Rust code

Well, for now, I will use the name _TASHAM_, but I cannot promise that I'll still stick to this name once
I switch some of the tools inside the stack.

## Prerequisites

### Docker & Docker Compose

This template uses Docker primarily for managing both local and remote infrastructure.

The instructions to install _Docker_ for each platform can be found [here](https://docs.docker.com/engine/install/).
For installing _Docker Compose_, [here](https://docs.docker.com/compose/install/linux/#install-the-plugin-manually).

### Fly.io account & `flyctl`

I've found that Fly.io is currently the easiest platform to deploy Dockerized apps.  
If you don't have a Fly.io account, head to their [website](https://fly.io/) and create one.

You'll also need to install `flyctl`, which is a CLI tool for managing Fly.io apps.  
The instruction to install `flyctl` for each platform can be found [here](https://fly.io/docs/hands-on/install-flyctl/).

Once you installed it, run this command to log in to your Fly.io account.

### `cargo-watch`

For live-reloading your source code, install `cargo-watch`.  

```bash
$ cargo install cargo-watch
```


```bash
$ fly auth login
```

## Local Development

First you should run SurrealDB. Run this script.

```bash
$ ./scripts/local_db.sh up

# The command above will use "surreal" as default DB user & password.
# To provide custom user & password, run
# $ SURREAL_USER=<custom user> SURREAL_PASS=<custom password> ./scripts/local_db.sh up
```

`local_db.sh` accepts `up` or `down` option.  
When you want to stop database, replace the command above with `down` options.

## Deployment

TODO: Add description
