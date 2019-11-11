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
cinderella https://github.com/aufziehvogel/Cinderella.git --branch master
```

You can use a different path than `.cinderella.toml` for your CI configuration
file with the argument `-f` or `--file`. This argument is evaluated relatively
to the git work directory. If you want to use a CI configuration file local
to your shell directory use absolute paths.

```bash
cinderella https://github.com/aufziehvogel/Cinderella.git --file /home/user/cinderella-test.toml
```


CI Configuration Format
-----------------------

The *CI configuration file* is a TOML file with one table per build pipeline.
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

All `commands` are executed as programs, i.e. no shell is involved. If you
want to execute one or multiple commands in a shell you have to call the
desired shell manually.

Pipelines are executed in the order in which they are defined. For the
given configuration file it is ensured that first `test` is being executed
followed by `build-release`.

### Variables

You can use variables in the configuration file. All variables are denoted
with a percentage symbol (`%`) and will be replaced before the commands
are being sent to the shell.
Currently supported variables are:

- `%BRANCH`: The name of the branch that is built

### Conditions

You can conditionally execute a pipeline with the `when` option, for example
to run a pipeline only for specific branches:

```toml
[build-release]
commands = [
    "cargo build --release",
]
when = "\"%BRANCH\" == \"master\""
```

The condition will be executed with the Rust library
[evalexpr](https://docs.rs/evalexpr/5.0.5/evalexpr/index.html).


E-Mail Notification
-------------------

You can send e-mail notifications on build failures. For this, create a file
called `config.toml` in the same directory as your Cinderella executable with
the following content (this file is called *Cinderella configuration file*
to distinguish it from the CI configuration file):

```toml
[email]
to = "recipient@example.com"
from = "noreply@example.com"
server = "example.com"
user = "example"
password = "password"
```

If Cinderella finds a `config.toml` file with a table `email` it will enable
e-mail notifications. If you want to disable e-mail notifications again,
delete the table `email` from your Cinderella configuration file or delete
the whole Cinderella configuration file.


Encrypted Environment Variables
-------------------------------

Sometimes a script needs to use credentials that you do not want to store in
a version control system in cleartext. For this use case, Cinderella supports
the storage of environment variables in an encrypted file. This file has to be
stored in `.cinderella/secrets`.

In cleartext create a TOML file `secrets.toml` that looks as follows:

```toml
USERNAME = "my-user"
PASSWORD = "my-secret"
```

You have to encrypt this file with GnuPG:

```bash
gpg -o .cinderella/secrets -c secrets.toml
rm secrets.toml
```

The password you chose during encryption has to be set in the *Cinderella
configuration file* (this means that you have to use the same password for
all projects you test and build with Cinderella):

```toml
[secrets]
password = "my-secret-for-decryption"
```

Of course, this means that an attacker on your server can decrypt all your
secrets. Secret encryption only ensures that credentials are not stored in your
repository in cleartext, but as soon as your server is compromised all your
credentials are compromised.


Open Points
-----------

This is a list of open points that are subject to implementation:

- introduce command shortcuts for commands that are often used but annoying
  to write in their full form
  - `"[bash] foo && bar"` for `"bash -c \"foo && bar\""`
  - `"[python-venv env] pip install -r requirements.txt && ./foo"` for
    `"bash -c \"source env/bin/activate && pip install -r requirements.txt && ./foo\""`
- substitute important environment variables like `$HOME` so that we do not
  have to run a bash in order to use them (or all environment variables?)
- keep a status of the last result per repository (to send *OK again* mails)
- send a more detailed error message on the build error reason:
  - return code of the failed command
  - full log of the executed commands (in a prompt style, command followed by
    output)
