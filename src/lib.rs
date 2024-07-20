#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use crate::processes::{get_process_by_name, get_process_command_line1};
use napi::{Error, Status};
use winapi::shared::minwindef::FALSE;
use winapi::um::{processthreadsapi, winnt};

mod ntdll;
mod processes;

#[napi]
pub fn get_process_command_line(process_name: String) -> Result<String, Error> {
  let pid = get_process_by_name(&process_name);

  if pid <= 0 {
    return Err(Error::new(Status::GenericFailure, "Process is not running"));
  }

  let process_handle = unsafe {
    processthreadsapi::OpenProcess(
      winnt::PROCESS_QUERY_INFORMATION | winnt::PROCESS_VM_READ,
      FALSE,
      pid,
    )
  };

  let command_line = get_process_command_line1(process_handle);

  Ok(command_line)
}
