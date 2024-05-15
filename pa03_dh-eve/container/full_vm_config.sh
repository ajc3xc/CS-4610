#!/bin/bash

sudo dnf install -y ShellCheck bash-completion bat cargo cflow cgdb clang clang-tools-extra cmake cppcheck cscope diffutils docker docker-compose doxygen doxygen-doxywizard eza file fish gcc gcc-g++ gcovr gdb git gnupg2 graphviz htop httpd java-latest-openjdk llvm ltrace lynx make mc meld micro mtr nano nasm nc ncurses nmap octave openjfx openssl-devel openssh-server pandoc poetry python3-black python3-devel python3-docopt python3-ipython python3-jedi python3-jupyter-core python3-matplotlib python3-mypy python3-numpy python3-pandas python3-pip python3-poetry python3-pudb python3-pygments python3-scapy python3-scikit* python3-scipy python3-seaborn python3-spyder qemu-* qtcreator ranger rr rust rust-debugger-common rust-doc rust-gdb rust-lldb rust-src rustfmt rustup shfmt strace tmux traceroute valgrind vim vim-X11 vim-enhanced vim-filesystem vim-fugitive vim-jedi vim-nerdtree vim-powerline vim-syntastic* vis wget whois wireshark x-tile yasm zsh

sudo dnf group install -y "Security Lab"

# Note: scikit-bio conflicts with scipy version on python 3.12 or newer.
pip3 install --upgrade assigner ete3 mediapy monkeytype mujoco py2cfg python-Levenshtein scikit-bio --user

# Doing this with --force because Jupytext (improperly) does not see system python:
pip3 install --upgrade --force jupyter jupyterlab notebook jupytext --user

cargo install nu

mkdir ~/bin
fish -c "fish_add_path --prepend bin/ .local/bin/ .cargo/bin/"

wget https://raw.githubusercontent.com/cyrus-and/gdb-dashboard/master/.gdbinit

# Modify this so that user is your own username:
sudo usermod --shell /usr/bin/fish user
