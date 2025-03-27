from setuptools import setup

setup(
    name="terminal_executor_python",
    version="0.1.0",
    packages=["terminal_executor"],
    entry_points={
        "console_scripts": [
            "terminalExecutor=terminal_executor.main:main",
        ],
    }
)
