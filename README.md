# apiece

Container-oriented application development workflow.

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

## Base Images

Base images are implemented as git repositories. Currently there is no registry,
so when creating a new application the base needs to be specified as a git URL.

### Existing Bases

Please send a pull request to include your base on this list.

* https://github.com/pl/apiece-base-express-4

## TODO

* add package name verification hook
* print base notes after bootstrapping a project
* support different architectures
* write docs
* rethink docker image tags
* document base image format
* think about docker cleanup commands
* add export command
* implement base registry
