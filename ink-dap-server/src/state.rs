use std::collections::HashMap;

use dap::types::Source;

#[derive(Default, Debug)]
pub(crate) struct DapState {
    pub(crate) main_thread_id: i64,
    pub(crate) current_source: Option<Source>,
    pub(crate) stopped_line: i64,
    pub(crate) stopped_column: i64,
    pub(crate) breakpoints_by_path: HashMap<String, Vec<i64>>,
    pub(crate) vars_ref: i64,
}

impl DapState {
    pub(crate) fn new() -> Self {
        Self {
            main_thread_id: 1,
            current_source: None,
            stopped_line: 1,
            stopped_column: 1,
            breakpoints_by_path: HashMap::new(),
            vars_ref: 2000,
        }
    }

    pub(crate) fn pick_stop_location(&mut self) {
        if let Some(src) = &self.current_source {
            if let Some(path) = &src.path {
                if let Some(lines) = self.breakpoints_by_path.get(path) {
                    if let Some(first) = lines.first() {
                        self.stopped_line = *first;
                        self.stopped_column = 1;
                        return;
                    }
                }
            }
        }
        self.stopped_line = 1;
        self.stopped_column = 1;
    }
}
