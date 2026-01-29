# Contributing to hypricer

First off, thank you for considering contributing to hypricer! It's people like you that make hypricer a great tool for the community.

---

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [How Can I Contribute?](#how-can-i-contribute)
4. [Development Setup](#development-setup)
5. [Pull Request Process](#pull-request-process)
6. [Style Guidelines](#style-guidelines)
7. [Community](#community)

---

## Code of Conduct

This project adheres to a Code of Conduct that we expect all contributors to follow:

### Our Pledge

- **Be respectful** - Treat everyone with respect
- **Be collaborative** - Work together towards common goals
- **Be patient** - Remember that everyone was a beginner once
- **Be constructive** - Provide helpful feedback

### Unacceptable Behavior

- Harassment or discrimination of any kind
- Trolling, insulting/derogatory comments
- Public or private harassment
- Publishing others' private information

**Enforcement:** Violations may result in temporary or permanent ban from the project.

---

## Getting Started

### Types of Contributions We're Looking For

- 🐛 **Bug reports** - Found something broken?
- ✨ **Feature requests** - Have an idea?
- 📚 **Documentation** - Noticed a typo or unclear explanation?
- 🎨 **Themes** - Created a cool theme to share?
- 🔧 **Registry items** - New watchers, providers, or components?
- 💻 **Core code** - Improvements to the engine itself?

**Not sure where to start?** Check out issues labeled [`good first issue`](https://github.com/yourusername/hypricer/labels/good%20first%20issue) or [`help wanted`](https://github.com/yourusername/hypricer/labels/help%20wanted).

---

## How Can I Contribute?

### Reporting Bugs

**Before submitting:**
1. Check if it's already reported in [Issues](https://github.com/yourusername/hypricer/issues)
2. Test with the latest version
3. Try with a minimal theme (like `modern_dark`)

**Bug Report Template:**
```markdown
**System Information:**
- OS: Arch Linux (kernel 6.7.1)
- Hyprland version: 0.35.0
- hypricer version: commit abc1234
- Rust version: 1.75.0

**Describe the Bug:**
A clear description of what's broken.

**To Reproduce:**
Steps to reproduce:
1. Clone repo
2. Run `hypricer build --profile seiki`
3. See error

**Expected Behavior:**
What should happen instead?

**Logs:**
```
[Paste relevant logs from live/daemon.log]
```

**Additional Context:**
Any other info (screenshots, config files, etc.)
```

---

### Suggesting Features

**Before suggesting:**
1. Check [existing feature requests](https://github.com/yourusername/hypricer/labels/enhancement)
2. Consider if it fits hypricer's scope (Hyprland theming)

**Feature Request Template:**
```markdown
**Is your feature request related to a problem?**
A clear description of the problem. Ex. "I'm always frustrated when..."

**Describe the solution you'd like:**
What you want to happen.

**Describe alternatives you've considered:**
Other solutions or workarounds you thought about.

**Additional context:**
Mockups, examples, or use cases.
```

---

### Contributing Themes

Themes are always welcome!

**Process:**
1. Create theme in `themes/yourtheme/`
2. Test thoroughly
3. Document in `themes/yourtheme/README.md`
4. Submit PR

**Theme Checklist:**
- [ ] Works with fresh hypricer installation
- [ ] All dependencies documented
- [ ] Includes README with screenshots
- [ ] Follows naming conventions (`theme_name`, not `theme-name`)
- [ ] No hardcoded personal paths (use `~` or variables)

See [Theme Developer Guide](docs/theme-developer-guide.md) for details.

---

### Contributing Registry Items

New watchers and providers expand what themes can do!

**Guidelines:**
- **Modularity**: One file per category (e.g., `hardware.toml`, `network.toml`)
- **Documentation**: Add comments explaining what it does
- **Dependencies**: Always include `check` fields
- **Safe defaults**: Providers must have sensible fallbacks

**Example:**
```toml
# catalog/registry/network.toml

# Watcher: wifi_ssid
# Returns: Current WiFi network name or "disconnected"
[watcher.wifi_ssid]
provider = "poll_cmd"
interval = 5000
cmd = "iwgetid -r || echo 'disconnected'"
check = "which iwgetid"

# Provider: public_ip
# Returns: Current public IP address
[provider.public_ip]
cmd = "curl -s ifconfig.me"
default = "offline"
check = "which curl"
```

---

### Contributing Code

Want to improve the core engine?

**Areas needing help:**
- [ ] `stream_cmd` provider implementation
- [ ] File system watchers (inotify)
- [ ] DBus signal watchers
- [ ] Web UI for theme management
- [ ] Better error messages
- [ ] Unit tests

**See [Development Setup](#development-setup) below.**

---

## Development Setup

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dev tools
cargo install cargo-watch    # Auto-recompile on changes
cargo install cargo-clippy  # Linter

# Clone your fork
git clone https://github.com/yourfork/hypricer
cd hypricer
```

### Building from Source

```bash
# Build CLI
cargo build --release

# Run tests
cargo test

# Lint
cargo clippy -- -D warnings

# Format code
cargo fmt
```

### Development Workflow

**1. Make changes to CLI:**
```bash
# Edit src/*.rs
nvim src/generator.rs

# Test immediately
cargo run -- build --profile seiki
```

**2. Test generated daemon:**
```bash
# Inspect generated code
cat generated/source/src/main.rs

# Compile it
cd generated/source
cargo build --release

# Run manually to see output
./target/release/hrm_daemon
```

**3. Auto-rebuild on changes:**
```bash
# Terminal 1: Watch CLI
cargo watch -x 'run -- build --profile seiki'

# Terminal 2: Watch daemon
cd generated/source
cargo watch -x 'build --release'
```

### Project Structure

```
hypricer/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── registry.rs      # Registry loader
│   ├── generator.rs     # Daemon code generator
│   └── structs.rs       # Data structures
│
├── catalog/
│   ├── registry/        # Component definitions
│   └── static/          # Shared configs
│
├── themes/              # Theme packages
│   ├── seiki/
│   └── modern_dark/
│
├── docs/                # Documentation
│   ├── architecture.md
│   ├── user-guide.md
│   └── ...
│
└── tests/               # (Future) Integration tests
```

### Running Tests

```bash
# Unit tests
cargo test

# Integration tests (future)
cargo test --test integration

# Test a specific module
cargo test registry::tests
```

---

## Pull Request Process

### Before Submitting

**Checklist:**
- [ ] Code compiles (`cargo build`)
- [ ] Tests pass (`cargo test`)
- [ ] Linter is happy (`cargo clippy`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Documentation updated (if needed)
- [ ] CHANGELOG.md updated (if user-facing change)

### PR Template

```markdown
## Description
Brief description of what this PR does.

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to change)
- [ ] Documentation update

## Testing
How did you test this?

- [ ] Tested with `modern_dark` theme
- [ ] Tested with `seiki` theme  
- [ ] Manual testing
- [ ] Added unit tests

## Checklist
- [ ] My code follows the project's style guidelines
- [ ] I have performed a self-review
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have updated the documentation
- [ ] My changes generate no new warnings
```

### Review Process

1. **Automated checks** run (CI/CD) - must pass
2. **Maintainer review** - may request changes
3. **Approval** - PR is merged
4. **Release** - Included in next version

**Timeline:** Reviews typically within 3-5 days.

---

## Style Guidelines

### Rust Code Style

**Follow standard Rust conventions:**
```bash
# Format automatically
cargo fmt

# Check for common mistakes
cargo clippy
```

**Specific guidelines:**

```rust
// ✅ Good: Descriptive names
fn validate_requirements(registry: &Registry) -> Result<()> {
    // ...
}

// ❌ Bad: Cryptic names
fn val_req(r: &Reg) -> Res {
    // ...
}

// ✅ Good: Error context
fs::read_to_string(&path)
    .with_context(|| format!("Failed to read theme: {:?}", path))?

// ❌ Bad: Generic error
fs::read_to_string(&path)?

// ✅ Good: Comments for complex logic
// Parse CPU usage, handling both integer and decimal formats
let cpu = output.trim()
    .split('.')
    .next()
    .and_then(|s| s.parse().ok())
    .unwrap_or(0);

// ❌ Bad: No explanation
let cpu = output.trim().split('.').next().and_then(|s| s.parse().ok()).unwrap_or(0);
```

### TOML Style

```toml
# ✅ Good: Sections with headers
###############################################################################
# SYSTEM WATCHERS
###############################################################################

[watcher.cpu_usage]
provider = "poll_cmd"
interval = 2000
cmd = "top -bn1 | awk '{print $2}'"

# ❌ Bad: No organization
[watcher.cpu_usage]
provider = "poll_cmd"
interval = 2000
cmd = "top -bn1 | awk '{print $2}'"
[watcher.memory]
provider = "poll_cmd"
# ...
```

### Documentation Style

- **Use examples** - Show, don't just tell
- **Be concise** - Respect the reader's time
- **Be complete** - But link to other docs instead of repeating
- **Use code blocks** - Syntax highlighting helps
- **Update when code changes** - Outdated docs are worse than no docs

```markdown
<!-- ✅ Good: Clear, with example -->
## Installing hypricer

Clone and build:
```bash
git clone https://github.com/you/hypricer
cd hypricer
cargo build --release
```

<!-- ❌ Bad: Vague -->
## Installing hypricer
You need to get the code and compile it.
```

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```bash
# Format: <type>(<scope>): <subject>

# Types:
feat:     # New feature
fix:      # Bug fix
docs:     # Documentation only
style:    # Code style (formatting, etc)
refactor: # Code change that neither fixes a bug nor adds a feature
test:     # Adding tests
chore:    # Maintenance (dependencies, build, etc)

# Examples:
feat(registry): add network watchers
fix(generator): escape shell commands properly
docs(readme): add installation video link
refactor(cli): simplify argument parsing
```

**Good commits:**
```
feat(watchers): add stream_cmd provider type

Implements real-time command streaming for watchers like
`playerctl --follow`. This enables music metadata updates
without polling.

Closes #42
```

**Bad commits:**
```
fixed stuff
WIP
asdf
update
```

---

## Community

### Communication Channels

- **GitHub Issues** - Bug reports, feature requests
- **GitHub Discussions** - General questions, ideas
- **Discord** - Real-time chat (coming soon)
- **Reddit** - r/hypricer (coming soon)

### Getting Help

**Stuck on something?**
- Check [docs/](docs/) first
- Search [closed issues](https://github.com/yourusername/hypricer/issues?q=is%3Aissue+is%3Aclosed)
- Ask in [Discussions](https://github.com/yourusername/hypricer/discussions)

**Want to help others?**
- Answer questions in Discussions
- Review pull requests
- Improve documentation

---

## Recognition

Contributors are listed in:
- [CONTRIBUTORS.md](CONTRIBUTORS.md)
- GitHub's contributor graph
- Release notes (for significant contributions)

---

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

## Questions?

Feel free to ask in [Discussions](https://github.com/yourusername/hypricer/discussions) or reach out to maintainers.

**Thank you for contributing! 🎉**
