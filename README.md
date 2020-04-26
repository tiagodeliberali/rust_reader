# rust_reader
hexdump clone implementation, based on [Chapter 7](https://livebook.manning.com/book/rust-in-action/chapter-7/v-12/38) of the book Rust In Action.

## Usage

Get information from STDIN and display information similar to hexdump -C

<pre><font color="#A6E22E"><b>➜  </b></font><font color="#A1EFE4"><b>release</b></font> <font color="#66D9EF"><b>git:(</b></font><font color="#F92672"><b>master</b></font><font color="#66D9EF"><b>)</b></font> head rust_reader | ./rust_reader
[0000000000000000] 7f 45 4c 46 02 01 01 00  00 00 00 00 00 00 00 00   |.ELF............|
[0000000000000010] 03 00 3e 00 01 00 00 00  40 51 00 00 00 00 00 00   |.........Q......|
[0000000000000020] 40 00 00 00 00 00 00 00  d0 6a 2e 00 00 00 00 00   |...........j....|
[0000000000000030] 00 00 00 00 40 00 38 00  0c 00 40 00 2d 00 2c 00   |......8.........|
[0000000000000040] 06 00 00 00 04 00 00 00  40 00 00 00 00 00 00 00   |................|
[0000000000000050] 40 00 00 00 00 00 00 00  40 00 00 00 00 00 00 00   |................|
...
</pre>

## Differences

Due to implementation details, `rust_reader` do not garantee that all lines displayed will have 16 characters. So, it is possible to find lines like:

<pre>[00000000000050d0] ff 25 22 19 04 00 68 0a  00 00 00 e9 40 ff ff ff   |........h.......|
[00000000000050e0] ff 25 1a 19 04 00 68 0b  00 00 00 e9 30 ff ff ff   |........h.......0|
[00000000000050f0] ff 25 3a 19 04 00 66 90  ff 25 62 19 04 00 66 90   |........f.......b...f|
[0000000000005100] ff 25 6a 19 04 00 66 90  ff 25 3a 1a 04 00 66 90   |....j...f...........f|
[0000000000005110] ff 25 e2 1a 04 00 66 90  ff 25 32 1b 04 00 66 90   |..........f.......2...f|
[0000000000005120] ff 25 b2 1b 04 00 66 90  ff 25 02 1c 04 00 66 90   |..........f...........f|
[0000000000005130] ff 25 82 1c 04 00 66 90  ff 25 7a 1e 04 00 66 90   |..........f.......z...f|
[0000000000005140] 31 ed 49 89 d1 5e 48 89  e2 48 83 e4 f0 50 54 4c   |1...I.......H......H.........PTL|
[0000000000005150] 8d 05 ea e0 02 00 48 8d  0d 83 e0 02 00 48 8d 3d   |............H............H|
[0000000000005160] 9c 33 00 00 ff 15 ee 1a  04 00 f4 0f 1f 44 00 00   |...3.................D|
</pre>

It happens because this implementation uses:

<pre>
String::from_utf8_lossy(&buffer).match_indices(char::is_alphanumeric) 
</pre>

And, sometimes, the index returned by this function is greater than 16, so we display it in the supplied position. 
