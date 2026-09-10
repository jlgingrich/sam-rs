{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  # See full reference at https://devenv.sh/reference/options/
  languages.rust = {
    enable = true;
    channel = "nightly";
    components = [
      "rustc"
      "cargo"
      "clippy"
      "rust-analyzer"
    ];
  };

  packages = with pkgs; [
    bacon
    cargo-seek
    cargo-nextest
    cargo-generate
  ];
  scripts.watcher = {
    exec = ''
      watchexec -c -e rs \
      "cargo clippy && cargo test && cargo run"
    '';
    packages = [ pkgs.watchexec ];
  };

  enterShell = ''
    echo "Crates ready to update with 'cargo update'":
    cargo update -n
  '';
  git-hooks.hooks = {
    clippy.enable = true;
  };

}
