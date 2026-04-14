class Pave < Formula
  desc "A cross-platform CLI tool for managing the PATH"
  homepage "https://github.com/microsoft/pave"
  version "0.1.1"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/microsoft/pave/releases/download/v#{version}/pave-aarch64-apple-darwin.tar.gz"
      sha256 "271d2182d16ea700ad93e089bc7c36c9878c2d47a64ce3d617e16942468e3451"
    end
    on_intel do
      url "https://github.com/microsoft/pave/releases/download/v#{version}/pave-x86_64-apple-darwin.tar.gz"
      sha256 "d218ac20dfadda0655ca22938a7cd56bf60e7112c78e96f6e0b995964f63ee59"
    end
  end

   on_linux do
    on_arm do
      url "https://github.com/microsoft/pave/releases/download/v#{version}/pave-aarch64-unknown-linux-musl.tar.gz"
      sha256 "a4efe9980b408c9c1b249635c1bb55fcd920e254aa2a3c8e39522dd39e5f6430"
    end
    on_intel do
      url "https://github.com/microsoft/pave/releases/download/v#{version}/pave-x86_64-unknown-linux-musl.tar.gz"
      sha256 "f1d0efb2bee210c974b365ca406777a5978c5cedb1f4eecded364a0e848281dd"
    end
  end

  def install
    bin.install "pave"
  end

  test do
    assert_match "pave", shell_output("#{bin}/pave --help")
  end
end
