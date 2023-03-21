load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@crates//:defs.bzl", "aliases", "all_crate_deps")

rust_binary(
    name = "blinky",
    srcs = ["src/main.rs"],
    aliases = aliases(),
    edition = "2021",
    linker_script = "linker.x",
    proc_macro_deps = all_crate_deps(proc_macro = True),
    rustc_flags = [
        "--codegen=link-args=-mthumb",
        "--codegen=link-args=-mabi=aapcs",
        "--codegen=link-args=-mcpu=cortex-m4",
        "--codegen=link-args=-mfloat-abi=hard",
        "--codegen=link-args=-mfpu=fpv4-sp-d16",
        "--codegen=link-args=-mfloat-abi=hard",
    ],
    deps = all_crate_deps(normal = True),
)
