"""Files permitted to change on a generated release branch."""

SEMVER_PART_COUNT: int = 3
NORMALIZED_VERSION: str = "0.0.0"
CHANGELOG_PATH: str = "CHANGELOG.md"
RELEASE_MANIFEST_PATH: str = ".release-please-manifest.json"
ROOT_PROJECT_PATH: str = "pyproject.toml"
CLI_PROJECT_PATH: str = "packages/fensu-cli/pyproject.toml"
CARGO_LOCK_PATH: str = "Cargo.lock"
ROOT_CARGO_PATH: str = "Cargo.toml"
UV_LOCK_PATH: str = "uv.lock"
VERSION_PATHS: tuple[str, ...] = (
    RELEASE_MANIFEST_PATH,
    ROOT_PROJECT_PATH,
    CLI_PROJECT_PATH,
    "crates/fensu-cli/Cargo.toml",
    "crates/fensu-policy/Cargo.toml",
    "crates/fensu-facts/Cargo.toml",
    "crates/fensu-native/Cargo.toml",
    ROOT_CARGO_PATH,
    UV_LOCK_PATH,
    CARGO_LOCK_PATH,
)
OWNED_CARGO_PACKAGES: frozenset[str] = frozenset(
    {"fensu-cli", "fensu-facts", "fensu-native", "fensu-policy"}
)
OWNED_WORKSPACE_DEPENDENCIES: frozenset[str] = frozenset({"fensu-facts"})
OWNED_UV_PACKAGES: frozenset[str] = frozenset({"fensu", "fensu-cli"})
ALLOWED_PATHS: frozenset[str] = frozenset((*VERSION_PATHS, CHANGELOG_PATH))
