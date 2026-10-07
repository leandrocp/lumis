use lumis_core::themes::TerminalColors;
#[cfg(any(not(windows), test))]
use std::io::Read;
use std::io::{self, Write};
use std::time::{Duration, Instant};

#[cfg(windows)]
mod windows;

const QUERY: &[u8] = b"\x1b]10;?\x07\x1b]11;?\x07\x1b]4;1;?\x07\x1b]4;2;?\x07\x1b]4;3;?\x07\x1b]4;4;?\x07\x1b]4;5;?\x07\x1b]4;6;?\x07\x1b[c";
const TIMEOUT: Duration = Duration::from_secs(1);

// Colorsaurus requires both OSC 10/11 replies and has no OSC 4 query. Keep
// partial replies in one exchange; separate OSC 4 commands also work on Konsole.
pub(crate) fn query() -> io::Result<TerminalColors> {
    if !supported_term(std::env::var("TERM").ok().as_deref()) {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "terminal does not support color queries",
        ));
    }
    let mut tty = terminal_trx::terminal()?;
    let mut tty = tty.lock();
    let mut tty = tty.enable_raw_mode()?;
    tty.write_all(QUERY)?;
    tty.flush()?;

    let deadline = Instant::now() + TIMEOUT;
    #[cfg(windows)]
    {
        use terminal_trx::ConsoleHandles;
        let mut reader = windows::Reader::new(tty.input_buffer_handle());
        read_colors(|buf| reader.read(buf, deadline))
    }
    #[cfg(not(windows))]
    read_colors(|buf| {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() || !wait_readable(&tty, remaining)? {
            return Err(io::ErrorKind::TimedOut.into());
        }
        tty.read(buf)
    })
}

fn supported_term(term: Option<&str>) -> bool {
    match term {
        None => cfg!(windows),
        Some("dumb" | "Eterm" | "screen" | "") => false,
        Some(term) => !term.starts_with("screen."),
    }
}

fn read_colors(mut read: impl FnMut(&mut [u8]) -> io::Result<usize>) -> io::Result<TerminalColors> {
    let mut parser = vte::Parser::new();
    let mut response = Response::default();
    let mut buf = [0; 256];
    while !response.done {
        match read(&mut buf) {
            Ok(0) => break,
            Ok(count) => parser.advance(&mut response, &buf[..count]),
            Err(err) if err.kind() == io::ErrorKind::Interrupted => {}
            Err(err) if err.kind() == io::ErrorKind::TimedOut => {
                if response.colors.background.is_none() {
                    return Err(err);
                }
                break;
            }
            Err(err) => return Err(err),
        }
    }
    if response.colors.background.is_none() {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "terminal did not report a background color",
        ));
    }
    Ok(response.colors)
}

#[derive(Default)]
struct Response {
    colors: TerminalColors,
    done: bool,
}

impl vte::Perform for Response {
    fn osc_dispatch(&mut self, params: &[&[u8]], _bell_terminated: bool) {
        match params {
            [b"10", value] => {
                if let Some(color) = parse_color(value) {
                    self.colors.foreground = Some(color);
                }
            }
            [b"11", value] => {
                if let Some(color) = parse_color(value) {
                    self.colors.background = Some(color);
                }
            }
            [b"4", entries @ ..] => {
                for pair in entries.as_chunks::<2>().0 {
                    let index = std::str::from_utf8(pair[0])
                        .ok()
                        .and_then(|n| n.parse::<usize>().ok());
                    if let (Some(slot), Some(color)) = (
                        index.and_then(|i| self.colors.ansi.get_mut(i)),
                        parse_color(pair[1]),
                    ) {
                        *slot = Some(color);
                    }
                }
            }
            _ => {}
        }
    }

    fn csi_dispatch(
        &mut self,
        _params: &vte::Params,
        intermediates: &[u8],
        ignore: bool,
        action: char,
    ) {
        if !ignore && intermediates == b"?" && action == 'c' {
            self.done = true;
        }
    }
}

fn parse_color(value: &[u8]) -> Option<(u8, u8, u8)> {
    let color = xterm_color::Color::parse(value).ok()?;
    Some((
        color.red.to_be_bytes()[0],
        color.green.to_be_bytes()[0],
        color.blue.to_be_bytes()[0],
    ))
}

#[cfg(unix)]
fn wait_readable(tty: &impl terminal_trx::Transceive, timeout: Duration) -> io::Result<bool> {
    use rustix::event::{fd_set_insert, fd_set_num_elements, select, FdSetElement, Timespec};
    let fd = tty.as_raw_fd();
    let mut set = vec![FdSetElement::default(); fd_set_num_elements(1, fd + 1)];
    fd_set_insert(&mut set, fd);
    let timeout = Timespec::try_from(timeout).map_err(io::Error::other)?;
    // SAFETY: the set contains only the borrowed terminal's live descriptor.
    // select works on macOS /dev/tty, where poll and kqueue do not.
    unsafe { select(fd + 1, Some(&mut set), None, None, Some(&timeout)) }
        .map(|ready| ready > 0)
        .map_err(Into::into)
}

#[cfg(not(any(unix, windows)))]
fn wait_readable(_tty: &impl terminal_trx::Transceive, _timeout: Duration) -> io::Result<bool> {
    Err(io::ErrorKind::Unsupported.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fragmented_reordered_and_partial_replies() {
        let bytes = b"\x1b]4;6;rgb:f/0/8\x07\x1b]11;rgb:1a1a/1b1b/2626\x1b\\\x1b]10;rgb:c0/ca/f5\x07\x1b[?1;2c";
        let mut offset = 0;
        let colors = read_colors(|buf| {
            assert!(offset < bytes.len(), "must stop after DA1");
            buf[0] = bytes[offset];
            offset += 1;
            Ok(1)
        })
        .unwrap();
        assert_eq!(offset, bytes.len());
        assert_eq!(colors.background, Some((26, 27, 38)));
        assert_eq!(colors.foreground, Some((192, 202, 245)));
        assert_eq!(colors.ansi[6], Some((255, 0, 136)));
        assert_eq!(colors.ansi[1], None);
    }

    #[test]
    fn malformed_replies_do_not_discard_valid_background() {
        let mut bytes = &b"\x1b]11;rgb:22/24/36\x07\x1b]11;invalid\x07\x1b]4;99;rgb:f/f/f;1;bad;2;rgb:1/2/3\x07\x1b]10;invalid\x07\x1b[?6c"[..];
        let colors = read_colors(|buf| bytes.read(buf)).unwrap();
        assert_eq!(colors.background, Some((34, 36, 54)));
        assert_eq!(colors.foreground, None);
        assert_eq!(colors.ansi[1], None);
        assert_eq!(colors.ansi[2], Some((17, 34, 51)));
    }

    #[test]
    fn background_only_survives_timeout_but_no_background_does_not() {
        let mut bytes = &b"\x1b]11;rgb:0/0/0\x07"[..];
        let colors = read_colors(|buf| {
            if bytes.is_empty() {
                Err(io::ErrorKind::TimedOut.into())
            } else {
                bytes.read(buf)
            }
        })
        .unwrap();
        assert_eq!(colors.background, Some((0, 0, 0)));
        assert_eq!(
            read_colors(|_| Err(io::ErrorKind::TimedOut.into()))
                .unwrap_err()
                .kind(),
            io::ErrorKind::TimedOut
        );
        let mut da1 = &b"\x1b[?1;2c"[..];
        assert_eq!(
            read_colors(|buf| da1.read(buf)).unwrap_err().kind(),
            io::ErrorKind::Unsupported
        );
    }

    #[test]
    fn unsupported_terminals_are_not_queried() {
        for term in ["dumb", "Eterm", "screen", "screen.xterm-256color", ""] {
            assert!(!supported_term(Some(term)), "{term}");
        }
        for term in ["xterm-256color", "tmux-256color", "xterm-kitty"] {
            assert!(supported_term(Some(term)), "{term}");
        }
        assert_eq!(supported_term(None), cfg!(windows));
    }
}
