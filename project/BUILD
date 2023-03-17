load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@crates//:defs.bzl", "aliases", "all_crate_deps")
load("//third_party/uf2:uf2.bzl", "uf2_binary")

rust_binary(
    name = "blinky",
    srcs = ["src/main.rs"],
    aliases = aliases(),
    edition = "2021",
    linker_script = ":linker.x",
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

genrule(
    name = "bin",
    srcs = [":blinky"],
    outs = ["blinky.bin"],
    cmd = "$(execpath @arm_none_eabi//:objcopy) -O binary $< $@",
    tools = ["@arm_none_eabi//:objcopy"],
)

uf2_binary(
    name = "uf2",
    base = "0x27000",
    bin = ":bin",
    family = "0xADA52840",
)
