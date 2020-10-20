load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_file")

def rust_worker_repositories():
    http_file(
        name = "rust_worker_linux_x86_64",
        executable = True,
        sha256 = "e3383d062752bcbeeb8a97d36bba0dea244db77b45a083bee718978a0deb5d37",
        urls = ["https://github.com/nikhilm/rustc-worker/releases/download/v0.2.0/rustc-worker-linux_x86_64"],
    )
    http_file(
        name = "rust_worker_macos_x86_64",
        executable = True,
        sha256 = "189cfb96f0a5626716bb6371716ad832f153d196e45bb1c0eb4c48a1ea1be7ae",
        urls = ["https://github.com/ankitects/rustc-worker/releases/download/v0.2.0/rustc-worker-macos_x86_64"],
    )

    native.register_toolchains(
        "@io_bazel_rules_rust//worker:linux_x86_64",
    )
    native.register_toolchains(
        "@io_bazel_rules_rust//worker:macos_x86_64",
    )
