# policast

This was my first attempt to build a video streaming application in Rust. Unfortunately FFMPEG (which is the main library used for this app) was available only through the command line as we didn't find any library bindings for Rust at the time.
The result is a "rusty" (pun intended) UI which can deliver video with a couple seconds of latency over LAN (ain't much but it's honest work).

I might resume working on this in the future (maybe even make some working bindings for Rust to use FFMPEG).
