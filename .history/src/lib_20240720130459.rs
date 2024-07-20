#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use crate::processes::{get_process_by_name, get_process_command_line1};
use winapi::shared::minwindef::FALSE;
use winapi::um::{processthreadsapi, winnt};

mod ntdll;
mod processes;

#[napi]
pub fn get_process_command_line(process_name: String) -> String {
  let pid = get_process_by_name(&process_name);

  let process_handle = unsafe {
    processthreadsapi::OpenProcess(
      winnt::PROCESS_QUERY_INFORMATION | winnt::PROCESS_VM_READ,
      FALSE,
      pid,
    )
  };

  get_process_command_line1(process_handle)
}
