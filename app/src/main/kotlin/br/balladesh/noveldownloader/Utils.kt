package br.balladesh.noveldownloader

import java.lang.Exception
import java.nio.file.Files
import java.nio.file.Path

fun cleanFileParagraphs(file: Path)
{
  val buffer = StringBuilder()

  try {
    Files.readAllLines(file).forEach { buffer.appendLine(it) }

    Files.write(
      file,
      buffer
        .replace(Regex("(?m)^[\\r\\n]+"), "")
        .replace(Regex("(?m)^(.+)$"), "$1\n")
        .toByteArray()
    )

//    Files.write(
//      file,
//      stringBuilder
//        .replace(Regex("(?m)^[\r\n]+"), "")
//        .replace(Regex("(?m)^(#\\s.+)"), "\n$1")
//        .toByteArray()
//    )
  } catch(e: Exception) {
    println(e.message)
  }
}
