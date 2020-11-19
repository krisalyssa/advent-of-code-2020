# advent-of-code-2020

My participation in the [Advent of Code](https://adventofcode.com/) for [2020](https://adventofcode.com/2020/).

Please feel free to browse through my solutions. Be warned that there may be spoilers if you haven't submitted your own answers already.

## Setting up the sandbox

These instructions are probably over-complicated. `docker create` instead of `docker run -it` followed by `exit` should be possible.

```shell
$ docker build -t advent-of-code-2020-rust .

$ docker run -it --name advent-of-code-2020 --mount type=bind,src=$HOME/Projects/advent-of-code/2020,dst=/advent-of-code/2020 advent-of-code-2020-rust
# exit
$ docker start advent-of-code-2020
$ docker exec -it advent-of-code-2020 zsh
#
```

## Connecting VS Code to the sandbox

1. Click on the whale icon in the sidebar.
2. Right-click on the running container and choose "Attach Visual Studio Code". A new VS Code window will open.
3. Click "Open Folder". Navigate to `/advent-of-code/2020` then click "OK".
4. Install the `rust-lang.rust` extension in the container. If asked to install additional Rust components, click "Yes".
5. When done, click on "Container advent-of-code-2020" in the bottom-left corner of the window, then choose "Close Remote Connection".
