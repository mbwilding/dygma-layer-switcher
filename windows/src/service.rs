use crate::init;
use anyhow::Result;
use common::config::Config;
use common::init::{log_init, single_check};

pub fn windows_service_main(args: Vec<std::ffi::OsString>) {
    let _ = run_service(args);
}

fn run_service(_args: Vec<std::ffi::OsString>) -> Result<()> {
    use windows_service::service::ServiceControl::*;
    use windows_service::service_control_handler::ServiceControlHandlerResult::*;

    let event_handler = move |control_event| -> windows_service::service_control_handler::ServiceControlHandlerResult {
        match control_event {
            Stop => {
                // TODO: Handle this properly, this will give an error
                std::process::exit(0);
                // NoError
            },
            Interrogate => NoError,
            _ => NotImplemented,
        }
    };

    let status_handle =
        windows_service::service_control_handler::register("Dygma Layer Switcher", event_handler)?;

    let next_status = windows_service::service::ServiceStatus {
        service_type: windows_service::service::ServiceType::OWN_PROCESS,
        current_state: windows_service::service::ServiceState::Running,
        controls_accepted: windows_service::service::ServiceControlAccept::STOP,
        exit_code: windows_service::service::ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: std::time::Duration::default(),
        process_id: None,
    };

    status_handle.set_service_status(next_status)?;

    log_init();
    single_check()?;
    Config::load();

    init::start()?;

    Ok(())
}
