fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        wasm_logger::init(wasm_logger::Config::default());
        log::info!("Loaded download_worker.rs!");

        use task_simple::gloo_worker::Registrable;
        console_error_panic_hook::set_once();
        task_simple::WebWorkerBackground::<bg_task_example::DownloadFunction>::registrar()
            .register();
    }
}
