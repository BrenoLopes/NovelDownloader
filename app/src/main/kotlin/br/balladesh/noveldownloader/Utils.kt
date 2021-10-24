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
        .replace(Regex("(?m)^[\\r\\n\\u00A0]+"), "")
        .replace(Regex("(?m)^(.+)$"), "$1\n")
        .toByteArray()
    )
  } catch(e: Exception) {
    println(e.message)
  }
}
