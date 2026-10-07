use std::io;
use std::os::windows::io::{AsRawHandle, BorrowedHandle};
use std::time::Instant;
use windows_sys::Win32::Foundation::{WAIT_OBJECT_0, WAIT_TIMEOUT};
use windows_sys::Win32::System::Console::{ReadConsoleInputW, INPUT_RECORD, KEY_EVENT};
use windows_sys::Win32::System::Threading::WaitForSingleObject;

pub(super) struct Reader<'a> {
    handle: BorrowedHandle<'a>,
    repeated: u8,
    remaining: u16,
}

impl<'a> Reader<'a> {
    pub(super) fn new(handle: BorrowedHandle<'a>) -> Self {
        Self {
            handle,
            repeated: 0,
            remaining: 0,
        }
    }

    pub(super) fn read(&mut self, buf: &mut [u8], deadline: Instant) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        loop {
            if Instant::now() >= deadline {
                return Err(io::ErrorKind::TimedOut.into());
            }
            if self.remaining > 0 {
                let count = usize::from(self.remaining).min(buf.len());
                buf[..count].fill(self.repeated);
                self.remaining -= count as u16;
                return Ok(count);
            }
            let record = self.read_record(deadline)?;
            if let Some((byte, count)) = ascii_key(&record) {
                self.repeated = byte;
                self.remaining = count;
            }
        }
    }

    fn read_record(&self, deadline: Instant) -> io::Result<INPUT_RECORD> {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(io::ErrorKind::TimedOut.into());
        }
        let millis = u32::try_from(remaining.as_millis()).unwrap_or(u32::MAX - 1);
        // SAFETY: the borrowed console handle remains open for both calls.
        match unsafe { WaitForSingleObject(self.handle.as_raw_handle(), millis) } {
            WAIT_OBJECT_0 => {}
            WAIT_TIMEOUT => return Err(io::ErrorKind::TimedOut.into()),
            _ => return Err(io::Error::last_os_error()),
        }
        let mut record = INPUT_RECORD::default();
        let mut count = 0;
        // ReadFile can block after a focus, resize or key-up event wakes the
        // wait. ReadConsoleInputW consumes that event so we can check the deadline.
        // SAFETY: record and count are valid output buffers for one input record.
        if unsafe { ReadConsoleInputW(self.handle.as_raw_handle(), &mut record, 1, &mut count) }
            == 0
        {
            return Err(io::Error::last_os_error());
        }
        Ok(record)
    }
}

fn ascii_key(record: &INPUT_RECORD) -> Option<(u8, u16)> {
    if u32::from(record.EventType) != KEY_EVENT {
        return None;
    }
    // SAFETY: EventType identifies the KeyEvent union member. The W API fills UnicodeChar.
    let key = unsafe { record.Event.KeyEvent };
    let character = unsafe { key.uChar.UnicodeChar };
    // OSC and DA1 replies use ASCII.
    (key.bKeyDown != 0 && (1..=127).contains(&character))
        .then_some((character as u8, key.wRepeatCount.max(1)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::OpenOptions;
    use std::os::windows::process::CommandExt;
    use std::process::{Command, Stdio};
    use std::time::Duration;
    use terminal_trx::ConsoleHandles;
    use windows_sys::Win32::System::Console::{
        GetConsoleMode, GetStdHandle, SetConsoleMode, SetStdHandle, WriteConsoleInputW,
        ENABLE_VIRTUAL_TERMINAL_INPUT, FOCUS_EVENT, INPUT_RECORD_0, KEY_EVENT_RECORD,
        KEY_EVENT_RECORD_0, STD_OUTPUT_HANDLE, WINDOW_BUFFER_SIZE_EVENT,
    };
    use windows_sys::Win32::System::Threading::CREATE_NEW_CONSOLE;

    #[test]
    fn console_events_respect_deadline() {
        const CHILD: &str = "LUMIS_TEST_WINDOWS_CONSOLE";
        if std::env::var_os(CHILD).is_some() {
            exercise_console();
            return;
        }
        let name = module_path!().split_once("::").unwrap().1;
        let mut child = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                &format!("{name}::console_events_respect_deadline"),
                "--nocapture",
            ])
            .env(CHILD, "1")
            .creation_flags(CREATE_NEW_CONSOLE)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        // SAFETY: the child owns this live process handle. Bound the regression
        // test itself so a blocking character read fails instead of hanging CI.
        let ready = unsafe { WaitForSingleObject(child.as_raw_handle(), 10_000) };
        if ready != WAIT_OBJECT_0 {
            child.kill().unwrap();
        }
        let output = child.wait_with_output().unwrap();
        assert_eq!(
            ready, WAIT_OBJECT_0,
            "console read exceeded the test deadline"
        );
        assert!(output.status.success(), "{output:?}");
    }

    fn key(character: u16, down: bool, count: u16) -> INPUT_RECORD {
        INPUT_RECORD {
            EventType: KEY_EVENT as u16,
            Event: INPUT_RECORD_0 {
                KeyEvent: KEY_EVENT_RECORD {
                    bKeyDown: i32::from(down),
                    wRepeatCount: count,
                    uChar: KEY_EVENT_RECORD_0 {
                        UnicodeChar: character,
                    },
                    ..Default::default()
                },
            },
        }
    }

    fn write_events(handle: BorrowedHandle<'_>, events: &[INPUT_RECORD]) {
        let original = mode(handle);
        // Queue the exact records, as if they arrived before raw mode. VT input
        // can translate away key-up events and repeat counts during injection.
        // SAFETY: the borrowed handle is a live console input handle.
        assert_ne!(
            unsafe {
                SetConsoleMode(
                    handle.as_raw_handle(),
                    original & !ENABLE_VIRTUAL_TERMINAL_INPUT,
                )
            },
            0
        );
        let mut count = 0;
        // SAFETY: the input slice and count pointer are valid for this synchronous call.
        let ok = unsafe {
            WriteConsoleInputW(
                handle.as_raw_handle(),
                events.as_ptr(),
                events.len() as u32,
                &mut count,
            )
        };
        let error = io::Error::last_os_error();
        // SAFETY: restore the mode of the same live console before reading.
        assert_ne!(
            unsafe { SetConsoleMode(handle.as_raw_handle(), original) },
            0
        );
        assert_ne!(ok, 0, "{error}");
        assert_eq!(count as usize, events.len());
    }

    fn mode(handle: BorrowedHandle<'_>) -> u32 {
        let mut mode = 0;
        // SAFETY: the console handle and mode output pointer are valid.
        assert_ne!(
            unsafe { GetConsoleMode(handle.as_raw_handle(), &mut mode) },
            0
        );
        mode
    }

    fn exercise_console() {
        let output = OpenOptions::new()
            .read(true)
            .write(true)
            .open("CONOUT$")
            .unwrap();
        // terminal-trx opens its fallback CONOUT$ with write access only, but
        // GetConsoleMode requires read access. Supply a normal console stdout
        // in this isolated child and keep stderr connected to the test harness.
        // SAFETY: both handles stay open until stdout is restored, including on panic.
        let original = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
        assert_ne!(
            unsafe { SetStdHandle(STD_OUTPUT_HANDLE, output.as_raw_handle()) },
            0
        );
        let result = std::panic::catch_unwind(check_console);
        // SAFETY: original is still the live pipe handle owned by this process.
        assert_ne!(unsafe { SetStdHandle(STD_OUTPUT_HANDLE, original) }, 0);
        if let Err(panic) = result {
            std::panic::resume_unwind(panic);
        }
    }

    fn check_console() {
        let mut tty = terminal_trx::terminal().unwrap();
        let mut tty = tty.lock();
        let before = mode(tty.input_buffer_handle());
        {
            let tty = tty.enable_raw_mode().unwrap();
            let handle = tty.input_buffer_handle();
            let mut reader = Reader::new(handle);
            let noise = [
                INPUT_RECORD {
                    EventType: FOCUS_EVENT as u16,
                    ..Default::default()
                },
                INPUT_RECORD {
                    EventType: WINDOW_BUFFER_SIZE_EVENT as u16,
                    ..Default::default()
                },
                key(u16::from(b'x'), false, 1),
                key(0, true, 1),
                key(0x00e9, true, 1),
            ];
            for record in noise {
                write_events(handle, &[record]);
                let start = Instant::now();
                let error = reader
                    .read(&mut [0; 16], start + Duration::from_millis(100))
                    .unwrap_err();
                assert_eq!(error.kind(), io::ErrorKind::TimedOut);
                assert!(start.elapsed() < Duration::from_secs(1));
            }
            let reply = b"\x1b]11;rgb:22/24/36\x07\x1b[?1;2c";
            let events: Vec<_> = noise
                .into_iter()
                .chain(reply.iter().map(|&byte| key(u16::from(byte), true, 1)))
                .collect();
            write_events(handle, &events);
            let deadline = Instant::now() + Duration::from_secs(1);
            let mut actual = Vec::new();
            while actual.len() < reply.len() {
                let mut buf = [0; 16];
                let count = reader.read(&mut buf, deadline).unwrap();
                actual.extend_from_slice(&buf[..count]);
            }
            assert_eq!(actual, reply);
            write_events(handle, &[key(u16::from(b'f'), true, 4)]);
            let mut buf = [0; 3];
            assert_eq!(reader.read(&mut buf, deadline).unwrap(), 3);
            assert_eq!(&buf, b"fff");
            assert_eq!(reader.read(&mut buf, deadline).unwrap(), 1);
            assert_eq!(buf[0], b'f');
        }
        assert_eq!(mode(tty.input_buffer_handle()), before);
    }
}
