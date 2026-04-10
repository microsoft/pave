class Pave < Formula
  desc "A cross-platform CLI tool for managing the PATH"
  homepage "https://github.com/microsoft/pave"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/microsoft/pave/releases/download/v#{version}/pave-aarch64-apple-darwin.tar.gz"
      sha256 "7d3b760231efa7a0415cb7c48b8702cdf3ec4094934fb5d3bad1400e3cd81382"
    end
    on_intel do
      url "https://github.com/microsoft/pave/releases/download/v#{version}/pave-x86_64-apple-darwin.tar.gz"
      sha256 "60c778a636e0d52cd9cbf854bb563c7657dde2c1b453cc31119b21929959675c"
    end
  end

   on_linux do
    on_arm do
      url "https://github.com/microsoft/pave/releases/download/v#{version}/pave-aarch64-unknown-linux-musl.tar.gz"
      sha256 "c81b13ff54cffc46f260b48431276c90e10ec5a2d9e0179602ff1d84cbb2208c"
    end
    on_intel do
      url "https://github.com/microsoft/pave/releases/download/v#{version}/pave-x86_64-unknown-linux-musl.tar.gz"
      sha256 "17e4380c27006ff12f97107b5a34acb4e980a61a0bfb36688fa8f1327b358cf4"
    end
  end

  def install
    bin.install "pave"
  end

  test do
    assert_match "pave", shell_output("#{bin}/pave --help")
  end
end
