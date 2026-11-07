; This file is auto-generated. Do not edit.
((comment) @injection.content
  (#set! injection.language "comment")
  (#match? @injection.content "TODO|WIP|NOTE|XXX|INFO|DOCS|PERF|TEST|HACK|WARN|FIX|BUG|ERROR|#[0-9]+|https?://"))

((shell_command
  (shell_fragment) @injection.content)
  (#set! injection.language "bash")
  (#set! injection.combined))

((run_instruction
  (heredoc_block) @injection.content)
  (#set! injection.language "bash")
  (#set! injection.include-children))