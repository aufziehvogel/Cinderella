Cinderella CI
=============

Build Status: ![Build Status according to Cinderella CI Build](http://cinderella.stefan-koch.name/cinderella.git/master.png)

Cinderella CI is a simple Continuous Integration solution for git repositories.
It is designed for people who do not want to work with big solutions like
Gitlab or Jenkins and probably also work with standard *bare* git repositories
(instead of Gitlab, gitea or similar).

Cinderella is a single binary that currently executes all builds directly
on the same machine. Positive: It ships as a single binary with dependencies
only to standard libraries like libc. It does not require Docker or similar.
Negative: Testing with a clean, bare OS is currently not in focus. It's
probably possible by starting up a fresh VM, copying cinderella onto it and
then executing it, but it's more effort on your side.


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
cinderella run https://github.com/aufziehvogel/Cinderella.git --branch master
```

You can also build a specific tag with:

```bash
cinderella run https://github.com/aufziehvogel/Cinderella.git --tag 0.1.0
```

You can use a different path than `.cinderella.toml` for your CI configuration
file with the argument `-f` or `--file`. This argument is evaluated relatively
to the git work directory. If you want to use a CI configuration file local
to your shell directory use absolute paths.

```bash
cinderella run https://github.com/aufziehvogel/Cinderella.git --file /home/user/cinderella-test.toml
```


Configuration Files
-------------------

Cinderella uses two configuration files:

- **CI Configuration File**: The CI configuration file is usually called
  `.cinderella.toml` and belongs to the project
  that should be built. It defines under which circumstances Cinderella should
  build the project (e.g. only tags, only specific branches) and which
  commands the Cinderella engine has to execute to run the build. Without
  this file, Cinderella will not run a build for the project.
- **Cinderella Configuration File**: The Cinderella configuration file belongs
  to your Cinderella instance. It is usually called `config.toml` and is
  located in the same folder as your Cinderella executable. It specifies
  parameters that Cinderella needs to perform actions like sending e-mails,
  writing output files or decrypting secrets. This file is optional.


CI Configuration Format
-----------------------

The *CI configuration file* has to be saved to the root directory of your
repository as `.cinderella.toml`.
It is a TOML file with one table per build pipeline.
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
followed by `build-release`. If an error occurs in any of the pipelines,
execution will be aborted and the following pipelines will not be executed.

### Variables

You can use variables in the configuration file. All variables are denoted
with a percentage symbol (`%`) and will be replaced before the commands
are being sent to the shell.
Currently supported variables are:

- `%REFTYPE`: The type of reference that is built, `branch` or `tag`
- `%BRANCH`: The name of the branch that is built, if it is a branch
- `%TAG`: The name of the tag that is built, if it is a tag

### Environment Variables

It is possible to use environment variables in your commands, e.g.
`commands = ["echo $HOME"]`. Cinderella will substitute them by their
values before the command gets sent to the operating system.

This is also true if you use `bash` or other shells in your commands list.
This means that in such cases the plaintext value of the environment
variable will be visible in your shell history.

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


Encrypted Variables
-------------------

**Warning:** I am not security expert and there has been no analysis regarding
the security of my implementation. I am using `sodiumoxide` internally, but
still I could do something wrong. If you want to use this feature on a public
repository, please review my implementation. I personally only use it for
internal repositories at the moment. If you find any vulnerabilities in my
implementation please tell me.

Sometimes a script needs to use credentials that you do not want to store in
a version control system in plaintext. For this use case, Cinderella supports
the storage of variables in an encrypted file. This file has to be
stored in `.cinderella/secrets`.

In plaintext create a TOML file `.cinderella/secrets.toml` that looks as
follows:

```toml
USERNAME = "my-user"
PASSWORD = "my-secret"
```

Optionally, you can add the plaintext file to your `.gitignore`.
You can create an encrypted file `.cinderella/secrets` by running the
following command from your project's root directory:

```bash
cinderella encrypt
```

After this step you may delete the `secrets.toml` if you want.

You can now use the variables in your build commands:

```toml
[build-release]
commands = [
   ".cinderella/upload-to-ftp.sh %USERNAME %PASSWORD",
]
```

To decrypt the encrypted file (and re-create `secrets.toml`) run:

```bash
cinderella decrypt
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


Badges
------

It's possible to generate build success/failure badges that can be displayed
on websites. To enable this feature add the following configuration:

```toml
[dashboard]
folder = "/var/www/cinderella"
```

The build process will then write the badges into `folder`. Serving the data
via HTTP is your own responsibility, use any web server of your choice.


Open Points
-----------

This is a list of open points that are subject to implementation:

- use virtual machines to execute unit tests, e.g.
  [using qemu](https://stackoverflow.com/questions/3146013/qemu-guest-automation)
  (a more high level abstraction like libvirt or vagrant would probably be
  simpler, but I'd prefer low-level)
- add more system tests
- improve stability and error messages (sometimes I receive a rust crash
  due to a failed expect/unwrap)
- keep a status of the last result per repository (to send *OK again* mails)
- allow cinderella to `run` with a non-git folder: Logic should be so that
  user passes a URL or a path to `run` (same as now) and Cinderella checks
  if this is a URL, a local git repository or a local folder without git and
  then runs the correct `Source` and `WorkingCopy`
- introduce command shortcuts for commands that are often used but annoying
  to write in their full form
  - `"[bash] foo && bar"` for `"bash -c \"foo && bar\""`
  - `"[python-venv env] pip install -r requirements.txt && ./foo"` for
    `"bash -c \"source env/bin/activate && pip install -r requirements.txt && ./foo\""`
- send a mail with all compiler warnings? (or optionally to be
  enabled/disabled in .cinderella.toml?); otherwise developers never see the
  warnings
