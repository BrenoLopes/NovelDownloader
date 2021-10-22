package br.balladesh.noveldownloader

import java.lang.Exception
import java.nio.charset.StandardCharsets
import java.nio.file.Files
import java.nio.file.Path

enum class AppState {
  DownloadChapterList,
  DownloadChapter,
  CleanChapter,
  CombineChapters,
  ConvertText,
  Finished,
}

data class Metadata(val chapterListSize: Long, val currentState: AppState = AppState.DownloadChapterList)

fun cleanFileParagraphs(file: Path)
{
  val stringBuilder = StringBuilder()

  try {
    val allLines = Files.readAllLines(file)
    allLines.forEach { stringBuilder.appendLine(it) }

    Files.write(
      file,
      stringBuilder
        .replace(Regex("(?m)^[\r\n]+"), "")
        .replace(Regex("(?m)^(#\\s.+)"), "\n$1")
        .toByteArray()
    )

    val cleanedText = stringBuilder
      .replace(Regex("^\\n+"), "")
      .replace(Regex("^(#.+)$"), "\n$1")
    Files.write(file, cleanedText.toByteArray(StandardCharsets.UTF_8))
  } catch(e: Exception) {
    println(e.message)
  }
}