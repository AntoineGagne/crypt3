# crypt3

A NIF that interfaces with [`crypt(3)`](https://www.man7.org/linux/man-pages/man3/crypt.3.html).

## Dependencies

* [OTP 21+](https://www.erlang.org/)
* [`rebar3`](https://www.rebar3.org/)
* [Rust](https://www.rust-lang.org/)

## Development

### Building

To compile this library, the following command can be used:

```sh
rebar3 compile
```

### Running Checks

To run all the checks, it is possible to use the `check` alias:

```sh
rebar3 check
```

This will run all the tests (`eunit`, Common Tests and `proper` tests) along
with `elvis`, `xref`, `dialyzer`, `edoc` and code coverage.
