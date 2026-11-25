# Source copy of the formula published in github.com/cesRay/homebrew-tap.
# After tagging a release, run packaging/homebrew/bump.sh <version> and copy
# this file to Formula/visor.rb in the tap.
class Visor < Formula
  desc "Ranger-style file navigator with content and filename search"
  homepage "https://github.com/cesRay/visor"
  url "https://github.com/cesRay/visor/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "68311188f631914578816eae7a91a6d2cb37e554e0a9c07d3aa25c5d8f8cac2d"
  license "MIT"
  head "https://github.com/cesRay/visor.git", branch: "main"

  depends_on "rust" => :build
  depends_on "vim"

  def install
    system "cargo", "install", *std_cargo_args
  end

  def caveats
    <<~EOS
      visor needs a small shell function to change your directory on exit.
      Add the line for your shell to its config file:
        zsh  (~/.zshrc):                   eval "$(visor init zsh)"
        bash (~/.bashrc):                  eval "$(visor init bash)"
        fish (~/.config/fish/config.fish): visor init fish | source
    EOS
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/visor --version")
    assert_match "--cwd-file", shell_output("#{bin}/visor init zsh")
  end
end
