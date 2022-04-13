# -*- coding: utf-8 -*-
"""
lavasnek_rs
-------------------
lavalink-rs bindings for Python

Cheat Sheet:

- Functions that return a `Result<T, E>` mean that it can raise an exception. `T` is the type they
return normally, and `E` is a list of possible exceptions that can raise.
- Functions that return an `Option<T>` mean that the value returned can be `None`, where `T` would be
the type of the returned value if not `None`.
- If something returns a `Future<T>`, it means that it returns
[this](https://docs.python.org/3/library/asyncio-future.html?#asyncio.Future),
and that function should be awaited to work.
- / on arguments means the end of positional arguments.
- Self (with a capital S) means the type of self.
- A type prefixed with `impl` means it's a Class that implements that Trait type.
"""

from .lavasnek_rs import *

__version__ = "0.1.0-alpha.5"
__author__ = "vicky5124 <vickyf5124@gmail.com>"
__license__ = "MPL-2.0"

__all__ = [
    "rust_sleep",
    "log_something",
    "Lavalink",
    "LavalinkBuilder",
    "PlayBuilder",
    "LavalinkEventHandler",
    "ConnectionInfo",
    "Track",
    "Tracks",
    "TrackQueue",
    "Info",
    "PlaylistInfo",
    "Node",
    "Band",
    "Stats",
    "PlayerUpdate",
    "TrackStart",
    "TrackFinish",
    "TrackException",
    "TrackStuck",
    "WebSocketClosed",
    "PlayerDestroyed",
    "NoSessionPresent",
    "NetworkError",
]
