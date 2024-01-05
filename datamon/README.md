# datamon

<p align="center">
DataFrame IPC via named pipes
</p>
<br>

![Example](/docs/media/datamon.gif)

## Getting Started 

#### Setup Python env
```bash
    # create env
    python3 -m venv client
    # activate env
    source client/bin/activate
```
#### Install dependencies and build
```bash
    make install
```
- **Note the _receiver_ and _sender_ must run in separate shells**

#### Run the receiver
```shell
    make receiver
```
#### Run the sender
```
    make sender
```

