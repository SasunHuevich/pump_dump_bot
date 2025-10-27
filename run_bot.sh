#!/bin/bash

docker build -t pump_dump_bot .

docker run -d \
    --name pump_dump_bot \
    -v $(pwd):/pump_dump_bot \
    -p 8080:80 pump_dump_bot