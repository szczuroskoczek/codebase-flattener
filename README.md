# codebase-flattener

Flatten a codebase into a single file for easy LLM ingestion.

---

**Features:**

- Recursively walk your project, respecting `.gitignore`, `.llmignore`, and custom ignore rules.
- Skip the output file itself to avoid self-inclusion.
- Follow or ignore symlinks via a flag.
- Overridable output filename (default: `codebase.txt`).

---

## 📦 Installation

### From Rust (crates.io)

Ensure you have Rust and Cargo installed (edition 2021+):

```bash
# Install from crates.io\ ncargo install codebase-flattener
```

Or build locally:

```bash
git clone https://github.com/szczuroskoczek/codebase-flattener.git
cd codebase-flattener
cargo build --release
# The binary will be in target/release/
```

### From npm

You can also install via npm to use the published JavaScript CLI wrapper:

```bash
# Install globally
npm install -g codebase-flattener

# Or use npx without global install
npx codebase-flattener
```

---

## 🚀 Usage

Once installed (via Rust or npm), simply run:

```bash
codebase-flattener
```

Or with any combination of options:

```bash
# Add custom ignore files
codebase-flattener --ignore extra.ignore --ignore temp.ignore

# Disable all default ignore files
codebase-flattener --no-default-ignores --ignore only.ignore

# Don’t follow symlinks
codebase-flattener --no-follow

# Change the output filename
codebase-flattener --output project_dump.txt
```

All files are dumped into `codebase.txt` with separators:

```
<<< FILE: path/to/file.ext >>>
<file contents>

```

---

## ⚙️ Command-line Options

| Option                    | Description                                                  |
| ------------------------- | ------------------------------------------------------------ |
| `-i, --ignore <FILE>`     | Append a custom ignore filename (can be used multiple times) |
| `--no-default-ignores`    | Disable default `.gitignore` & `.llmignore` loading          |
| `-n, --no-follow`         | Don’t follow symbolic links                                  |
| `-o, --output <FILENAME>` | Set the output file name (default: `codebase.txt`)           |
| `-h, --help`              | Print help information                                       |
| `-V, --version`           | Print version info                                           |

---

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/foo`)
3. Commit your changes (`git commit -am "Add foo"`)
4. Push to the branch (`git push origin feature/foo`)
5. Open a pull request

We welcome improvements, bug fixes, and enhanced documentation!

---

## 📝 License

Licensed under MIT. See [LICENSE](LICENSE) for details.

---

Created by [Krystian Mikołajczyk](https://github.com/szczuroskoczek) | [Crates.io](https://crates.io/crates/codebase-flattener) | [npm](https://www.npmjs.com/package/codebase-flattener) | [GitHub](https://github.com/szczuroskoczek/codebase-flattener)
