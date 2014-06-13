Study-RustyCap
==============

An experimental project to implement pcap processing in Rust language.
Implements processing of file structure described by wireshark wiki: http://wiki.wireshark.org/Development/LibpcapFileFormat

**Status:** Early Alpha. Expect to be rough around the edges.

**Compile:** rustc rustycap.rs

**Usage:** ./rustycap dumpfile

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

##Major##
* Packet header decoding
* Packet data decoding
* Subcommand processing
* To be defined after completion of decoder

##Minor##
* Add description for link types
* Bugfix: Constant length line number for pretty printer

##Testing##
* Decoding of Big Endian file
