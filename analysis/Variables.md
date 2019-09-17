Variables
=========

This document collects how other CI/CD solutions use variables in their
configuration file. What's the format, where they can be used and so on.

Jenkins
-------

- works with environment variables
- conditional code execution only possible with plugin?
  - can use environment variables (in examples always `$` is used) or
    special entities like `Time` and `Day of week` (via dropdown?)

TeamCity
--------

- system properties and environment variables
- system properties have `%sys.property%` syntax

Gitlab
------

- variables are written as shell variables, i.e. `$VARIABLE` in Linux and
  `%VARIABLE%` on Windows
- standard `only` and `except` filters do not use variables, but it's
  possible in `only.variables` and `except.variables` with the Linux shell
  syntax, e.g. `$RELEASE == "staging"`

Drone CI
--------

- environment variables for commands
- `$$VAR` syntax for variables that should be substituted in the YAML
  configuration before execution (e.g. settings that are no shell commands)
- `$$VAR` syntax supports some simple string substitutions known from bash

Circle CI
---------

- environment variables that can be defined at many different locations
  - clear precedence order (e.g. shell > YAML > project settings)
- variables in filters not possible?
