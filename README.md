<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="website/public/assets/logo-dark.png">
    <img src="website/public/assets/logo-light.png" alt="Fensu" width="280">
  </picture><br>
  <sub>by <a href="https://sqlstack.io">SQLStack</a></sub>
</p>

Fensu checks repository architecture: **import boundaries, module roles, and function
shape** for Python, Rust, TypeScript, JavaScript, and Svelte.

- **Boundaries:** stop one domain from importing another domain’s private helpers.
- **Roles:** keep entry points, helpers, models, and types in their owning modules.
- **Function shape:** keep orchestrators small, dataflow explicit, and signatures clear.

## Get started

Requires **Python 3.12+**. Install in your project’s Python environment:

```bash
pip install fensu
```

From an existing Python repository’s `pyproject.toml` directory:

```bash
fensu init --yes
fensu check
```

`fensu init --yes` detects your project, creates `fensu.toml`, runs an initial check,
and installs agent guidance. Existing configuration is preserved.

Run `fensu check` locally and in CI to enforce your policy. For established
repositories, [adopt rules incrementally using explicit boundaries and exceptions](https://fensu.io/docs/adoption/).

See the [quickstart](https://fensu.io/docs/quickstart/) for empty repositories,
existing configuration, and language-specific setup.

## A finding and its fix

An order entry point importing inventory’s private helper crosses a domain boundary:

```python
# src/shop/orders/main/submit.py
from shop.inventory._helpers.reserve import reserve
```

Excerpt from a real `fensu check --no-color` run with `FFL102` selected:

```text
FFL102  import 'shop.inventory._helpers.reserve' reaches into internal structure of 'shop.inventory'
 --> src/shop/orders/main/submit.py:1:0
  = help: Import from classes, models, types, constants, exceptions, or a thin main/ entry.
```

Publish the operation through inventory’s thin `main/` entry, then import that
public surface instead:

```python
from shop.inventory.main.reserve import reserve
```

That change clears this boundary finding; it does not disable the rule. Inspect a
diagnostic’s full contract and settings with `fensu rule FFL102`.

## The default layout

A Python domain owns its behavior and vocabulary:

```text
src/shop/
└── inventory/
    ├── main/           # public entry points and orchestration
    │   └── reserve.py
    ├── _helpers/       # private implementation phases
    ├── classes/        # one class per module
    ├── models.py       # data models
    ├── types.py        # type declarations
    ├── constants.py
    └── exceptions.py
```

A leaf needs meaningful `main/` behavior, not an empty placeholder. Domains can
split into named subdomains; tests mirror the code they cover. Tooling has its own
scope. See the [architecture model](https://fensu.io/docs/concepts/architecture-model/)
for complete role contracts, layouts, and defaults.

## Explore the repository

| Command | Purpose |
| --- | --- |
| `fensu rule FFL102` | Explain a rule and its effective settings. |
| `fensu map submit --root src --depth 2` | Show proven static Python calls, without running the program. |
| `fensu dupes --since origin/main --diff` | Review changed duplicated-code clusters; advisory unless `--fail-on-new` is passed. |
| `fensu skills --check` | Verify installed guidance matches the current policy. |

Call mapping needs neither Fensu configuration nor rule adoption. It marks
unresolved calls, cycles, and depth limits instead of guessing dynamic targets.
See the [map reference](https://fensu.io/docs/cli/map/) and
[duplicated-code guide](https://fensu.io/docs/concepts/duplicated-code/).

## Documentation

[fensu.io/docs](https://fensu.io/docs/) is the full manual; the README is the starting point.

| Start here | Reference |
| --- | --- |
| [Quickstart](https://fensu.io/docs/quickstart/) | Installation, initialization, and reading diagnostics. |
| [Adoption](https://fensu.io/docs/adoption/) | Roll out enforcement safely on existing code. |
| [Configuration](https://fensu.io/docs/concepts/configuration/) | Targets, ownership roots, selection, thresholds, exceptions, and ignores. |
| [Architecture](https://fensu.io/docs/concepts/architecture-model/) | Roles, domain boundaries, tooling, and test layout. |
| [Rust](https://fensu.io/docs/concepts/analyzers/rust/) · [Web](https://fensu.io/docs/concepts/analyzers/typescript-sveltekit/) | Analyzer-specific setup and rule packs. |
| [CLI](https://fensu.io/docs/cli/) · [Agent guidance](https://fensu.io/docs/cli/skills/) | Commands, execution, and skill update safety. |
| [Caching](https://fensu.io/docs/concepts/caching/) | Cache identities, controls, and safe cleanup. |
| [Custom rules](https://fensu.io/docs/concepts/custom-rules/) · [RuleContext API](https://fensu.io/docs/reference/rule-context/) | Author, configure, and test project-specific policies. |

[Python dead-code enforcement](docs/dead-code.md) has a separate repository guide
covering reachability, explicit dynamic roots, and adoption caveats.

## Part of SQLStack

<picture><source media="(prefers-color-scheme: dark)" srcset="website/public/assets/sqlstack-dark.png"><img src="website/public/assets/sqlstack.png" alt="" width="24"></picture> Fensu is part of [SQLStack](https://sqlstack.io), a family of tools for data engineers.

- [SQLBuild](https://sqlbuild.com) — SQL transformations with compile-time checks, end-to-end tests, and safe renames.
- [SQLClean](https://sqlclean.io) — Fast SQL linting and deterministic formatting.
- [SQLTide](https://docs.streambuild.dev) — Build, replay, and safely deploy ClickHouse streaming pipelines with SQL.
