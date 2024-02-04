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

```bash
$ fly auth login
```

## Local Development

TODO: Add description

## Deployment

TODO: Add description
