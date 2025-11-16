class Kanttiinit < Formula
  desc "Command-line interface for browsing restaurant menus from Kanttiinit.fi"
  homepage "https://kanttiinit.fi"
  url "https://github.com/otahontas/kanttiinit-cli/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "" # Update this after creating a release
  license "MIT"
  head "https://github.com/otahontas/kanttiinit-cli.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "kanttiinit", shell_output("#{bin}/kanttiinit --version")
  end
end
