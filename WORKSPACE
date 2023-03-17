load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository", "new_git_repository")
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# ###############################################################################
# ARM none ebai gcc
# ###############################################################################
git_repository(
    name = "arm_none_eabi",
    commit = "52966525aa98ba8054b093336cc6aeb7e7b85259",
    patch_args = ["-p1"],
    patches = ["//third_party/rules_arm_gcc:diff.patch"],
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
load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository")
load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies(bootstrap = True)

crates_repository(
    name = "crates",
    annotations = {
        "critical-section": [crate.annotation(
            deps = ["@crates__cortex-m-0.7.6//:cortex_m"],
        )],
    },
    # cargo_config = "//.cargo:config.toml",
    cargo_lockfile = "//:Cargo.Bazel.lock",
    lockfile = "//:cargo-bazel-lock.json",
    manifests = ["//project:Cargo.toml"],
    rust_version = "1.64.0",
)

load("@crates//:defs.bzl", "crate_repositories")

crate_repositories()



# ###############################################################################
# UF2 tools
# ###############################################################################
new_git_repository(
    name = "uf2",
    build_file = "//third_party/uf2:uf2.BUILD",
    commit = "162acf4e4cb7c8e9ec589c842d277dcb5c43db89",
    patches = ["//third_party/uf2:uf2.patch"],
    remote = "https://github.com/microsoft/uf2",
    shallow_since = "1658848581 +0200",
)
