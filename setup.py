from setuptools import setup
from setuptools_rust import Binding, RustExtension, Strip

setup_requires = ["setuptools-rust>=0.9.2"]
install_requires = []

setup(
    name="lavasnek_rs",
    author="vicky5124 <vickyf5124@gmail.com>",
    version="0.1.0-alpha.3",
    description="A lavalink-rs wrapper for any python async library",
    long_description=open("README.md", encoding="utf-8").read(),
    long_description_content_type="text/markdown",
    license="MPL-2.0",
    url="https://github.com/vicky5124/lavasnek_rs",
    project_urls={
        "Repository": "https://github.com/vicky5124/lavasnek_rs",
        "Issue tracker": "https://github.com/vicky5124/lavasnek_rs/issues",
        "Chat": "https://discord.gg/Jx4cNGG",
    },
    classifiers=[],
    rust_extensions=[RustExtension("lavasnek_rs.lavasnek_rs", "Cargo.toml", binding=Binding.PyO3, strip=Strip.Debug)],
    setup_requires=setup_requires,
    include_package_data=True,
    packages=["lavasnek_rs"],
    zip_safe=False,
    python_requires=">=3.6",
)
