# zixcel-japan-administrative-areas

Exact Japanese subdivision codes and Japanese/English display names. This
library does not infer residence, normalize user input or implement a HAT role.

```rust
use zixcel_japan_administrative_areas::find_exact;
assert_eq!(find_exact("JP-13").unwrap().name_ja, "東京都");
```

Source and dataset copied unchanged from hathq/hat-japan-administrative-areas.
HAT capability, account selection and unsupported-country errors remain in the
product adapter. Original Git ancestry, LICENSE and NOTICE are retained.
Registry publication and the product dependency switch follow verification.

Run `cargo test --locked` and `cargo package --locked`.

[License](LICENSE) · [Notices](NOTICE) · [Security reporting](SECURITY.md)

[OSS responsibility and source completion](docs/oss-source-contract.md) defines the supported package boundary and required acceptance. It does not declare unexecuted source, distribution or production checks complete.

Dependency updates: [repository policy](docs/dependency-updates.md).
Registry releases: [version tags and GitHub OIDC](docs/registry-release.md).
