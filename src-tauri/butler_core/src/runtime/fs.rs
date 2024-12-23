use deno_core::{error::AnyError, extension, op2};

#[op2(async)]
#[string]
async fn op_read_file(#[string] path: String) -> Result<String, AnyError> {
    tokio::fs::read_to_string(path).await.map_err(Into::into)
}

#[op2(async)]
async fn op_write_file(#[string] path: String, #[string] content: String) -> Result<(), AnyError> {
    tokio::fs::write(path, content).await.map_err(Into::into)
}

#[op2(fast)]
fn op_remove_file(#[string] path: String) -> Result<(), AnyError> {
    std::fs::remove_file(path).map_err(Into::into)
}

extension!(butler_fs,
    ops = [op_read_file, op_write_file, op_remove_file,],
    esm_entry_point = "butler::fs",
    esm = [dir "./runtime", "butler::fs" = "fs.js"],);
