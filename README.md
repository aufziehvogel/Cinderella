# Cinderella CI

Being one of my many project ideas this will probably never take off, but the
idea is to create a lightweight CI solution that can be used without docker.

Problems with existing solutions:

- Gitlab CI: Gitlab requires a lot of memory and comes with its full git
  management
- Jenkins: Not very lightweight and in my opinion does not "feel" nice in usage
  (i.e. no clear approach because very plugin based, security configuration can
  be in multiple locations in the UI)
- Drone: Do not want to support builds [without Docker][drone-docker]
- TeamCity: Seems to require Tomcat (i.e. includes Java UI stuff), installation
  instructions (for agents) look complex on first sight

Solutions that might be interesting, but there might also be problems:

- GoCD:
  - states that it requires 1-2GB of memory only for the Server (without
    the Agents)
  - Pipeline as Code is not first-class citizen, plus it seems you always have
    to do adjustments in the web GUI
- Hydra: Can only be installed on Nix OS?
- Concourse: Uses containers for all builds?
- builds.sr.ht: I think no documentation on how to setup self-hosted


## Goal

So what is this project's goal?

- Perform CI and CD according to build steps from a config file
- Status Reporting:
  - If possible: no web GUI, alternatively: static HTML files with current
    status
  - Build failures and "OK again" status via mail
- Encrypted variables directly in files in the repository (similar to ansible
  vault files) -> key for decryption must still be stored on the
  server (thus security is limited if repo and key are on the same server, but
  same goes for Gitlab etc.)
- If possible: Should play nicely with ansible, because I deploy most projects
  with ansible (had some trouble with Gitlab there, I think because of the
  docker setup)


## Use Cases

Project Development (if it takes place at all) will be according to use cases:

1. Deploy a Zola-based website (static site generator) to the same server as
   Cinderella is installed on
2. Deploy a Zola-based website to an FTP server (requires encrypted password)
3. Run test execution for a Python project
4. Deploy a project with an ansible-playbook


## References

- [List of CI/CD tools][cicd]
- [reddit discussion on lightweight self-hosted CD][reddit-cd]

[drone-docker]: https://github.com/drone/drone/issues/2378
[cicd]: https://github.com/ligurio/awesome-ci
[reddit-cd]: https://www.reddit.com/r/devops/comments/a4tyju/lightweight_self_hosted_cd/
