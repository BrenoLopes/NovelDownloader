# NovelDownloader
A simple terminal application that grabs all the chapters from a novel website and put it all in a text file, using markdown,
to make it easy to create an epub file using calibre.

## How to use it
First you must install [calibre](https://calibre-ebook.com/download) in your directory of choice.
Then open your terminal and run the executable with ``noveldownloader -u <url> -c <calibre_directory>``.

If you installed in in the default ``C:\Program Files\Calibre2`` you can ommit the path.

For more details run ``noveldownloader -h``

## Supported websites
For the complete list, run ``noveldownloader --supported-websites`` for help.
