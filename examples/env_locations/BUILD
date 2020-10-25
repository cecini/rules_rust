load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_test",
)
load("@io_bazel_rules_rust//cargo:cargo_build_script.bzl", "cargo_build_script")

# generate a file
genrule(
    name = "data_generator",
    outs = ["generated.data"],
    cmd = "echo hello > $@",
)

_data = [
    # provide the generated file as a dep
    "generated.data",
    # we should also be able to access external binaries
    # such as protoc.
    "@com_google_protobuf//:protoc",
]

cargo_build_script(
    name = "build",
    srcs = ["build.rs"],
    build_script_env = {
        "GENERATED_DATA": "$(execpath generated.data)",
        "SOME_TOOL": "$(execpath @com_google_protobuf//:protoc)",
    },
    data = _data,
)

rust_test(
    name = "test",
    srcs = [
        "main.rs",
    ],
    data = _data,
    edition = "2018",
    rustc_env = {
        "GENERATED_DATA": "$(rootpath generated.data)",
        "SOME_TOOL": "$(rootpath @com_google_protobuf//:protoc)",
    },
    deps = [
        ":build",
    ],
)
