// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use deno_core::error::AnyError;
use std::rc::Rc;
use ts_module_loader::TsModuleLoader;

mod runtime;
mod ts_module_loader;

pub async fn run(file_path: &str) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path, &std::env::current_dir()?)?;
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(TsModuleLoader)),
        extensions: vec![
            runtime::fs::butler_fs::init_ops_and_esm(),
            runtime::log::butler_log::init_ops_and_esm(),
        ],
        ..Default::default()
    });

    let mod_id = js_runtime.load_main_es_module(&main_module).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(Default::default()).await?;
    result.await
}
