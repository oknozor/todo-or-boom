# Todo or boom
ToB is todos and fixme linter. You most likely want to automate its usage using it either in a git commit hook 
or in your CI/CD pipeline. 


## Warning
⚠️ ToB is still a work in progress, currently only C-style comments are implemented, ToB will work with language like java, rust, c etc ⚠️

## Usage

### Command line

```shell
todo-or-boom -g "**/*.rs"
todo-or-boom -g "**/*.java"
```
### Pre-commit git-hook

```shell
#!/bin/sh
set -e

todo-or-boom -g "**/*.rs"
```