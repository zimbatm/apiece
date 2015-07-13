# apiece

Bootstrap, develop, build and run applications in Docker or locally.

Uses base images for bootstrapping applications, which include scripts for
building and running the created application in consistent, isolated
[environments](#environments).

## Requirements

* Docker 1.6.0+
* Git

## Installation

TODO: There are binaries in GitHub releases, but they are not built
automatically and are only provided for Linux.

## Basic usage

Start by creating an application using `apiece new` specifying the base image
and the name of your app.

```bash
apiece new https://github.com/pl/apiece-base-express-4 apiece-hello-world
cd apiece-hello-world
```

Build the application using Docker:

```bash
apiece build
```

Start the server and expose the application port:
```bash
apiece run -p 1234
```

Check that the application is running:
```bash
curl http://localhost:1234
```

## Environments

There are 3 environments supported by `apiece`: `production` (default),
`development` and `local`. Each of the environments supports separate
`build`/`run`/`exec`/`clean` scripts and mounted data directories.

Following table illustrates the differences between the environments:

                                   | production | development | local
-----------------------------------|------------|-------------|------
runs in Docker                     | Y          | Y           |
requires local application runtime |            |             | Y
supports persistent data           | Y          | Y           | Y
supports live application updates  |            | Y           | Y
always starts in clean environment | Y          |             |

### Docker

Containerized environments run applications in Docker containers, which provide
isolation between the host system (e.g. server or development machine).

### Local Application Runtime

When using Docker, the host system does not need to provide application runtime
(e.g. Ruby for Rails applications) - everything is included in the Docker image.

### Persistent Data

Each environment exposes a dedicated data folder, which is persisted between
application restarts on given host.

### Live Application Updates

Some base images support live application updates. For example, Rails can
refresh the code after modifications without restarting the application itself.
Such updates are supported by `development` and `local` environments to make
programmer's life easier.

### Clean Environment

Production applications and tests should be run in a clean environment to make
execution repeatable. Although using clean environments is advised, it is not
possible to support it along live updates.

## Application Layout

When bootstrapping an application, apiece creates an `apiece.io` directory
in the root of the project. This folder contains:

* build hooks (production, development, local)
* exec hooks (production, development, local)
* run hooks (production, development, local)
* clean hooks (development, local)
* Docker files (production, development)
* data directories (production, development, local)
* app-name file containing the application name

Command hooks and Docker files can be modified whenever needed, just make sure
they respect semantics of their environments.

## Commands

All commands and available options can be found by running `apiece -h`.

### new

```
new <base> <name>
```

Bootstraps a new application from given base image in a new directory.

Application name format may differ between base images. More information can be
found in the specific base image repository README file.

### build

```
build local [-d DIR]
build [dev] [-d DIR --forward-ssh-agent]
```

Builds the application in given environment.

### run

```
run local [-d DIR -p <port>]
run [dev] [-d DIR -p <port> --forward-ssh-agent --dockeropt=OPT...]
```

Runs the application in given environment using the last build. Supports
exposing the application port and SSH agent sock forwarding for containerized
environments.

### exec

```
exec local [-d DIR] [--] <command>...
exec [dev] [-d DIR --forward-ssh-agent --dockeropt=OPT...] [--] <command>...
```

Executes am arbitrary command in the environment context.

### clean

```
clean local [-d DIR]
clean dev [-d DIR --dockeropt=OPT...]
```

Removes artifacts created by the application inside the working directory.
Useful for cleaning up logs and temporary files that can't be places outside the
working directory and need to be removed before switching between `local` and
`development` environments.

For example, Rails creates a local `tmp` directory with compiled assets, which
when created in the `dev` will be owned by the user running the docker daemon
(usually root).

### info name

Returns the name of the application.

## Base Images

Base images are implemented as git repositories. Currently there is no registry,
so when creating a new application the base needs to be specified as a git URL.

### Existing Bases

Please send a pull request to include your base on this list.

* https://github.com/pl/apiece-base-express-4
* https://github.com/pl/apiece-base-rails-4

## Build Instructions

In order to build `apiece`, Rust 1.1.0+ and Cargo must be installed. Running

```
cargo build --release
```

will build the binary and place it in `target/release/apiece`.
