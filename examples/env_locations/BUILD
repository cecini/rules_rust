load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_test",
)

# generate a file
genrule(
    name = "data_generator",
    outs = ["generated.data"],
    cmd = "echo hello > $@",
)

rust_test(
    name = "test",
    srcs = [
        "main.rs",
    ],
    data = [
        # provide the generated file as a dep
        "generated.data",
        # we should also be able to access external binaries
        # such as protoc.
        "@com_google_protobuf//:protoc",
    ],
    edition = "2018",
    rustc_env = {
        # and pass in the locations to rustc
        "GENERATED_DATA": "$(rootpath generated.data)",
        "SOME_TOOL": "$(rootpath @com_google_protobuf//:protoc)",
    },
)
