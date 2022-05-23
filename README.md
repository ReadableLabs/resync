## Resync

<!-- ![1]() -->

Resync is a multi language comment checker written in rust. It's a CLI tool which uses the git history of your repo to detect out of sync comments.

Resync looks at things such as the commit dates, and commit consistency to tell if a comment is stale or not. While resync may not be 100% accurate at identifying out of sync comments, it tries to only show you places where the function has been updated a lot, but the comment hasn't.

![video showing resync](1.png)

There's also a vscode extension which supports resync, showing you all out of sync comments while you're coding.
![2]()

[You can download it here](https://github.com/ReadableLabs/resync)

### Download

- Vscode

### Supported Languages

- [x] Rust
- [ ] Javscript
- [ ] Typescript
- [ ] C
- [ ] C++
- [ ] C#
- [ ] Python
- [ ] PHP
- [ ] Kotlin
- [ ] Java

### Docs

### TODO

- Use [cursive](https://github.com/gyscos/cursive) to make a terminal GUI for resync
- use optional dependencies if you only want to compile for a few languages
- Allow resync to be used on the master branch, without creating resync branch
- Make async for faster querying
