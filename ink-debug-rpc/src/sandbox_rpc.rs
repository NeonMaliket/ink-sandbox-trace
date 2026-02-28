use polkavm::RawInstance;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct SandboxRpc {
    base_url: String,
    http: reqwest::Client,
    rt: std::sync::Arc<tokio::runtime::Runtime>,
}

#[derive(Debug, Serialize)]
struct LogRequest {
    message: String,
}

impl Default for SandboxRpc {
    fn default() -> Self {
        if let Err(e) = simple_logger::init_with_level(log::Level::Debug) {
            log::warn!("Logger error: {e}");
        };

        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("tokio runtime");

        Self {
            base_url: "http://localhost:9229".to_string(),
            http: reqwest::Client::new(),
            rt: std::sync::Arc::new(rt),
        }
    }
}

impl SandboxRpc {
    fn log_url(&self) -> String {
        format!("{}/log", self.base_url)
    }

    fn send_log_async(&self, message: String) {
        let http = self.http.clone();
        let url = self.log_url();
        let rt = self.rt.clone();

        let body = serde_json::to_string(&LogRequest { message }).unwrap();

        rt.spawn(async move {
            match http
                .post(url)
                .header("Content-Type", "application/json")
                .body(body)
                .send()
                .await
            {
                Ok(resp) => {
                    log::debug!("[send_log_async] status = {}", resp.status());
                }
                Err(e) => {
                    log::error!("[send_log_async] request error: {e}");
                }
            }
        });
    }

    pub fn step(&self, instance: &RawInstance) {
        log::debug!("[Sandbox Rpc] Step");
        let pc = instance
            .program_counter()
            .expect("[Sandbox Rpc] PC not found");

        log::info!("[Sandbox Rpc] [PC: {}]", pc);
        println!("[Sandbox Rpc] [PC: {}]", pc);

        self.send_log_async(format!("[Sandbox Rpc] [PC: {}]", pc));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_log_async() {
        let rpc = SandboxRpc::default();
        rpc.send_log_async("Test log message".into());
        std::thread::sleep(std::time::Duration::from_millis(300));
    }
}
