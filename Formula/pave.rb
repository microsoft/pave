class Pave < Formula
  desc "A cross-platform CLI tool for managing the PATH"
  homepage "https://github.com/microsoft/pave"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/microsoft/pave/releases/download/v#{version}/pave-aarch64-apple-darwin.tar.gz"
      sha256 "0019dfc4b32d63c1392aa264aed2253c1e0c2fb09216f8e2cc269bbfb8bb49b5"
    end
    on_intel do
      url "https://github.com/microsoft/pave/releases/download/v#{version}/pave-x86_64-apple-darwin.tar.gz"
      sha256 "0019dfc4b32d63c1392aa264aed2253c1e0c2fb09216f8e2cc269bbfb8bb49b5"
    end
  end

   on_linux do
    on_arm do
      url "https://github.com/microsoft/pave/releases/download/v#{version}/pave-aarch64-unknown-linux-musl.tar.gz"
      sha256 "0019dfc4b32d63c1392aa264aed2253c1e0c2fb09216f8e2cc269bbfb8bb49b5"
    end
    on_intel do
      url "https://github.com/microsoft/pave/releases/download/v#{version}/pave-x86_64-unknown-linux-musl.tar.gz"
      sha256 "0019dfc4b32d63c1392aa264aed2253c1e0c2fb09216f8e2cc269bbfb8bb49b5"
    end
  end

  def install
    bin.install "pave"
  end

  test do
    assert_match "pave", shell_output("#{bin}/pave --help")
  end
end
