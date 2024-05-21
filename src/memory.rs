extern crate procfs;

use libc::{c_void, iovec, pid_t, process_vm_readv, process_vm_writev};
use std::convert::TryFrom;
use std::io;
use std::mem::{MaybeUninit, size_of};

#[derive(Clone)]
pub struct Memory{
    pub pid : pid_t,
    pub base_address : u64,
}

impl Memory {
    pub fn new(process_name : &str) -> Memory{
        Memory{
            pid: get_process_id_by_name(process_name).expect("Not valid pid"),
            base_address : 0x140000000,
        }
    }

    pub fn write<T>(&self, addr: u64, value: T) -> io::Result<()>
        where
            T: Copy,
    {
        let buf = unsafe {
            let mut buf = MaybeUninit::<T>::uninit();
            *buf.as_mut_ptr() = value;
            std::slice::from_raw_parts(buf.as_ptr() as *const u8, size_of::<T>())
        };

        // Write the bytes to memory
        self.write_memory(addr, buf)
    }

    pub fn read<T>(&self, addr: u64) -> io::Result<T>
        where
            T: Copy,
    {
        let mut uninit_value = MaybeUninit::<T>::uninit();

        let mut buf = unsafe {
            std::slice::from_raw_parts_mut(uninit_value.as_mut_ptr() as *mut u8, size_of::<T>())
        };

        self.read_memory(addr, &mut buf)?;

        Ok(unsafe { uninit_value.assume_init() })
    }

    pub fn read_u64(&self, addr: u64) -> u64 {
        let mut buf = [0u8; size_of::<u64>()];
        self.read_memory(addr, &mut buf).unwrap_or_default();

        // Convert the buffer to a u64
        let result = u64::from_le_bytes(buf);
        result
    }

    pub fn read_i32(&self, addr: u64) -> i32 {
        let mut buf = [0u8; size_of::<i32>()];
        self.read_memory(addr, &mut buf).unwrap_or_default();

        let result = i32::from_ne_bytes(buf);
        result
    }

    pub fn read_f32(&self, addr: u64) -> f32 {
        let mut buf = [0u8; size_of::<f32>()];
        self.read_memory(addr, &mut buf).unwrap_or_default();

        let result = f32::from_ne_bytes(buf);
        result
    }

    pub fn read_string(&self, addr: u64, bytes: usize) -> io::Result<String> {
        let mut buffer = vec![0u8; bytes];
        self.read_memory(addr, &mut buffer)?;

        if let Some(first_null) = buffer.iter().position(|&b| b == 0) {
            buffer.truncate(first_null);
        }

        Ok(String::from_utf8_lossy(&buffer).to_string())
    }

    pub fn read_utf16_string(&self, addr: u64, bytes: usize) -> io::Result<String> {
        let mut buffer = vec![0u8; bytes];
        self.read_memory(addr, &mut buffer)?;

        // Assuming the buffer is properly aligned and contains valid UTF-16 data,
        // we need to cast our u8 buffer to u16. This requires the buffer length to be even.
        if buffer.len() % 2 != 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "UTF-16 data should have even byte length"));
        }

        let utf16_data: Vec<u16> = buffer
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();

        let end = utf16_data.iter().position(|&c| c == 0).unwrap_or(utf16_data.len());

        String::from_utf16(&utf16_data[..end])
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to decode UTF-16 string"))
    }

    pub fn read_gname(&self, actor_id: i32, g_names : u64) -> io::Result<String> {
        let index = actor_id as u64 / 0x4000;
        let offset = actor_id as u64 % 0x4000;
        let name_ptr_address = g_names + index * 0x8;
        let name_ptr = self.read_u64(name_ptr_address);
        let name_address = self.read_u64(name_ptr + offset * 0x8);
        self.read_string(name_address + 0x10, 64)
    }

    pub fn read_memory(&self, addr: u64, buf: &mut [u8]) -> io::Result<()> {
        let local_iov = iovec {
            iov_base: buf.as_mut_ptr() as *mut c_void,
            iov_len: buf.len(),
        };

        let remote_iov = iovec {
            iov_base: addr as *mut c_void,
            iov_len: buf.len(),
        };

        let result = unsafe { process_vm_readv(self.pid, &local_iov, 1, &remote_iov, 1, 0) };

        if result == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    pub fn write_memory(&self, addr: u64, buf: &[u8]) -> io::Result<()> {
        let local_iov = iovec {
            iov_base: buf.as_ptr() as *mut c_void,
            iov_len: buf.len(),
        };

        let remote_iov = iovec {
            iov_base: addr as *mut c_void,
            iov_len: buf.len(),
        };

        let result = unsafe { process_vm_writev(self.pid, &local_iov, 1, &remote_iov, 1, 0) };

        if result == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}


fn get_process_id_by_name(exe_name: &str) -> Result<i32, String> {
    let all_procs = match procfs::process::all_processes() {
        Ok(procs) => procs,
        Err(_) => return Err("Failed to get process list".into()),
    };

    for proc in all_procs {
        match proc {
            Ok(proc) => { // Correctly handling the Result from iterating over all_procs
                match proc.cmdline() {
                    Ok(cmdline) => { // Directly matching against Ok(cmdline)
                        // The cmdline is a Vec<String> of the command line arguments.
                        // The first argument is usually the executable name.
                        if let Some(first_arg) = cmdline.first() {
                            if first_arg.contains(exe_name) {
                                println!("{}", proc.pid());
                                return Ok(proc.pid());
                            }
                        }
                    },
                    Err(_) => continue, // Could not read cmdline for process.
                }
            },
            Err(_) => continue, // Handling a failed process iteration
        }
    }

    Err(format!("Cannot find executable with name: {}", exe_name))
}

