package br.balladesh.noveldownloader.providers

import br.balladesh.noveldownloader.CmdParams
import java.io.Serializable
import java.util.*

data class Chapter(val url: String, val title: String) : Serializable

interface NovelProvider
{
  fun getProviderDomainName(): String
  fun doSupportDomain(url: String): Boolean
  fun downloadNovel(params: CmdParams): Unit
}

class NovelProviderFactory
{
  private val providers = arrayListOf<NovelProvider>(
    BoxNovel()
  )

  fun loadProvider(url: String): Optional<NovelProvider> {
    for (provider in providers) {
      if (!provider.doSupportDomain(url))
        continue
      return Optional.of(provider)
    }

    return Optional.empty()
  }

  fun displayAllProviderDomainNames() {
    for (provider in providers) {
      println(provider.getProviderDomainName())
    }
  }
}
