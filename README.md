# Switch Hosts CLI

A CLI for Switch Hosts.

## Usage

### Config hosts in single file

```shell
$ swh open # default: ~/.config/swh/config.kdl
```

Config file example, syntax in [KDL](https://kdl.dev/):

```kdl

// ~/.config/swh/config.kdl

version "1.0"

env "local" enabled=true {

  host "127.0.0.1" name="localhost" {
    alias "localhost.domain"
  }

  host "255.255.255.255" name="broadcast.host"

}

include "dev" // support in the future

```

### SWH CLI

```bash

$ swh list

╭────┬──────────────┬──────────╮
│ #  │ env          │ enabled  │
╞════╪══════════════╪══════════╡
│ 0  │ development  │ true     │
├────┼──────────────┼──────────┤
│ 1  │ staging      │ false    │
├────┼──────────────┼──────────┤
│ 2  │ production   │ false    │
╰────┴──────────────┴──────────╯
```

```bash
$ swh toggle production
```

```bash
$ swh show production
```

## Roadmap

Using `swh` instead of SwitchHosts.

- [x] SWH CLI
- [x] Host config in single file
- [ ] Host config in multiple files
- [ ] Host CRUD in CLI
- [ ] More features
- [ ] SWH HTTP API

## License

MIT.
