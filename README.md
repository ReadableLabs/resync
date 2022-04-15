### Resync

A multi language comment checker.

Resync uses the git history of your repo to detect out of sync comments. It's a CLI tool which works in multiple languages, and supports docstrings and inline comments.

The goal of resync is to be a fast, small, and simple comment checker. It uses other libraries for parsing, but the release build should try to be under 10mb.

Resync tells you if specific params/returns are out of sync, or if comments are stale.

#### Supported Languages
- [ ] Rust
- [ ] Javscript
- [ ] Typescript
- [ ] C
- [ ] C++
- [ ] C#
- [ ] Python
- [ ] PHP
- [ ] Kotlin
- [ ] Java


#### Project Structure
```
├── docs
├── src
│   └── parsers # See doc/parsers.md for more details
└── tests
```
[parsers.md](./doc/parsers.md)

### TODO
- Make work on base branch without sync (replace all 0's with current unix time) - sync will be used for Readable
