# lavasnek_rs

- Dev Docs: [Main Site](https://5124.mywire.org:5124/docs/) | Fallback: [GitHub Pages](https://vicky5124.github.io/lavasnek_rs/lavasnek_rs/lavasnek_rs.html)
- [GitHub repo](https://github.com/vicky5124/lavasnek_rs/)
- [GitLab repo](https://gitlab.com/vicky5124/lavasnek_rs/)

## Using the library

The library is available on PyPi, and you can install it via `pip install lavasnek_rs --pre -U --user`

To install a developement release of the library, go to the Actions tab on GitHub, select the latest commit,
and download the Artifact that suits your needs. Extract the artifact, and install the .whl file with
`pip install -U --user filename.whl`

Then you should be able to import the library and use it!

## Building and Running the hikari example

If you wanna build the project from source, (for contributing, compiling to a different architecture than
x86_64 or for python 3.10) you will need:

```bash
# It is highly recommended that you use Linux, either natively or with WSL2

# You will need python 3.6 or newer.
sudo apt install python3-pip curl # for Ubuntu, Debian or derivatives
sudo pacman -S python3-pip curl # for Arch, EndeavourOS, Manjaro or derivatives

# You will also need the rust programming language.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# You will also need a lavalink server running.
# see https://github.com/freyacodes/Lavalink or use docker
docker run --name lavalink -p 2333:2333 -d -v $HOME/application.yml:/opt/Lavalink/application.yml fredboat/lavalink:dev
```

Then to run the project, just run all of this.
Only the last 2 are repeatable.

```bash
python -m venv .env
source .env/bin/activate
pip install maturin

maturin develop
python examples/pure_hikari_basic_queue/bot.py
```
