# shell.nix

let
    pkgs = import <nixpkgs> {};

in pkgs.mkShell {
    buildInputs = [
        pkgs.bash
        pkgs.coreutils
    ];
    shellHook = ''
        unset REPO_URL
        unset GITHUB_TOKEN
        unset RUNNER_TOKEN
        unset RUNNER_VERSION
        unset NGROK_AUTHTOKEN
        unset NGROK_PORT
        gh auth status
    '';
}