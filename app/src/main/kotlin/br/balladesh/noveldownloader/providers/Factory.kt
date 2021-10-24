package br.balladesh.noveldownloader.providers

import java.util.*

class NovelProviderFactory
{
  private val providers = arrayListOf(
    BoxNovel(),
    VolareNovels()
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
