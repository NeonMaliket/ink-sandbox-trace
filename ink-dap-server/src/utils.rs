use dap::requests::LaunchRequestArguments;
use serde_json::json;

pub(crate) fn extract_port_from_args(args: &LaunchRequestArguments) -> Option<u16> {
    let additional_data = args.additional_data.clone().unwrap_or(json!({}));

    if let Some(args) = additional_data.get("args").and_then(|v| v.as_array()) {
        if args.len() >= 2 {
            if let Some(arg0) = args[0].as_str() {
                if arg0 == "--port" {
                    if let Some(port_str) = args[1].as_str() {
                        if let Ok(port) = port_str.parse::<u16>() {
                            return Some(port);
                        }
                    }
                }
            }
        }
    }
    None
}
