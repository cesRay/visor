/// Wrapper for zsh and bash. It runs visor with a temp file, then cds into
/// the directory visor wrote there (nothing is written when the user presses Esc).
pub const POSIX_INIT: &str = r#"visor() {
  local tmp dir code
  tmp="$(command mktemp -t visor-cwd.XXXXXX)" || return 1
  command visor --cwd-file "$tmp" "$@"
  code=$?
  dir="$(command cat -- "$tmp" 2>/dev/null)"
  command rm -f -- "$tmp"
  if [ -n "$dir" ] && [ -d "$dir" ] && [ "$dir" != "$PWD" ]; then
    builtin cd -- "$dir" || return 1
  fi
  return $code
}
"#;

/// Same wrapper for fish
pub const FISH_INIT: &str = r#"function visor
    set -l tmp (command mktemp -t visor-cwd.XXXXXX); or return 1
    command visor --cwd-file $tmp $argv
    set -l code $status
    set -l dir (command cat -- $tmp 2>/dev/null)
    command rm -f -- $tmp
    if test -n "$dir"; and test -d "$dir"; and test "$dir" != "$PWD"
        builtin cd -- $dir; or return 1
    end
    return $code
end
"#;

/// Printed when the user leaves on a directory but the wrapper is not installed
pub const SETUP_HINT: &str =
    "visor: cannot change your shell's directory without shell integration.
Add this line to ~/.zshrc (or use `visor init bash` / `visor init fish`):
  eval \"$(visor init zsh)\"";
