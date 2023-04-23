# Converter Buddy Webapp

This project uses purely Rust to create a complete webapp utility for format conversion. The website uses [Yew](https://yew.rs/) framework and [converter-buddy](https://github.com/attilio-oliva/converter-buddy) library as a high-level conversion utility.

Conversions are performed by browser, this means it is safer to use for data sensible files. However, some conversion on a browser environment could not be available or slow (read [limitations](#Limitations)). The quick way to overcome these problems would require a server collaboration if desired (to add in future) 

## Limitations
- Underlying Rust libraries could use std::fs to access the current filesystem, but such thing is not possible in a browser environment. Because of that, library API that accepts pure data as vectors.

- Operations performed by underlying Rust libraries are by nature synchronous. The reason stands in the usage of std::io traits like Read and Write for I/O operations. This will translate in a possible blocking conversion, causing the browser to freeze. Browsers put apps in a single thread environment, so there is no way other than using only asynchronous libraries for conversions. Unfortunately, there is a lack of such type of libraries, limiting the combinations of format types supported in this app.