"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""

package(default_visibility = [
    # Public for visibility by "@raze__crate__version//" targets.
    #
    # Prefer access through "//wasm_bindgen/raze", which limits external
    # visibility to explicit Cargo.toml dependencies.
    "//visibility:public",
])

licenses([
    "notice",  # "MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_binary",
    "rust_library",
    "rust_test",
)

# Unsupported target "alternate_screen" with type "example" omitted
# Unsupported target "alternate_screen_raw" with type "example" omitted
# Unsupported target "async" with type "example" omitted
# Unsupported target "click" with type "example" omitted
# Unsupported target "color" with type "example" omitted
# Unsupported target "commie" with type "example" omitted
# Unsupported target "detect_color" with type "example" omitted
# Unsupported target "is_tty" with type "example" omitted
# Unsupported target "keys" with type "example" omitted
# Unsupported target "mouse" with type "example" omitted
# Unsupported target "rainbow" with type "example" omitted
# Unsupported target "read" with type "example" omitted
# Unsupported target "rustc_fun" with type "example" omitted
# Unsupported target "simple" with type "example" omitted
# Unsupported target "size" with type "example" omitted

rust_library(
    name = "termion",
    srcs = glob(["**/*.rs"]),
    crate_features = [
    ],
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "1.5.3",
    deps = [
        "@raze__libc__0_2_58//:libc",
        "@raze__numtoa__0_1_0//:numtoa",
    ],
)

# Unsupported target "truecolor" with type "example" omitted
