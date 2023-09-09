# Converter Buddy Webapp
<a href="https://attilio-oliva.github.io/converter-buddy-webapp/">
    <img src="https://img.shields.io/badge/GitHub%20Pages-222222?style=for-the-badge&logo=GitHub%20Pages&logoColor=white" />
</a>


This project uses purely Rust to create a complete web app utility for format conversion. The website uses [Dioxus](https://dioxuslabs.com/) framework and [converter-buddy](https://github.com/attilio-oliva/converter-buddy) library as a high-level conversion utility.

  

**Conversions are performed by browser**, this means it is safer to use for data sensible files. However, some conversion on a browser environment could not be available or slow (read [limitations](#Limitations)). The quick way to overcome these problems would require a server collaboration, but this project is intended to be a pure client-side web app (at least for now).

## Currenly supported conversions
Generally speaking, any major image conversion is supported: PNG, JPEG, BMP, GIF, TIFF. Furthermore, the SVG and WebP formats are also supported, but only if used as input formats.

Image to PDF conversions are also supported, but not the other way around.


## Limitations

- **Possible slow conversions**: access to local file system is slow, so conversions involving local files could be slow, especially for big files.

- **Possible browser freezing**: some conversions could freeze the browser tab for a while. The browser reserve a single thread for any web app, so if a conversion is too heavy, the browser will freeze until the conversion is completed. This is not a concern for small files, but it could be for big files.