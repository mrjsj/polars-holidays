SHELL=/bin/bash

venv:
	python3 -m venv .venv && \
	source .venv/bin/activate && \
	pip install ".[dev]"

install:
	unset CONDA_PREFIX && \
	source .venv/bin/activate && \
	maturin develop

install-release:
	unset CONDA_PREFIX && \
	source .venv/bin/activate && \
	maturin develop --release

pre-commit:
	source .venv/bin/activate && \
	rustup run nightly cargo fmt --all && \
	cargo clippy --all-features && \
	ruff check . --fix --exit-non-zero-on-fix && \
	ruff format polars_holidays tests && \
	mypy polars_holidays tests

test:
	source .venv/bin/activate && \
	pytest tests/* -v

# run: install
# 	source .venv/bin/activate && \
# 	python run.py

# run-release: install-release
# 	source .venv/bin/activate && \
# 	python run.py

