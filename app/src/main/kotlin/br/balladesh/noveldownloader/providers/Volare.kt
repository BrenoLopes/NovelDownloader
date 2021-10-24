package br.balladesh.noveldownloader.providers

import br.balladesh.noveldownloader.*
import org.jsoup.Connection

class VolareNovels : NovelProvider
{
  private val persist = DiskPersist()

  override fun getProviderDomainName(): String {
    return "volarenovels.com";
  }

  override fun doSupportDomain(url: String): Boolean {
    return url.contains(this.getProviderDomainName())
  }

  override fun downloadNovel(params: CmdParams) {
    val theUrl = params.url.replace("http://", "https://")

    val chapterList = downloadChapters(
      theUrl,
      this.persist,
      ".panel .list-chapters a",
      true,
      Connection.Method.GET
    ).reversed()

    fetchAllChapters(
      chapterList,
      ".panel.panel-default .panel-body",
      this.persist,
      Connection.Method.GET
    )

    val inputPath = mergeChapters(chapterList, this.persist)

    convertIntoMarkdownFile(params.calibrePath, inputPath, params.output)
  }

}
