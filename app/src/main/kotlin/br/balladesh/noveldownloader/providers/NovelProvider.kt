package br.balladesh.noveldownloader.providers

import br.balladesh.noveldownloader.CmdParams
import java.io.Serializable

interface NovelProvider
{
  fun getProviderDomainName(): String
  fun doSupportDomain(url: String): Boolean
  fun downloadNovel(params: CmdParams): Unit
}

data class Chapter(val url: String, val title: String) : Serializable
