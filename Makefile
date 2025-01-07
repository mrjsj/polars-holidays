SHELL=/bin/bash

venv:
	python3 -m venv .venv
	source .venv/bin/activate
	pip install ".[dev]"

install:
	unset CONDA_PREFIX && \
	source .venv/bin/activate && maturin develop

install-release:
	unset CONDA_PREFIX && \
	source .venv/bin/activate && maturin develop --release

pre-commit:
	rustup run nightly cargo fmt --all && cargo clippy --all-features
	.venv/bin/python -m ruff check . --fix --exit-non-zero-on-fix
	.venv/bin/python -m ruff format polars_holidays tests
	.venv/bin/mypy polars_holidays tests

test:
	pytest tests/* -v

run: install
	source .venv/bin/activate && python run.py

run-release: install-release
	source .venv/bin/activate && python run.py

