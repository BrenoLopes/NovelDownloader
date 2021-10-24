package br.balladesh.noveldownloader

import br.balladesh.noveldownloader.providers.NovelProviderFactory
import kotlin.system.exitProcess

fun main(args: Array<String>) {
  val params = parseCommandLine(args)

  // Load provider for the current Url
  val provider = NovelProviderFactory().loadProvider(params.url).orElseThrow {
    println("There is no supported provider for this url!")
    exitProcess(0)
  }

  provider.downloadNovel(params)
}
