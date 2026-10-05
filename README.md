# izac

A small package manager for installing prebuilt binaries from a personal repository.

```
izac list              # show packages in the repository
izac install <name>    # install a package
izac installed         # show what is installed locally
izac upgrade           # upgrade installed packages to the repository version
```

## Packaging a binary

A package is a zstd-compressed tarball (`.tar.zst`) that is extracted as-is into
`~/.local/share/izac/packages/<name>/<version>/`. After extraction, izac expects the
executable at:

```
<install dir>/bin/<name>
```

and symlinks it to `~/.local/bin/<name>`.

Two rules follow from that:

1. **The executable must be at `bin/<name>` inside the archive**, and `<name>` must match
   the package `name` in the index exactly.
2. **No wrapping top-level directory.** The archive is extracted directly into the install
   directory, so `bin/` must be at the root of the archive, not `hello-world-0.1.0/bin/`.

### Example

```sh
NAME=hello-world
VERSION=0.1.0

mkdir -p staging/bin
cp target/release/$NAME staging/bin/$NAME
chmod +x staging/bin/$NAME

tar --zstd -cf $NAME-$VERSION-x86_64.tar.zst -C staging .
```

Check the layout before publishing:

```sh
tar --zstd -tf $NAME-$VERSION-x86_64.tar.zst
# ./
# ./bin/
# ./bin/hello-world
```

Anything else in the archive (libraries, data files, docs) is extracted alongside `bin/`
and lives in the same versioned directory. izac does not add it to any search path, so the
binary must find those files itself (for example relative to its own location).

### Binary requirements

- Built for the machine it will be installed on. The architecture is currently assumed to
  be `x86_64`; there is no architecture detection.
- Executable bit set (`chmod +x`) before archiving, since tar preserves it.
- Self-contained, or only dependent on libraries already present on the target system.
  izac has no dependency handling.

## Publishing to the repository

The repository is served from `http://vault:8090/repo/`. izac reads `repo/index.json`:

```json
{
  "packages": [
    {
      "name": "hello-world",
      "version": "0.1.0",
      "description": "This is a test package",
      "url": "http://vault:8090/repo/hello-world-0.1.0-x86_64.tar.zst"
    }
  ]
}
```

| Field         | Notes                                                                 |
| ------------- | --------------------------------------------------------------------- |
| `name`        | Package name and the binary name (`bin/<name>`). Used as the install key. |
| `version`     | Any string. Each version installs to its own directory.               |
| `description` | Shown by `izac list`.                                                 |
| `url`         | Direct download URL of the `.tar.zst`.                                |

To publish:

1. Upload the tarball to the server so the `url` resolves.
2. Add or update the package entry in `index.json`.

### Releasing a new version

Build and upload a new tarball, then change `version` and `url` in the existing entry
(don't add a second entry with the same `name`). Users get it with `izac upgrade`.

Any version that differs from the installed one counts as an upgrade, including a lower
one, so don't roll back by editing the index unless you want clients to downgrade.

## What izac does on install

1. Downloads the tarball to `/tmp/<name>-<version>-x86_64.tar.zst`.
2. Extracts it with `tar --zstd -xf` into `~/.local/share/izac/packages/<name>/<version>/`.
3. Symlinks `<install dir>/bin/<name>` to `~/.local/bin/<name>`, replacing an existing
   symlink there. A regular file at that path is left alone and the install stops.
4. Records `name` and `version` in `~/.local/share/izac/installed.json`.

Make sure `~/.local/bin` is on your `PATH`.

## Current limitations

- No checksum or signature verification; downloads are plain HTTP.
- No uninstall command, and old versions are left on disk after an upgrade.
- No dependency handling.
- Repository URL and architecture are hard-coded.
