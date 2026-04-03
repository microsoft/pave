class Pave < Formula
  desc "A cross-platform CLI tool for managing the PATH"
  homepage "https://github.com/microsoft/pave"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/microsoft/pave/releases/download/v#{version}/pave-aarch64-apple-darwin.tar.gz"
      sha256 "c74ffb814dfd24c392d9efccc7e3e2db57c4e63c879e8ac604f8deda7d3a0e42"
    end
    on_intel do
      url "https://github.com/microsoft/pave/releases/download/v#{version}/pave-x86_64-apple-darwin.tar.gz"
      sha256 "0f8b4e33fa24ac43f568530d1a64c468b90c23b747a870b6018531df599f6b9a"
    end
  end

   on_linux do
    on_arm do
      url "https://github.com/microsoft/pave/releases/download/v#{version}/pave-aarch64-unknown-linux-musl.tar.gz"
      sha256 "8d0529c4b9e39c86a1a7ec288ad841a4146f87924737b6347cb1f3251f42b518"
    end
    on_intel do
      url "https://github.com/microsoft/pave/releases/download/v#{version}/pave-x86_64-unknown-linux-musl.tar.gz"
      sha256 "dbb61af46a28d9a24091266725f070697d85b1623f08f18afafeea7a3c11c73f"
    end
  end

  def install
    bin.install "pave"
  end

  test do
    assert_match "pave", shell_output("#{bin}/pave --help")
  end
end
