package br.balladesh.noveldownloader.providers

import br.balladesh.noveldownloader.*
import org.jsoup.Connection

class BoxNovel : NovelProvider
{
  private val persist = DiskPersist()

  override fun getProviderDomainName(): String {
    return "boxnovel.com"
  }

  override fun doSupportDomain(url: String): Boolean {
    return url.contains(this.getProviderDomainName())
  }

  override fun downloadNovel(params: CmdParams) {
    val theUrl = "${params.url}/ajax/chapters/"
      .replace(Regex("/{2,}"), "/")
      .replace("https:/", "https://")

    val chapterList = downloadChapters(
      theUrl,
      this.persist,
      ".wp-manga-chapter a",
      false,
      Connection.Method.POST
    )

    fetchAllChapters(
      chapterList,
      ".read-container .reading-content .text-left",
      this.persist,
      Connection.Method.GET
    )

    val inputPath = mergeChapters(chapterList, this.persist)

    convertIntoMarkdownFile(params.calibrePath, inputPath, params.output)
  }
}
