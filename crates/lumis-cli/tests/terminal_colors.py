import errno
import json
import os
from pathlib import Path
import pty
import select
import subprocess
import sys
import tempfile
import termios
import time


BINARY = sys.argv[1]
DATA_DIR = sys.argv[2]
CASES = Path(__file__).resolve().parents[3] / "fixtures/theme-choice-cases.json"
DA1 = b"\x1b[c"
SOURCE = b'fn main() { let greeting = "hello terminal"; }\n'


def response(case):
    def osc(command, color):
        rgb = "/".join(color[i:i + 2] * 2 for i in (1, 3, 5))
        return f"\x1b]{command};rgb:{rgb}\x07".encode()

    result = b""
    for command, key in [(10, "foreground"), (11, "background")]:
        if case.get(key):
            result += osc(command, case[key])
    for index, color in enumerate(case.get("ansi") or []):
        if 1 <= index <= 6 and color:
            result += osc(f"4;{index}", color)
    return result + b"\x1b[?1;2c"


def run(directory, case, *, extra=(), piped=False, reply=True, term="xterm-256color", verbose=True):
    master, slave = pty.openpty()
    before = termios.tcgetattr(slave)
    env = dict(os.environ, LUMIS_CONFIG=str(directory / "missing.toml"),
               LUMIS_DATA_DIR=DATA_DIR)
    if term is None:
        env.pop("TERM", None)
    else:
        env["TERM"] = term

    command = [BINARY, *(["-v"] if verbose else []), "highlight",
               str(directory / "source.txt"), "--language", "rust", *extra]
    process = subprocess.Popen(command, stdin=slave, stdout=subprocess.PIPE if piped else slave,
                               stderr=slave, env=env)
    received = bytearray()
    answered = False
    query_started = None
    started = time.monotonic()
    try:
        while True:
            if time.monotonic() - started > 30:
                raise AssertionError(f"hung: {case['name']}: {received!r}")
            ready, _, _ = select.select([master], [], [], 0.05)
            if ready:
                try:
                    chunk = os.read(master, 4096)
                except OSError as error:
                    if error.errno != errno.EIO:
                        raise
                    break
                if not chunk:
                    break
                received.extend(chunk)
                if DA1 in received and not answered:
                    answered = True
                    query_started = time.monotonic()
                    if reply:
                        os.write(master, response(case))
            elif process.poll() is not None:
                break
        stdout, _ = process.communicate(timeout=2)
        elapsed = None if query_started is None else time.monotonic() - query_started
        assert process.returncode == 0, (case["name"], received)
        after = termios.tcgetattr(slave)
        # macOS may set PENDIN after restoring canonical mode.
        after[3] &= ~getattr(termios, "PENDIN", 0)
        before[3] &= ~getattr(termios, "PENDIN", 0)
        assert after == before, ("raw mode was not restored", before, after, received)
        if answered:
            assert received.count(DA1) == 1, received
            for query in [b"10;?", b"11;?", *(f"4;{i};?".encode() for i in range(1, 7))]:
                assert received.count(query) == 1, (query, received)
        return bytes(received), stdout, answered, elapsed
    finally:
        if process.poll() is None:
            process.kill()
            process.wait()
        os.close(master)
        os.close(slave)


def main():
    cases = json.loads(CASES.read_text())
    assert len(cases) >= 20
    with tempfile.TemporaryDirectory(prefix="lumis-theme-choice-") as path:
        directory = Path(path)
        (directory / "source.txt").write_bytes(SOURCE)
        for case in cases:
            output, _, queried, _ = run(directory, case)
            assert queried, (case["name"], output)
            if case["expected"] is None:
                assert b"theme: auto unavailable (" in output, (case["name"], output)
                assert SOURCE.replace(b"\n", b"\r\n") in output, output
                assert b"\x1b[38;2;" not in output, output
            else:
                expected = f"theme: {case['expected']}\r\n".encode()
                assert expected in output, (case["name"], output)
                assert b"\x1b[38;2;" in output, output

        case = cases[0]
        for extra in [(), ("--theme", "auto")]:
            output, stdout, queried, _ = run(directory, case, piped=True, extra=extra, verbose=False)
            assert not queried and b"stdout is not a terminal" in output, output
            assert stdout == SOURCE, stdout

        output, _, queried, _ = run(directory, case, extra=("--formatter", "html-inline"), verbose=False)
        assert not queried and b"only available for terminal output" in output, output
        assert b"<pre" in output and b"\x1b[38;2;" not in output, output

        for extra, marker in [(("--theme", "dracula"), b"theme: dracula"),
                              (("--theme", "mfd_blackout"), b"theme: mfd_blackout")]:
            output, _, queried, _ = run(directory, case, extra=extra)
            assert not queried and marker in output, output

        output, stdout, queried, _ = run(directory, case, piped=True, extra=("--theme", "dracula"))
        assert not queried and b"theme: dracula" in output, output
        assert b"\x1b[38;2;" in stdout, stdout

        for term in [None, "dumb", "screen", "screen.xterm-256color", "Eterm"]:
            output, _, queried, _ = run(directory, case, term=term, verbose=False)
            assert not queried and b"does not support color queries" in output, output

        output, _, queried, elapsed = run(directory, case, reply=False, verbose=False)
        assert queried and b"theme: auto unavailable (" in output, output
        assert 0.9 <= elapsed < 2.5, elapsed
    print(f"{len(cases)} fixture cases and terminal fallback/query checks passed")


if __name__ == "__main__":
    main()
