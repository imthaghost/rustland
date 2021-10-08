import os
import select
import pandas as pd
from io import BytesIO
from select import poll, POLLIN


def get_message(fifo: int) -> str:
    size = os.read(fifo)

def connect_to_pipe():
    pass


if __name__ == "__main__":
    FIFO = '/tmp/datamon/data'
    fifo = os.open(FIFO, os.O_RDONLY | os.O_NONBLOCK)
    p = poll()
    p.register(fifo, POLLIN)

    while True:
        events = p.poll(1)
        for e in events:
            
            bytes = os.read(fifo, 1000)
            buf = BytesIO(bytes)
            print(pd.read_feather(buf))
            # macD










    

