# Todo

  0. Clean up codebase (e.g., remove commented-out code)
  1. Identify which structs and functions need to be modified
  2. Identify which structs will be wrapped in proxies/facades/adapters (e.g., Board)
  3. Identify which Python dunder functions to support (e.g., `__getitem__`)
  4. Support Move Strategies written in Python
    a. Replace `Box<...>` with a `MonoStrategy` wrapper
    b. Add a PyStrategy variant to that

# Need

 - `PyPlayer`
     - `PyStrategy`
 - `PyGame`
 - `PyBoard` - readonly
 - Eliminate/replace `'a` and `'game` lifetimes where reasonable alternatives
   exist
 - Debug output for Python `__repr__` (new trait? `PyDunderRepr`?)


# Notes

  - <https://pyo3.rs/main/doc/pyo3/types/trait.pyanymethods>
  - <https://pyo3.rs/main/doc/pyo3/macro.intern>
  - <https://pyo3.rs/main/doc/pyo3/struct.py>
