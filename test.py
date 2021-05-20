#! /usr/bin/env python3

from ctypes import cdll

lib = cdll.LoadLibrary("target/debug/libdummy.so")

print(lib.dummy())
