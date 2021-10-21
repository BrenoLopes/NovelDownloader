# NovelDownloader
A simple terminal application that grabs all the chapters from a novel website and put it all in a text file, using markdown,
to make it easy to create an epub file using calibre.

## How to use it
First you must install [calibre](https://calibre-ebook.com/download) in your directory of choice.
Then open your terminal and run the executable with ``java -jar noveldownloader.jar -u <url> --calibre-dir <calibre_directory>``.

I'll generate a txt file in markdown format, so that you can use to convert it into wharever format you want by using calibre or
any other websites that supports converting from markdown.

If you installed it in the default folder ``C:\Program Files\Calibre2`` you can omit the path.

For more details run ``java -jar noveldownloader.jar -h``

## Requirements
Java 8+
Calibre - E-book Management

## Supported websites
For the complete list, run ``java -jar noveldownloader.jar --supported-websites`` for help.
