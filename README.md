Cinderella CI
=============

Cinderella CI is a simple Continuous Integration solution for git repositories.
It is designed for people who do not want to work with big solutions like
Gitlab or Jenkins and probably also work with standard *bare* git repositories
(instead of Gitlab, gitea or similar).


Usage
-----

Cinderella expects a `.cinderella.toml` file with the CI configuration in the
root folder of your repository. When you execute Cinderella with the clone URL
of a git repository it will clone the URL into a temporary folder, search for
the CI configuration file and if available execute the build steps.

There is a sample hook in `hooks/post-update` which you can use in your remote
repository to execute Cinderella automatically each time you push to your
repository.

You can also manually execute Cinderella. To do so pass it the path to your
git repository and optionally the name of the branch you want to build:

```bash
cinderella https://github.com/aufziehvogel/Cinderella.git --branch development
```


Configuration Format
--------------------

The CI configuration file is a TOML file with one table per build pipeline.
Common build pipelines are `test` or `build`. E.g. the following is a valid
configuration file executing a `test` phase and a `build-release` phase.

```toml
[test]
commands = [
    "cargo test",
]

[build-release]
commands = [
    "cargo build --release",
]
```

Pipelines are executed in the order in which they are defined. For the
given configuration file it is ensured that first `test` is being executed
followed by `build-release`.

### Variables

You can use variables in your build commands. These variables will get
substituted with actual values from the build context. Currently supported
variables are:

- `{{ branch }}`: The name of the branch that is built

### Conditions

You can conditionally execute a pipeline with the `when` option, for example
to run a pipeline only for specific branches:

```toml
[build-release]
commands = [
    "cargo build --release",
]
when = "'{{ branch }}' == 'master'"
```

The condition will be executed with the Linux `test` command. Please be aware
that this behaviour will likely change in the future and another test
execution engine might be used.
