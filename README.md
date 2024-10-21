# Redmine Command Line Client

A command line client to for redmine to access issues [tickets], download, upload files and add notes to the issues.

I've developed this client on my vacation as a practice to learn RUST and write API clients. The initial version was written in 2021 Summer.

Due to workload, I've stopped adding new features and bug-fixes.

We've been using the tool for two years actively and works fine. Would like to share with the community so it could be improved further and updated.

Rust libraries has matured and sure some parts could be refactored using libraries.

### Notes For Compiling

The client app connects via the Redmine API, it stores the API key and the adress on a config file. The config files is encrypted with a magic key with salted mac adress.

So while compiling please edit the magic key on sessions.rs file. Or write your encrypt and decrypt functions.

### Bugs

While downloading attachment files, it does download by checking if the filename partially matches to the full attachment names and downloads by overwriting. If there are two attecments with same name, it'll overwrite the same file.
