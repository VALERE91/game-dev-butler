use deno_core::{extension, op2};

#[op2(fast)]
fn op_log(#[string] msg: String) {
    println!("{msg}");
}

extension!(
    butler_log,
    ops = [op_log],
    esm_entry_point = "butler::log",
    esm = [dir "./runtime", "butler::log" = "log.js"],
);
