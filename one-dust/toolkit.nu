# check that you're using the standard code style
#
# > it is important to make `clippy` happy :relieved:
export def clippy [
    --verbose # print extra information about the command's progress
] {
    if $verbose {
        print $"running ('toolkit clippy' | pretty-print-command)"
    }

    try {(
        cargo clippy
            --all-targets
            --no-deps
            --workspace
        --
            -D warnings
            -D rustdoc::broken_intra_doc_links
            -W clippy::explicit_iter_loop
            -W clippy::explicit_into_iter_loop
            -W clippy::semicolon_if_nothing_returned
            -W clippy::doc_markdown
            -W clippy::manual_let_else
    )} catch {
        error make --unspanned {
            msg: $"\nplease fix the above ('clippy' | pretty-print-command) errors before continuing!"
        }
    }
}

export def sync [] {
    RUST_LOG=debug cargo run sync
}

export def monitor [] {
    RUST_LOG=debug cargo run monitor
}