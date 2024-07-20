use std::mem;
use std::ptr;

use winapi::{shared::basetsd, shared::minwindef, um::memoryapi, um::winnt, um::wow64apiset};

use crate::ntdll;

fn read_process_memory_raw(
  process_handle: winnt::HANDLE,
  base_address: minwindef::LPCVOID,
  buffer: minwindef::LPVOID,
  size: usize,
) -> bool {
  unsafe {
    let mut bytes_read: basetsd::SIZE_T = 0;
    let result = memoryapi::ReadProcessMemory(
      process_handle,
      base_address,
      buffer as minwindef::LPVOID,
      size as basetsd::SIZE_T,
      &mut bytes_read,
    );
    result != 0
  }
}

fn read_process_memory<T>(
  process_handle: winnt::HANDLE,
  base_address: minwindef::LPCVOID,
  buffer: *mut T,
) -> bool {
  read_process_memory_raw(
    process_handle,
    base_address,
    buffer as minwindef::LPVOID,
    mem::size_of::<T>(),
  )
}

pub fn get_process_peb_address(process_handle: winnt::HANDLE) -> winnt::PVOID {
  let mut basic_info: ntdll::PROCESS_BASIC_INFORMATION = unsafe { mem::zeroed() };
  if ntdll::nt_query_information_process::<ntdll::PROCESS_BASIC_INFORMATION>(
    process_handle,
    ntdll::PROCESSINFOCLASS::ProcessBasicInformation,
    &mut basic_info,
  ) {
    basic_info.PebBaseAddress
  } else {
    ptr::null_mut()
  }
}

pub fn get_process_peb_address_wow32(process_handle: winnt::HANDLE) -> winnt::PVOID {
  let mut peb_address: winnt::PVOID = ptr::null_mut();
  if ntdll::nt_query_information_process::<winnt::PVOID>(
    process_handle,
    ntdll::PROCESSINFOCLASS::ProcessWow64Information,
    &mut peb_address,
  ) {
    peb_address
  } else {
    ptr::null_mut()
  }
}

fn is_wow64_process(process_handle: winnt::HANDLE) -> bool {
  let mut result: minwindef::BOOL = minwindef::FALSE;
  unsafe {
    wow64apiset::IsWow64Process(process_handle, &mut result);
  }
  result != minwindef::FALSE
}

pub fn get_process_command_line1(process_handle: winnt::HANDLE) -> String {
  if is_wow64_process(process_handle) {
    return get_process_command_line_32(process_handle);
  }
  let empty = String::new();

  let peb_address = get_process_peb_address(process_handle);
  let mut peb: ntdll::PROCESS_ENVIRONMENT_BLOCK = unsafe { mem::zeroed() };
  if !read_process_memory::<ntdll::PROCESS_ENVIRONMENT_BLOCK>(process_handle, peb_address, &mut peb)
  {
    return empty;
  }

  let mut process_parameters: ntdll::RTL_USER_PROCESS_PARAMETERS = unsafe { mem::zeroed() };
  if !read_process_memory::<ntdll::RTL_USER_PROCESS_PARAMETERS>(
    process_handle,
    peb.ProcessParameters,
    &mut process_parameters,
  ) {
    return empty;
  }

  let byte_count = process_parameters.CommandLine.Length as usize;
  let char_count = byte_count / 2;
  let mut buffer: Vec<winnt::WCHAR> = Vec::with_capacity(char_count);
  unsafe {
    buffer.set_len(char_count);
  }
  if !read_process_memory_raw(
    process_handle,
    process_parameters.CommandLine.Buffer as minwindef::LPCVOID,
    buffer.as_mut_ptr() as minwindef::LPVOID,
    byte_count,
  ) {
    return empty;
  }
  String::from_utf16_lossy(&buffer)
}

pub fn get_process_command_line_32(process_handle: winnt::HANDLE) -> String {
  let empty = String::new();

  let peb_address = get_process_peb_address_wow32(process_handle);
  let mut peb: ntdll::PROCESS_ENVIRONMENT_BLOCK_32 = unsafe { mem::zeroed() };
  if !read_process_memory::<ntdll::PROCESS_ENVIRONMENT_BLOCK_32>(
    process_handle,
    peb_address,
    &mut peb,
  ) {
    return empty;
  }

  let mut process_parameters: ntdll::RTL_USER_PROCESS_PARAMETERS_32 = unsafe { mem::zeroed() };
  if !read_process_memory::<ntdll::RTL_USER_PROCESS_PARAMETERS_32>(
    process_handle,
    peb.ProcessParameters as minwindef::LPCVOID,
    &mut process_parameters,
  ) {
    return empty;
  }

  let byte_count = process_parameters.CommandLine.Length as usize;
  let char_count = byte_count / 2;
  let mut buffer: Vec<winnt::WCHAR> = Vec::with_capacity(char_count);
  unsafe {
    buffer.set_len(char_count);
  }
  if !read_process_memory_raw(
    process_handle,
    process_parameters.CommandLine.Buffer as minwindef::LPCVOID,
    buffer.as_mut_ptr() as minwindef::LPVOID,
    byte_count,
  ) {
    return empty;
  }
  String::from_utf16_lossy(&buffer)
}

pub fn get_process_by_name(target_name: &str) -> u32 {
  let h_process = unsafe {
    winapi::um::tlhelp32::CreateToolhelp32Snapshot(winapi::um::tlhelp32::TH32CS_SNAPPROCESS, 0)
  };

  let mut entry: winapi::um::tlhelp32::PROCESSENTRY32W = unsafe { mem::zeroed() };

  entry.dwSize = mem::size_of::<winapi::um::tlhelp32::PROCESSENTRY32W>() as u32;

  while unsafe { winapi::um::tlhelp32::Process32NextW(h_process, &mut entry) } != 0 {
    let process_name: String = String::from_utf16_lossy(&entry.szExeFile);
    if process_name.contains(target_name) {
      unsafe { winapi::um::handleapi::CloseHandle(h_process) };
      return entry.th32ProcessID;
    }
  }
  return 0;
}
