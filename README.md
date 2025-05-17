# Downloader

This project is a reimplementation of the core functionality of the [MiSTer Downloader](https://github.com/MiSTer-devel/Downloader_MiSTer) in Rust, delivered as a library.

### What functionality does it cover?

It should cover the "online importer" component which is the core of the tool.

It should NOT cover:
- Config files. Configuration should be provided to the library through its API, and not through an .ini file.
- MiSTer specific paths. Base path, system path, store path, pext paths, and forbidden paths should be provided as part of the configuration.
- Concrete loggers. Instead it should provide some reporting interface that could be used to customize logging.
- Migrations.
- Certs fixes.
- OS updates/changes.
