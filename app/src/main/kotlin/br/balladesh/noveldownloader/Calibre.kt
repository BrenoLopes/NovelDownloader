package br.balladesh.noveldownloader

import org.buildobjects.process.ProcBuilder
import java.io.BufferedReader
import java.io.InputStreamReader
import java.lang.Exception
import java.nio.file.FileSystems
import java.nio.file.Path
import kotlin.system.exitProcess

fun convertIntoMarkdownFile(calibreLocation: String, inputPath: Path, outputName: String) {
  println("Cleaning file with Calibre")

  val calibrePath = FileSystems.getDefault().getPath(calibreLocation, "ebook-convert")
  val outputPath = FileSystems.getDefault().getPath("${outputName}.txt".replace(".txt.txt", ".txt"))

  try {
    val procResult = ProcBuilder(calibrePath.toAbsolutePath().toString())
      .withArg(inputPath.toAbsolutePath().toString())
      .withArg(outputPath.toAbsolutePath().toString())
      .withArg("--smarten-punctuation")
      .withArgs("--txt-output-formatting", "markdown")
      .withArg("--force-max-line-length")
      .withArg("--keep-links")
      .withArg("--enable-heuristics")
//      .withArg("--disable-delete-blank-paragraphs")
      .withArgs("--html-unwrap-factor", "0.4")
      .withOutputConsumer {
        BufferedReader(InputStreamReader(it)).forEachLine { line ->
          println(line)
        }
      }
      .withNoTimeout()
      .run()

    if (procResult.exitValue == 0) {
      println("Finished")
    }
  } catch (e: Exception) {
    println("Couldn't find calibre. Please install it in the default folder or tell the app it's location. ${e.message}")
    exitProcess(1)
  }

  cleanFileParagraphs(outputPath)
}
