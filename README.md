
# Rust vs Python: Performance & Resource Usage Comparison

This repository provides a comparison between a data processing script written in Python and its counterpart written in Rust. The primary focus is to highlight the improvements in speed and resource usage when using Rust.

## Overview

- **Original script:** A data processing script for Spotify data using pandas and seaborn in Python.
- **Rust counterpart:** Utilizes `rust-csv`, `polars`, and `plotters` for similar data processing and visualization tasks.

## Installation & Usage

### Python

**Requirements:**
- Python 3.x
- pandas
- seaborn

```bash
pip install pandas seaborn
python script_name.py
```

### Rust

**Requirements:**
- Rust and Cargo

```bash
cargo build --release
cargo run --release
```

## Performance & Resource Usage

Both versions of the script were tested using the same dataset. A detailed performance report can be found [here](#).

**Highlights:**
- **Execution Time:** Rust version shows a X% improvement over the Python version.
- **Memory Usage:** Rust version uses Y MB less memory on average during execution.

> Note: Replace X and Y with actual performance improvement percentages and memory usage differences.

## Conclusion

This project demonstrates the potential benefits of using Rust for data processing tasks, especially in terms of speed and resource efficiency. While Python offers ease of use and a vast ecosystem for data science, Rust provides significant performance gains, making it an excellent choice for more resource-intensive tasks.
