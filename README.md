# lavasnek_rs

Dev Docs: https://5124.mywire.org:5124/docs/
GitHub repo: https://github.com/vicky5124/lavasnek_rs/
GitLab repo: https://gitlab.com/vicky5124/lavasnek_rs/

## Building and Running

```bash
# It is highly recommended that you use Linux, either natively or with WSL2

# You will need python 3.6 or newer.
sudo apt install python3-pip curl # for Ubuntu, Debian or derivatives
sudo pacman -S python3-pip curl # for Arch, EndeavourOS, Manjaro or derivatives

# You will also need rust with the nightly toolchain.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # to install rustlang
rustup toolchain install nightly # to install the nightly toolchain

# You will also need a lavalink server running.
# see https://github.com/freyacodes/Lavalink or use docker
docker run --name lavalink -p 2333:2333 -d -v $HOME/application.yml:/opt/Lavalink/application.yml fredboat/lavalink:dev
```

Then to run the project, just run all of this.
Only the last 2 are repeatable.

```bash
$ python -m venv .env
$ source .env/bin/activate
$ pip install maturin

$ maturin develop
$ python test.py
```
