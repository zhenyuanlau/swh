# Switch Hosts CLI

A CLI for Switch Hosts.

## SwitchHosts

Need to install [SwitchHosts](https://github.com/oldj/SwitchHosts), and turn on HTTP API by checking `Preferences -> General -> HTTP API on`.

## Examples

### Get Hosts List

```bash
$ swh list
╭────┬──────────────┬──────────────────────────────────────┬───────╮
│ #  │ title        │ id                                   │ on    │
╞════╪══════════════╪══════════════════════════════════════╪═══════╡
│ 0  │ development  │ 1ba37287-b3af-4359-b44d-bbf0f2953ee8 │ true  │
├────┼──────────────┼──────────────────────────────────────┼───────┤
│ 1  │ staging      │ 2675f67d-97d7-4689-a59a-4ea666eb97d1 │ false │
├────┼──────────────┼──────────────────────────────────────┼───────┤
│ 13 │ production   │ 48d52d0e-9f26-487c-a981-b45f46cdf9c5 │ false │
╰────┴──────────────┴──────────────────────────────────────┴───────╯
```

### Toggle Hosts

```bash
$ swh toggle 1ba37287-b3af-4359-b44d-bbf0f2953ee8

ok
```

## Roadmap

Using `swh` instead of SwitchHosts.

- [x] High-frequency operations
- [ ] Hosts CRUD
- [ ] Enhancements

## License

MIT.
