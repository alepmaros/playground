#!/bin/bash

redis-server --daemonize yes
jupyter lab --no-browser --ip=0.0.0.0