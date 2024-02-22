<p align="center">
  <img
    width="250"
    src="./docs/logo.png"
    alt="Todo Or Boom - do it or die"
  />
</p>

<p align="center">
  <a href="https://github.com/oknozor/todo-or-boom/actions"
    ><img
      src="https://github.com/oknozor/todo-or-boom/workflows/CI/badge.svg"
      alt="GitHub Actions workflow status"
  /></a>
  <a href="https://codecov.io/gh/oknozor/todo-or-boom"
    ><img
      src="https://codecov.io/gh/oknozor/todo-or-boom/branch/main/graph/badge.svg"
      alt="Code coverage status"
  /></a>
  <br />
  <a href="https://conventionalcommits.org"
    ><img
      src="https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg"
      alt="Conventional commits"
  /></a>
  <a href="https://github.com/oknozor/todo-or-boom/blob/main/LICENSE"
    ><img
      src="https://img.shields.io/github/license/oknozor/todo-or-boom"
      alt="Repository license"
  /></a>
</p>


<h1></h1>

ToB is todos and fixme linter. You most likely want to automate its usage using it either in a git commit hook 
or in your CI/CD pipeline. 



| ![example screenshot ok](docs/todo_ok.png) | ![example screenshot err](docs/todo_err.png) |
|-------------------------------------|-------------------------------------|


## Warning
⚠️ ToB is still a work in progress, currently only C-style comments are implemented, ToB will work with language like java, rust, c etc. ⚠️

## Usage

see: `todo-or-boom --help`

### Command line

```shell
todo-or-boom -g "*.rs"
todo-or-boom -g "*.java"
```

### Pre-commit git-hook

```shell
#!/bin/sh
set -e

todo-or-boom -g "**/*.rs"
```



