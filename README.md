Study-RustyCap
==============

An experimental project to implement pcap processing in Rust language.
Implements processing of file structure described by wireshark wiki: http://wiki.wireshark.org/Development/LibpcapFileFormat

**Status:** Early Alpha. Expect to be rough around the edges.

**Compile:** rustc rustycap.rs

**Usage:** ./rcap dumpfile

LICENSE
-------
All code in repo under BSD 3-Clause license.

Work completed
--------------
* Command line argument processing and file reading for pcap files
* Hex pretty printer (for examining read out code)
* PCap global header decoding
* Packet header decoding

Work in progress
----------------
* Decoder redesign
* Code tree refactoring

TODO
----

###Major###
* Packet data decoding
* Subcommand processing
* To be defined after completion of decoder

###Minor###
* Add description for link types
* Bugfix: Constant length line number for pretty printer
* Bugfix: Line number for the first line of pretty printer
* Big Endian record decoder
* Documentation
* Test case generation

###Testing###
* Decoding of Big Endian file
