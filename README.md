# bagex

`bagex` serves as an entrance for programs, it manages environment variables
for the programs through a configuration file.

## Configuration

The configuration file is in [toml](https://github.com/toml-lang/toml) format,
consists of 3 parts: `path`, `env`, and `exe`.

- `path`: It is an array of paths, which will be **prepended** to the
  environment variable `PATH` when `bagex` tries to find a executable.
- `env`: This section creates multiple one-to-many mappings _from an
  environment variable to some executables_.
- `exe`: This section creates multiple one-to-many mappings _from an
  executable to some environment variables_.

An illustrative example configuration can be found [here](./config.toml).

## Usage

`bagex` takes an executable name as its argument, and tries to read its
configuration from `$XDG\_CONFIG\_HOME/bagex/config.toml` if the environment
variable `XDG\_CONFIG\_HOME` is set, or it tries to read
`$HOME/.config/bagex/config.toml`.  Optionally use `-c|--config-file` to
specify the configuration file to read.  Example usage:

```shell
$ bagex -c ./config.toml printenv
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
