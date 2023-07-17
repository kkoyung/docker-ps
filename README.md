# docker-ps

This is my custom-formatted `docker ps` command. It simply calls `docker ps` command, read its output and print it out in my custom format. The output format includes the following columns:

- Container ID
- Image used
- Container name
- Container status
- Exposed and published ports

## Build

```bash
$ cargo build --release
# Executable is built at target/release/docker-ps
```

## Usage

Example:

```plain
$ docker-ps
CONTAINER ID  IMAGE                          NAMES                       STATUS                      PORTS
9aa6c90e1ab4  drone/drone-runner-docker:1    drone-runner-1              Up About an hour            ─3000/tcp
6e6e845161e7  drone/drone:2                  drone-app-1                 Up About an hour            ┬80/tcp
                                                                                                     ├0.0.0.0:5000->443/tcp
                                                                                                     └:::5000->443/tcp
6914bb420f89  lissy93/dashy                  dashy-app-1                 Up About an hour (healthy)  ─127.0.0.1:4000->80/tcp
dd0c832b41bd  moby/buildkit:buildx-stable-1  buildx_buildkit_mybuilder0  Exited (1) 13 days ago      ╌
```
