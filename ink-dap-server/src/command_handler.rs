use std::io::{Stdin, Stdout};

use dap::base_message::Sendable;
use dap::events::Event;
use dap::requests::{
    AttachRequestArguments, Command, ContinueArguments, DisconnectArguments, InitializeArguments,
    LaunchRequestArguments, PauseArguments, Request, RestartArguments, ScopesArguments,
    SetBreakpointsArguments, SetExceptionBreakpointsArguments, StackTraceArguments,
    VariablesArguments,
};
use dap::responses::{
    ContinueResponse, Response, ResponseBody, ResponseMessage, ScopesResponse,
    SetBreakpointsResponse, SetExceptionBreakpointsResponse, StackTraceResponse, ThreadsResponse,
    VariablesResponse,
};
use dap::server::Server;
use dap::types::{
    Breakpoint, Capabilities, Scope, Source, StackFrame, StoppedEventReason, Thread, Variable,
};

use crate::log::dap_log;
use crate::state::DapState;
use crate::types::DynResult;
use crate::utils::extract_port_from_args;

// --------------------
// ROUTER
// --------------------
pub(crate) fn handle(
    req: Request,
    server: &mut Server<Stdin, Stdout>,
    state: &mut DapState,
) -> DynResult<()> {
    dap_log(server, "--- New DAP Request Received ---");
    dap_log(server, format!("DAP STATE: {state:?}"));
    dap_log(server, "----------------------------------");
    match &req.command {
        Command::Initialize(args) => handle_initialize(req.clone(), args, server),
        Command::Launch(args) => handle_launch(req.clone(), args, server, state),
        Command::Restart(args) => handle_restart(req.clone(), args, server),
        Command::Attach(args) => handle_attach(req.clone(), args, server),
        Command::ConfigurationDone => handle_configuration_done(req.clone(), server),
        Command::SetBreakpoints(args) => handle_set_breakpoints(req.clone(), args, server, state),
        Command::SetExceptionBreakpoints(args) => {
            handle_set_exception_breakpoints(req.clone(), args, server)
        }
        Command::Threads => handle_threads(req.clone(), server, state),
        Command::Pause(args) => handle_pause(req.clone(), args, server, state),
        Command::Continue(args) => handle_continue(req.clone(), args, server, state),
        Command::StackTrace(args) => handle_stack_trace(req.clone(), args, server, state),
        Command::Scopes(args) => handle_scopes(req.clone(), args, server, state),
        Command::Variables(args) => handle_variables(req.clone(), args, server, state),
        Command::Disconnect(args) => handle_disconnect(req.clone(), args, server),
        _ => handle_unsupported(req, server),
    }
}

// --------------------
// HANDLERS
// --------------------
fn handle_initialize(
    req: Request,
    args: &InitializeArguments,
    server: &mut Server<Stdin, Stdout>,
) -> DynResult<()> {
    dap_log(server, format!("Initialize: {args:?}"));

    // Минимальные capabilities чтобы VS Code начал слать стандартные запросы.
    // Если у тебя в crate `dap` другие поля/имена — замени по аналогии.
    let caps = Capabilities {
        supports_configuration_done_request: Some(true),
        supports_set_variable: Some(false),
        supports_step_back: Some(false),
        supports_restart_frame: Some(false),
        supports_goto_targets_request: Some(false),
        supports_conditional_breakpoints: Some(false),
        supports_hit_conditional_breakpoints: Some(false),
        supports_terminate_request: Some(false),
        supports_evaluate_for_hovers: Some(false),
        ..Default::default()
    };

    server.respond(req.success(ResponseBody::Initialize(caps)))?;
    server.send_event(Event::Initialized)?;
    Ok(())
}

fn handle_configuration_done(req: Request, server: &mut Server<Stdin, Stdout>) -> DynResult<()> {
    dap_log(server, "ConfigurationDone");
    server.respond(req.success(ResponseBody::ConfigurationDone))?;
    Ok(())
}

fn handle_launch(
    req: Request,
    args: &LaunchRequestArguments,
    server: &mut Server<Stdin, Stdout>,
    _st: &mut DapState,
) -> DynResult<()> {
    dap_log(server, format!("Launch: {args:?}"));
    let port = extract_port_from_args(args);
    dap_log(server, format!("Running on port: {port:?}"));

    server.respond(req.success(ResponseBody::Launch))?;
    Ok(())
}

fn handle_restart(
    req: Request,
    args: &RestartArguments,
    server: &mut Server<Stdin, Stdout>,
) -> DynResult<()> {
    dap_log(server, format!("Restart: {args:?}"));
    server.respond(req.success(ResponseBody::Restart))?;
    Ok(())
}

fn handle_attach(
    req: Request,
    args: &AttachRequestArguments,
    server: &mut Server<Stdin, Stdout>,
) -> DynResult<()> {
    dap_log(server, format!("Attach: {args:?}"));
    server.respond(req.success(ResponseBody::Attach))?;
    Ok(())
}

fn handle_set_breakpoints(
    req: Request,
    args: &SetBreakpointsArguments,
    server: &mut Server<Stdin, Stdout>,
    st: &mut DapState,
) -> DynResult<()> {
    dap_log(server, format!("SetBreakpoints: {args:?}"));

    // Запомнить source чтобы потом отдать stackTrace с тем же source/path
    st.current_source = Some(args.source.clone());

    // Сохранить линии брейков по path
    if let Some(path) = args.source.path.clone() {
        let mut lines = Vec::new();
        if let Some(source_breakpoints) = &args.breakpoints {
            for bp in source_breakpoints {
                lines.push(bp.line);
            }
        }
        st.breakpoints_by_path.insert(path, lines);
    }

    let mut breakpoints = Vec::new();
    if let Some(source_breakpoints) = &args.breakpoints {
        for (i, src_bp) in source_breakpoints.iter().enumerate() {
            breakpoints.push(Breakpoint {
                id: Some(i as i64 + 1),
                verified: true,
                message: None,
                source: Some(args.source.clone()),
                line: Some(src_bp.line),
                column: src_bp.column,
                end_line: None,
                end_column: None,
                instruction_reference: None,
                offset: None,
            });

            dap_log(server, format!("Set breakpoint at line {}", src_bp.line));
        }
    }

    // ВАЖНО: на SetBreakpoints должен быть РОВНО ОДИН ответ SetBreakpointsResponse
    server.respond(
        req.success(ResponseBody::SetBreakpoints(SetBreakpointsResponse {
            breakpoints,
        })),
    )?;
    Ok(())
}

fn handle_set_exception_breakpoints(
    req: Request,
    args: &SetExceptionBreakpointsArguments,
    server: &mut Server<Stdin, Stdout>,
) -> DynResult<()> {
    dap_log(server, format!("SetExceptionBreakpoints: {args:?}"));

    server.respond(req.success(ResponseBody::SetExceptionBreakpoints(
        SetExceptionBreakpointsResponse { breakpoints: None },
    )))?;
    Ok(())
}

fn handle_threads(
    req: Request,
    server: &mut Server<Stdin, Stdout>,
    st: &mut DapState,
) -> DynResult<()> {
    dap_log(server, "Threads request received");

    let threads = vec![Thread {
        id: st.main_thread_id,
        name: "Main Thread".to_string(),
    }];

    server.respond(req.success(ResponseBody::Threads(ThreadsResponse { threads })))?;
    Ok(())
}

fn handle_pause(
    req: Request,
    args: &PauseArguments,
    server: &mut Server<Stdin, Stdout>,
    st: &mut DapState,
) -> DynResult<()> {
    dap_log(server, format!("Pause: {args:?}"));

    server.respond(req.success(ResponseBody::Pause))?;

    // выбрать линию, куда “остановились” (для демо — первый брейкпоинт или 1)
    st.pick_stop_location();

    // ВАЖНО: после PauseResponse нужно послать Stopped event
    server.send_event(Event::Stopped(dap::events::StoppedEventBody {
        reason: StoppedEventReason::Pause,
        description: Some("Paused".to_string()),
        thread_id: Some(st.main_thread_id),
        preserve_focus_hint: Some(false),
        text: None,
        all_threads_stopped: Some(true),
        hit_breakpoint_ids: None,
    }))?;

    Ok(())
}

fn handle_continue(
    req: Request,
    args: &ContinueArguments,
    server: &mut Server<Stdin, Stdout>,
    st: &mut DapState,
) -> DynResult<()> {
    dap_log(server, format!("Continue: {args:?}"));

    server.respond(req.success(ResponseBody::Continue(ContinueResponse {
        all_threads_continued: Some(true),
    })))?;

    server.send_event(Event::Continued(dap::events::ContinuedEventBody {
        thread_id: st.main_thread_id,
        all_threads_continued: Some(true),
    }))?;

    Ok(())
}

fn handle_stack_trace(
    req: Request,
    args: &StackTraceArguments,
    server: &mut Server<Stdin, Stdout>,
    st: &mut DapState,
) -> DynResult<()> {
    dap_log(server, format!("StackTrace: {args:?}"));

    let source = st.current_source.clone().unwrap_or(Source {
        name: Some("unknown".to_string()),
        path: None,
        source_reference: None,
        presentation_hint: None,
        origin: None,
        sources: None,
        adapter_data: None,
        checksums: None,
    });

    let frames = vec![StackFrame {
        id: 1,
        name: "main".to_string(),
        source: Some(source),
        line: st.stopped_line,
        column: st.stopped_column,
        end_line: None,
        end_column: None,
        can_restart: None,
        instruction_pointer_reference: None,
        module_id: None,
        presentation_hint: None,
    }];

    server.respond(req.success(ResponseBody::StackTrace(StackTraceResponse {
        stack_frames: frames,
        total_frames: Some(1),
    })))?;

    Ok(())
}

fn handle_scopes(
    req: Request,
    args: &ScopesArguments,
    server: &mut Server<Stdin, Stdout>,
    st: &mut DapState,
) -> DynResult<()> {
    dap_log(server, format!("Scopes: {args:?}"));

    let scopes = vec![Scope {
        name: "Locals".to_string(),
        presentation_hint: None,
        variables_reference: st.vars_ref,
        named_variables: None,
        indexed_variables: None,
        expensive: false,
        source: None,
        line: None,
        column: None,
        end_line: None,
        end_column: None,
    }];

    server.respond(req.success(ResponseBody::Scopes(ScopesResponse { scopes })))?;
    Ok(())
}

fn handle_variables(
    req: Request,
    args: &VariablesArguments,
    server: &mut Server<Stdin, Stdout>,
    _st: &mut DapState,
) -> DynResult<()> {
    dap_log(server, format!("Variables: {args:?}"));

    let variables = vec![Variable {
        name: "demo".to_string(),
        value: "1".to_string(),
        type_field: Some("i32".to_string()),
        presentation_hint: None,
        evaluate_name: Some("demo".to_string()),
        variables_reference: 0,
        named_variables: None,
        indexed_variables: None,
        memory_reference: None,
    }];

    server.respond(req.success(ResponseBody::Variables(VariablesResponse { variables })))?;
    Ok(())
}

fn handle_disconnect(
    req: Request,
    args: &DisconnectArguments,
    server: &mut Server<Stdin, Stdout>,
) -> DynResult<()> {
    dap_log(server, format!("Disconnect: {args:?}"));
    server.respond(req.success(ResponseBody::Disconnect))?;
    Ok(())
}

fn handle_unsupported(req: Request, server: &mut Server<Stdin, Stdout>) -> DynResult<()> {
    dap_log(server, format!("Unsupported command: {:?}", req.command));

    server.send(Sendable::Response(Response {
        request_seq: req.seq,
        success: false,
        message: Some(ResponseMessage::Error(format!(
            "Unsupported command: {:?}",
            req.command
        ))),
        body: None,
        error: None,
    }))?;
    Ok(())
}
