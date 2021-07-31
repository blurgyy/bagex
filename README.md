# bagex

## Synopsis

`bagex` serves as an entrance for programs, it manages environment variables
for the programs according to a configuration file.

## Configuration

The configuration file is in [toml](https://github.com/toml-lang/toml) format,
and consists of 3 parts: `path`, `env`, and `exe`.

- `path`: It is an array of paths, which will be **prepended** to the
  environment variable `$PATH` when `bagex` tries to find a executable.
- `env`: This section creates multiple one-to-many mappings _from an
  environment variable to some executables_.
- `exe`: This section creates multiple one-to-many mappings _from an
  executable to some environment variables_.

An illustrative example configuration can be found [here](./config.toml).

## Usage

`bagex` takes an executable name as its argument, and tries to read its
configuration from `$XDG_CONFIG_HOME/bagex/config.toml` if the environment
variable `XDG_CONFIG_HOME` is set, otherwise it tries to read
`$HOME/.config/bagex/config.toml`.  Optionally use `-c|--config-file` to
specify the configuration file to read.  Example usage:

```shell
$ cat config.toml
[env.answer_to_the_ultimate_question_of_life_the_universe_and_everything]
42 = [
    "printenv",
]
[exe.printenv]
some_random_string = "YmFnZXgK"
pi = 3.14

$ bagex -c config.toml printenv
[..truncated..]
answer_to_the_ultimate_question_of_life_the_universe_and_everything=42
pi=3.14
some_random_string=YmFnZXgK
```

To add arguments for the requested executable (`printenv` in the above
example), append the arguments after a double dash (`--`):

```shell
$ bagex -c ./config.toml bagex -- --version
bagex 0.1.0
```

Use `-d|--dry-run` to show the command to run and abort:

```shell
$ bagex -c ./config -d echo -- -en "Hello  world!"
/sbin/echo "-en" "Hello  world!"
```

## Use cases

### Systemd services

It _is_ quite convenient to set a process' environment in a systemd service,
but every time the environment has to change, one must run `systemctl [--user]
daemon-reload` after changing the `.service` file, then call `systemctl
[--user] restart foo.service` to update the running environment of the
program.

`bagex` makes this process even more convenient by specifying the environment
of a program in a separate config file, so that each time the environment is
changed in `bagex`'s config, only a `systemctl [--user] restart foo.service`
has to be called.

### Application launchers (like rofi, sxhkd, etc.)

## License

[MIT](./LICENSE)
