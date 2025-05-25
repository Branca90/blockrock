# Zion Core\n\nPrototipo di un nodo P2P basato su Rust e libp2p.\n\n## Requisiti\n- Rust (ultima versione stabile)\n- Rust's package manager

Usage: cargo [+toolchain] [OPTIONS] [COMMAND]
       cargo [+toolchain] [OPTIONS] -Zscript <MANIFEST_RS> [ARGS]...

Options:
  -V, --version
          Print version info and exit
      --list
          List installed commands
      --explain <CODE>
          Provide a detailed explanation of a rustc error message
  -v, --verbose...
          Use verbose output (-vv very verbose/build.rs output)
  -q, --quiet
          Do not print cargo log messages
      --color <WHEN>
          Coloring [possible values: auto, always, never]
  -C <DIRECTORY>
          Change to DIRECTORY before doing anything (nightly-only)
      --locked
          Assert that `Cargo.lock` will remain unchanged
      --offline
          Run without accessing the network
      --frozen
          Equivalent to specifying both --locked and --offline
      --config <KEY=VALUE|PATH>
          Override a configuration value
  -Z <FLAG>
          Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for
          details
  -h, --help
          Print help

Commands:
    build, b    Compile the current package
    check, c    Analyze the current package and report errors, but don't build object files
    clean       Remove the target directory
    doc, d      Build this package's and its dependencies' documentation
    new         Create a new cargo package
    init        Create a new cargo package in an existing directory
    add         Add dependencies to a manifest file
    remove      Remove dependencies from a manifest file
    run, r      Run a binary or example of the local package
    test, t     Run the tests
    bench       Run the benchmarks
    update      Update dependencies listed in Cargo.lock
    search      Search registry for crates
    publish     Package and upload this package to the registry
    install     Install a Rust binary
    uninstall   Uninstall a Rust binary
    ...         See all commands with --list

See 'cargo help <command>' for more information on a specific command. per compilazione\n\n## Esecuzione\n```bash\nROCKET_PORT=8000 cargo run\nROCKET_PORT=8001 cargo run\n```\n\n## Endpoint\n- : Verifica lo stato del nodo.\n- : Restituisce il .\n- : Elenca i peer connessi.\n\n## Test\n```bash\ncurl http://127.0.0.1:8000/health\ncurl http://127.0.0.1:8000/p2p/status\ncurl http://127.0.0.1:8000/p2p/peers\n```
