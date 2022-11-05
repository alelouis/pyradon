# **pyradon**
![rustc](https://img.shields.io/badge/rustc-1.61.0-important)
![rustc](https://img.shields.io/badge/python-3.9-normal)

Python bindings for Radon transform rust implementation ([described here](https://github.com/alelouis/radon-transform)).  
Binding are handled by [PyO3](https://github.com/PyO3/pyo3).

## **How to install**
Clone the repository and create a Python virtual environment with [`maturin`](https://github.com/PyO3/maturin) dependency).
```bash
gh clone alelouis/pyradon
conda create -n env_name python=3.9
pip install maturin
```
Then build the library and generate the wheel using `maturin`.
```bash
maturin build --release
```
Finally, install the built wheel in `target/wheels`. For example on arm64 macosx:
```bash
pip install target/wheels/pyradon-0.1.0-cp39-cp39-macosx_11_0_arm64.whl
```

## **How to use**
Then import the module `pyradon` and use the `transform` method from it.
```python
import pyradon
radon_transform = pyradon.transform(...)
```

## **Example**
The notebook `test.ipynb` shows an example of the module on a Shepp-Logan phantom.