Study-RustyCap
==============

An experimental project to implement pcap processing in Rust language.
Implements processing of file structure described by wireshark wiki: http://wiki.wireshark.org/Development/LibpcapFileFormat

**Status:** Early Alpha. Expect to be rough around the edges.

LICENSE
-------
All code in repo under BSD 3-Clause license.

Work completed
--------------
* Command line argument processing and file reading for pcap files
* Hex pretty printer (for examining read out code)

Work in progress
----------------
* PCap global header decoding

TODO
----
* Packet header decoding
* Packet data decoding
* To be defined after completion of decoder
