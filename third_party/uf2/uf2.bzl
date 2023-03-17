"""Bazel macro for flashing UF2 targets"""

def uf2_binary(name, bin, family, base):
    """Convert a binary file to UF2 and load it onto a UF2 device.

    Args:
        name: name of the run target
        bin: binary firmware file
        family: hex code for the UF2 chip family
        base: flash address to load the bianry at
    """
    uf2conv = "@uf2//:uf2conv"
    native.sh_binary(
        name = name,
        srcs = ["//third_party/uf2:uf2.sh"],
        env = {
            "UF2_TOOL": "$(location {})".format(uf2conv),
            "UF2_BIN": "$(location {})".format(bin),
            "UF2_BASE": base,
            "UF2_FAMILY": family,
        },
        data = [bin, uf2conv],
    )
