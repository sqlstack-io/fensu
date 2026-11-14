# fensu-facts

Python source facts for [Fensu](https://github.com/sqlstack-io/fensu): parse a module once into a `ProgramHandle`, then read its fact rows.

```toml
fensu-facts = { version = "0.30", default-features = false }
```

`default-features = false` leaves out the Python bindings.

The public API is `ProgramHandle` (built with `ProgramHandle::parse_source(source, major, minor)`), `ParseFailure`, and the row types in `facts`. Modules hidden from the documentation are shared with the rest of Fensu's workspace and may change in any release.
