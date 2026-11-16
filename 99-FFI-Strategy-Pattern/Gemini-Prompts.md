# Maturin and uv Manual Setup Transcript

**User:**
mrust uv manually set up mathurin config

---

**Assistant:**
To manually set up a Rust and Python project using [Maturin](https://github.com) and [uv](https://docs.rs), you need to configure your `Cargo.toml`, `pyproject.toml`, and `.cargo/config.toml` files so that both `uv` and `cargo` use the same virtual environment and build backend.

### 1. Configure `Cargo.toml`
Set your crate type to include both `cdylib` (for the Python extension) and `rlib` (so Rust binaries or other libraries can use it):

```toml
[package]
name = "my_rust_project"
version = "0.1.0"
edition = "2021"

[lib]
name = "my_rust_project"
crate-type = ["cdylib", "rlib"]

[dependencies]
pyo3 = { version = "0.22", features = ["abi3-py38"] } # Adjust PyO3 version as needed
```

### 2. Configure `pyproject.toml`
Create a `pyproject.toml` file in your project root with the Maturin build-system backend and project metadata:

```toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "my_rust_project"
version = "0.1.0"
requires-python = ">=3.8"
dependencies = []

# Optional: Add extra Maturin-specific configuration
[tool.maturin]
# e.g., specify bindings or release profile
# bindings = "pyo3"
```

### 3. Configure `.cargo/config.toml` (Crucial for `uv` + PyO3)
When using `uv` without a global Python installation, `cargo run` can fail to link PyO3 because it searches for system Python instead of your `uv` virtual environment. 

Create a `.cargo/config.toml` file to point PyO3 directly to the `uv` managed virtual environment:

```toml
[env]
PYO3_PYTHON = { value = ".venv/bin/python", relative = true }
```
*(On Windows, adjust the path to `.venv\Scripts\python.exe` if necessary).*

### 4. Initialize and Build with `uv` and `maturin`
Run these commands in your project terminal:

1. **Add Maturin to your `uv` environment:**
   ```bash
   uv add maturin
   ```

2. **Develop and install the package locally into your `uv` virtual environment:**
   ```bash
   uv run maturin develop
   ```

3. **Test the installation:**
   ```bash
   uv run python -c "import my_rust_project; print(my_rust_project)"
   ```

---

**User:**

```
maturin failed
        Caused by: pyproject.toml at /home/thomas/Courses/Reviews/cs330-examples/99-FFI-Strategy-Pattern/Example-5/pyproject.toml is invalid
        Caused by: TOML parse error at line 5, column 11
        |
      5 | authors = {name = "Thomas J. Kennedy", email="tkennedy@cs.odu.edu"}
```

---

**Assistant:**
The error occurs because the **`authors` field in `pyproject.toml` must be a list of tables**, not a single inline table. 

In Python packaging standards (PEP 621), `authors` requires square brackets `[ ]` wrapping the curly braces `{ }`.

### How to Fix It
Update line 5 of your `pyproject.toml` file to wrap the entry in square brackets:

```toml
authors = [{ name = "Thomas J. Kennedy", email = "tkennedy@cs.odu.edu" }]
```

*(Note: The same rule applies to the `maintainers` field if you use it).*

### Next Steps
Once you save the file, re-run your build command:
```bash
uv run maturin develop
```
