/// types for use in the WASI filesystem
#[cfg(feature = "enable-serde")]
use serde::{Deserialize, Serialize};
#[cfg(all(unix, feature = "sys-poll", not(feature = "os")))]
use std::convert::TryInto;
use std::time::Duration;
use wasmer_vbus::VirtualBusError;
use wasmer_wasi_types::wasi::{BusErrno, Errno, Rights};

#[cfg(all(not(feature = "mem-fs"), not(feature = "host-fs")))]
pub use crate::{fs::NullFile as Stderr, fs::NullFile as Stdin, fs::NullFile as Stdout};
#[cfg(feature = "host-fs")]
pub use wasmer_vfs::host_fs::{Stderr, Stdin, Stdout};
#[cfg(not(feature = "host-fs"))]
pub use wasmer_vfs::mem_fs::{Stderr, Stdin, Stdout};

use wasmer_vfs::{FsError, VirtualFile};
use wasmer_vnet::NetworkError;

pub fn fs_error_from_wasi_err(err: Errno) -> FsError {
    match err {
        Errno::Badf => FsError::InvalidFd,
        Errno::Exist => FsError::AlreadyExists,
        Errno::Io => FsError::IOError,
        Errno::Addrinuse => FsError::AddressInUse,
        Errno::Addrnotavail => FsError::AddressNotAvailable,
        Errno::Pipe => FsError::BrokenPipe,
        Errno::Connaborted => FsError::ConnectionAborted,
        Errno::Connrefused => FsError::ConnectionRefused,
        Errno::Connreset => FsError::ConnectionReset,
        Errno::Intr => FsError::Interrupted,
        Errno::Inval => FsError::InvalidInput,
        Errno::Notconn => FsError::NotConnected,
        Errno::Nodev => FsError::NoDevice,
        Errno::Noent => FsError::EntryNotFound,
        Errno::Perm => FsError::PermissionDenied,
        Errno::Timedout => FsError::TimedOut,
        Errno::Proto => FsError::UnexpectedEof,
        Errno::Again => FsError::WouldBlock,
        Errno::Nospc => FsError::WriteZero,
        Errno::Notempty => FsError::DirectoryNotEmpty,
        _ => FsError::UnknownError,
    }
}

pub fn fs_error_into_wasi_err(fs_error: FsError) -> Errno {
    match fs_error {
        FsError::AlreadyExists => Errno::Exist,
        FsError::AddressInUse => Errno::Addrinuse,
        FsError::AddressNotAvailable => Errno::Addrnotavail,
        FsError::BaseNotDirectory => Errno::Notdir,
        FsError::BrokenPipe => Errno::Pipe,
        FsError::ConnectionAborted => Errno::Connaborted,
        FsError::ConnectionRefused => Errno::Connrefused,
        FsError::ConnectionReset => Errno::Connreset,
        FsError::Interrupted => Errno::Intr,
        FsError::InvalidData => Errno::Io,
        FsError::InvalidFd => Errno::Badf,
        FsError::InvalidInput => Errno::Inval,
        FsError::IOError => Errno::Io,
        FsError::NoDevice => Errno::Nodev,
        FsError::NotAFile => Errno::Inval,
        FsError::NotConnected => Errno::Notconn,
        FsError::EntryNotFound => Errno::Noent,
        FsError::PermissionDenied => Errno::Perm,
        FsError::TimedOut => Errno::Timedout,
        FsError::UnexpectedEof => Errno::Proto,
        FsError::WouldBlock => Errno::Again,
        FsError::WriteZero => Errno::Nospc,
        FsError::DirectoryNotEmpty => Errno::Notempty,
        FsError::Lock | FsError::UnknownError => Errno::Io,
    }
}

pub fn net_error_into_wasi_err(net_error: NetworkError) -> Errno {
    match net_error {
        NetworkError::InvalidFd => Errno::Badf,
        NetworkError::AlreadyExists => Errno::Exist,
        NetworkError::Lock => Errno::Io,
        NetworkError::IOError => Errno::Io,
        NetworkError::AddressInUse => Errno::Addrinuse,
        NetworkError::AddressNotAvailable => Errno::Addrnotavail,
        NetworkError::BrokenPipe => Errno::Pipe,
        NetworkError::ConnectionAborted => Errno::Connaborted,
        NetworkError::ConnectionRefused => Errno::Connrefused,
        NetworkError::ConnectionReset => Errno::Connreset,
        NetworkError::Interrupted => Errno::Intr,
        NetworkError::InvalidData => Errno::Io,
        NetworkError::InvalidInput => Errno::Inval,
        NetworkError::NotConnected => Errno::Notconn,
        NetworkError::NoDevice => Errno::Nodev,
        NetworkError::PermissionDenied => Errno::Perm,
        NetworkError::TimedOut => Errno::Timedout,
        NetworkError::UnexpectedEof => Errno::Proto,
        NetworkError::WouldBlock => Errno::Again,
        NetworkError::WriteZero => Errno::Nospc,
        NetworkError::Unsupported => Errno::Notsup,
        NetworkError::UnknownError => Errno::Io,
    }
}

pub fn vbus_error_into_bus_errno(bus_error: VirtualBusError) -> BusErrno {
    use VirtualBusError::*;
    match bus_error {
        Serialization => BusErrno::Ser,
        Deserialization => BusErrno::Des,
        InvalidWapm => BusErrno::Wapm,
        FetchFailed => BusErrno::Fetch,
        CompileError => BusErrno::Compile,
        InvalidABI => BusErrno::Abi,
        Aborted => BusErrno::Aborted,
        BadHandle => BusErrno::Badhandle,
        InvalidTopic => BusErrno::Topic,
        BadCallback => BusErrno::Badcb,
        Unsupported => BusErrno::Unsupported,
        BadRequest => BusErrno::Badrequest,
        AccessDenied => BusErrno::Denied,
        InternalError => BusErrno::Internal,
        MemoryAllocationFailed => BusErrno::Alloc,
        InvokeFailed => BusErrno::Invoke,
        AlreadyConsumed => BusErrno::Consumed,
        MemoryAccessViolation => BusErrno::Memviolation,
        _ => BusErrno::Unknown,
    }
}

pub fn bus_errno_into_vbus_error(bus_error: BusErrno) -> VirtualBusError {
    use VirtualBusError::*;
    match bus_error {
        BusErrno::Ser => Serialization,
        BusErrno::Des => Deserialization,
        BusErrno::Wapm => InvalidWapm,
        BusErrno::Fetch => FetchFailed,
        BusErrno::Compile => CompileError,
        BusErrno::Abi => InvalidABI,
        BusErrno::Aborted => Aborted,
        BusErrno::Badhandle => BadHandle,
        BusErrno::Topic => InvalidTopic,
        BusErrno::Badcb => BadCallback,
        BusErrno::Unsupported => Unsupported,
        BusErrno::Badrequest => BadRequest,
        BusErrno::Denied => AccessDenied,
        BusErrno::Internal => InternalError,
        BusErrno::Alloc => MemoryAllocationFailed,
        BusErrno::Invoke => InvokeFailed,
        BusErrno::Consumed => AlreadyConsumed,
        BusErrno::Memviolation => MemoryAccessViolation,
        _ => UnknownError,
    }
}

#[allow(dead_code)]
pub(crate) fn bus_read_rights() -> Rights {
    Rights::FD_FDSTAT_SET_FLAGS
        .union(Rights::FD_FILESTAT_GET)
        .union(Rights::FD_READ)
        .union(Rights::POLL_FD_READWRITE)
}

#[allow(dead_code)]
pub(crate) fn bus_write_rights() -> Rights {
    Rights::FD_FDSTAT_SET_FLAGS
        .union(Rights::FD_FILESTAT_GET)
        .union(Rights::FD_WRITE)
        .union(Rights::POLL_FD_READWRITE)
}

#[derive(Debug, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum PollEvent {
    /// Data available to read
    PollIn = 1,
    /// Data available to write (will still block if data is greater than space available unless
    /// the fd is configured to not block)
    PollOut = 2,
    /// Something didn't work. ignored as input
    PollError = 4,
    /// Connection closed. ignored as input
    PollHangUp = 8,
    /// Invalid request. ignored as input
    PollInvalid = 16,
}

impl PollEvent {
    fn from_i16(raw_num: i16) -> Option<PollEvent> {
        Some(match raw_num {
            1 => PollEvent::PollIn,
            2 => PollEvent::PollOut,
            4 => PollEvent::PollError,
            8 => PollEvent::PollHangUp,
            16 => PollEvent::PollInvalid,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PollEventBuilder {
    inner: PollEventSet,
}

pub type PollEventSet = i16;

#[derive(Debug)]
pub struct PollEventIter {
    pes: PollEventSet,
    i: usize,
}

impl Iterator for PollEventIter {
    type Item = PollEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pes == 0 || self.i > 15 {
            None
        } else {
            while self.i < 16 {
                let result = PollEvent::from_i16(self.pes & (1 << self.i));
                self.pes &= !(1 << self.i);
                self.i += 1;
                if let Some(r) = result {
                    return Some(r);
                }
            }
            unreachable!("Internal logic error in PollEventIter");
        }
    }
}

pub fn iterate_poll_events(pes: PollEventSet) -> PollEventIter {
    PollEventIter { pes, i: 0 }
}

#[cfg(all(unix, feature = "sys-poll", not(feature = "os")))]
fn poll_event_set_to_platform_poll_events(mut pes: PollEventSet) -> i16 {
    let mut out = 0;
    for i in 0..16 {
        out |= match PollEvent::from_i16(pes & (1 << i)) {
            Some(PollEvent::PollIn) => libc::POLLIN,
            Some(PollEvent::PollOut) => libc::POLLOUT,
            Some(PollEvent::PollError) => libc::POLLERR,
            Some(PollEvent::PollHangUp) => libc::POLLHUP,
            Some(PollEvent::PollInvalid) => libc::POLLNVAL,
            _ => 0,
        };
        pes &= !(1 << i);
    }
    out
}

#[cfg(all(unix, feature = "sys-poll", not(feature = "os")))]
fn platform_poll_events_to_pollevent_set(mut num: i16) -> PollEventSet {
    let mut peb = PollEventBuilder::new();
    for i in 0..16 {
        peb = match num & (1 << i) {
            libc::POLLIN => peb.add(PollEvent::PollIn),
            libc::POLLOUT => peb.add(PollEvent::PollOut),
            libc::POLLERR => peb.add(PollEvent::PollError),
            libc::POLLHUP => peb.add(PollEvent::PollHangUp),
            libc::POLLNVAL => peb.add(PollEvent::PollInvalid),
            _ => peb,
        };
        num &= !(1 << i);
    }
    peb.build()
}

#[allow(dead_code)]
impl PollEventBuilder {
    pub fn new() -> PollEventBuilder {
        PollEventBuilder { inner: 0 }
    }

    pub fn add(mut self, event: PollEvent) -> PollEventBuilder {
        self.inner |= event as PollEventSet;
        self
    }

    pub fn build(self) -> PollEventSet {
        self.inner
    }
}

#[cfg(all(unix, feature = "sys-poll", not(feature = "os")))]
pub(crate) fn poll(
    selfs: &[&(dyn VirtualFile + Send + Sync + 'static)],
    events: &[PollEventSet],
    seen_events: &mut [PollEventSet],
    timeout: Duration,
) -> Result<u32, FsError> {
    if !(selfs.len() == events.len() && events.len() == seen_events.len()) {
        return Err(FsError::InvalidInput);
    }
    let mut fds = selfs
        .iter()
        .enumerate()
        .filter_map(|(i, s)| s.get_fd().map(|rfd| (i, rfd)))
        .map(|(i, host_fd)| libc::pollfd {
            fd: host_fd.try_into().unwrap(),
            events: poll_event_set_to_platform_poll_events(events[i]),
            revents: 0,
        })
        .collect::<Vec<_>>();
    let result = unsafe {
        libc::poll(
            fds.as_mut_ptr(),
            selfs.len() as _,
            timeout.as_millis() as i32,
        )
    };

    if result < 0 {
        // TODO: check errno and return value
        return Err(FsError::IOError);
    }
    // convert result and write back values
    for (i, fd) in fds.into_iter().enumerate() {
        seen_events[i] = platform_poll_events_to_pollevent_set(fd.revents);
    }
    // unwrap is safe because we check for negative values above
    Ok(result.try_into().unwrap())
}

#[cfg(any(not(unix), not(feature = "sys-poll"), feature = "os"))]
pub(crate) fn poll(
    files: &[&(dyn VirtualFile + Send + Sync + 'static)],
    // FIXME: evaluate changd signature
    // files: &[super::InodeValFilePollGuard],
    events: &[PollEventSet],
    seen_events: &mut [PollEventSet],
    timeout: Duration,
) -> Result<u32, FsError> {
    if !(files.len() == events.len() && events.len() == seen_events.len()) {
        tracing::debug!("the slice length of 'files', 'events' and 'seen_events' must be the same (files={}, events={}, seen_events={})", files.len(), events.len(), seen_events.len());
        return Err(FsError::InvalidInput);
    }

    let mut ret = 0;
    for (n, file) in files.iter().enumerate() {
        let mut builder = PollEventBuilder::new();

        let can_read = file.bytes_available_read()? > 0;
        let can_write = file.bytes_available_write()? > 0;
        let is_closed = file.is_open() == false;

        tracing::debug!(
            "poll_evt can_read={} can_write={} is_closed={}",
            can_read,
            can_write,
            is_closed
        );

        for event in iterate_poll_events(events[n]) {
            match event {
                PollEvent::PollIn if can_read => {
                    builder = builder.add(PollEvent::PollIn);
                }
                PollEvent::PollOut if can_write => {
                    builder = builder.add(PollEvent::PollOut);
                }
                PollEvent::PollHangUp if is_closed => {
                    builder = builder.add(PollEvent::PollHangUp);
                }
                PollEvent::PollInvalid if is_closed => {
                    builder = builder.add(PollEvent::PollInvalid);
                }
                PollEvent::PollError if is_closed => {
                    builder = builder.add(PollEvent::PollError);
                }
                _ => {}
            }
        }
        let revents = builder.build();
        if revents != 0 {
            ret += 1;
        }
        seen_events[n] = revents;
    }

    if ret == 0 && timeout > Duration::ZERO {
        return Err(FsError::WouldBlock);
    }

    Ok(ret)
}

pub trait WasiPath {}

#[deprecated(
    since = "3.0.0-beta.2",
    note = "Moved to `wasmer_wasi::pipe::WasiBidirectionalSharedPipePair`, `Pipe` is only a transitional reexport"
)]
pub use crate::state::WasiBidirectionalSharedPipePair as Pipe;

/*
TODO: Think about using this
trait WasiFdBacking: std::fmt::Debug {
    fn get_stat(&self) -> &Filestat;
    fn get_stat_mut(&mut self) -> &mut Filestat;
    fn is_preopened(&self) -> bool;
    fn get_name(&self) -> &str;
}
*/
