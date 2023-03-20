#!/bin/bash
socat -d -d pty,raw,echo=0,b115200 pty,raw,echo=0,b115200
#socat -d -d pty,rawer,echo=0,b115200,crnl pty,rawer,echo=0,b115200,crnl &

