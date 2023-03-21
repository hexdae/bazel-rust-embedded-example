load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository", "new_git_repository")
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# ###############################################################################
# ARM none ebai gcc
# ###############################################################################
git_repository(
    name = "arm_none_eabi",
    commit = "88d8e25b06be484188b04905f11f2788d302aa72",
    remote = "https://github.com/hexdae/bazel-arm-none-eabi",
)

load("@arm_none_eabi//:deps.bzl", "arm_none_eabi_deps")

arm_none_eabi_deps()
#---------------------------------------------------------------------

###############################################################################
# Rules Rust
###############################################################################

http_archive(
    name = "rules_rust",
    sha256 = "dc8d79fe9a5beb79d93e482eb807266a0e066e97a7b8c48d43ecf91f32a3a8f3",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.19.0/rules_rust-v0.19.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    extra_target_triples = ["thumbv7em-none-eabihf"],
    versions = ["1.64.0"],
)

# ###############################################################################
# # CARGO Crates
# ###############################################################################
load("@rules_rust//crate_universe:defs.bzl", "crates_repository")
load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies(bootstrap = True)

crates_repository(
    name = "crates",
    cargo_lockfile = "//:Cargo.lock",
    lockfile = "//:cargo-bazel-lock.json",
    manifests = [
        "//:Cargo.toml",
        "//project:Cargo.toml",
    ],
)

load("@crates//:defs.bzl", "crate_repositories")

crate_repositories()
