package br.balladesh.noveldownloader

import br.balladesh.noveldownloader.providers.NovelProviderFactory
import java.lang.Exception
import java.nio.charset.StandardCharsets
import java.nio.file.FileSystems
import java.nio.file.Files
import kotlin.system.exitProcess

fun main(args: Array<String>) {
  val params = parseCommandLine(args)

  // Load provider for the current Url
  val provider = NovelProviderFactory().loadProvider(params.url).orElseThrow {
    println("There is no supported provider for this url!")
    exitProcess(0)
  }

  val outputPath = FileSystems.getDefault().getPath("${params.output}.txt".replace(".txt.txt", ".txt"))

  val stringBuilder = StringBuilder()

  try {
    val stream = Files.lines(outputPath, StandardCharsets.UTF_8)
    stream.forEach { stringBuilder.append(it + "\n") }

    val cleanedText = stringBuilder
      .replace(Regex("^\\n+"), "")
      .replace(Regex("^(#.+)$"), "\n$1")
    Files.write(outputPath, cleanedText.toByteArray(StandardCharsets.UTF_8))

    stream.close()
  } catch(e: Exception) {
    println(e.message)
  }

  // provider.downloadNovel(params)
}
