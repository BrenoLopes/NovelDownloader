package br.balladesh.noveldownloader

import br.balladesh.noveldownloader.providers.Chapter
import org.jsoup.Connection
import org.jsoup.Jsoup
import java.net.URL
import java.util.concurrent.ExecutorCompletionService
import java.util.concurrent.ExecutorService
import java.util.concurrent.Executors
import java.util.concurrent.atomic.AtomicInteger
import kotlin.system.exitProcess

// Functions to fetch list of chapters
fun downloadChapters(
  url: String,
  persist: DiskPersist,
  cssQuery: String,
  appendDomain: Boolean = false,
  method: Connection.Method = Connection.Method.GET
): List<Chapter> {
  val chapters = persist.get<String, List<Chapter>>(url)
    .orElseGet{
      val chapterList = fetchChapterList(url, cssQuery, appendDomain, method)
      persist.put(url, chapterList)
      chapterList
    }
    .reversed()

  return chapters
}

private fun fetchChapterList(
  url: String,
  cssQuery: String,
  appendDomain: Boolean = false,
  method: Connection.Method = Connection.Method.GET
): List<Chapter> {
  val theUrl = removeDoubleSlash("$url/")

  val domain = URL(url).host

  val chapterList = mutableListOf<Chapter>()

  val connection = Jsoup
    .connect(theUrl)
    .method(method)
    .userAgent(getUserAgent())
    .sslSocketFactory(SSLHelper.socketFactory())

  val response = connection.execute()

  if (response.statusCode() != 200) {
    println("Failed to load the chapters from this provider. Please contact the developer or update your app.")
    exitProcess(0)
  }

  val document = response.parse()
  val chapters = document.select(cssQuery)

  for(chapter in chapters) {
    val chapterUrl = if (appendDomain)
      "https://" + domain + chapter.attr("href")
    else
      chapter.attr("href")

    chapterList.add(Chapter(chapterUrl, chapter.text()))
  }

  return chapterList.toList()
}

// Functions to fetch each chapter
fun fetchAllChapters(
  chapterList: List<Chapter>,
  cssSelect: String,
  persist: DiskPersist,
  method: Connection.Method = Connection.Method.GET
)
{
  val threadPollExecutor = Executors.newFixedThreadPool(4)
  val completionService = createFetchChapterJobs(chapterList, threadPollExecutor, cssSelect, persist, method)
  doFetchAllChapters(chapterList, completionService)

  threadPollExecutor.shutdown()
}

private fun createFetchChapterJobs(
  chapterList: List<Chapter>,
  executorService: ExecutorService,
  cssQuery: String,
  persist: DiskPersist,
  method: Connection.Method = Connection.Method.GET
): ExecutorCompletionService<Unit> {
  val executorCompletionService = ExecutorCompletionService<Unit>(executorService)

  for (index in chapterList.indices) {
    executorCompletionService.submit {
      if (persist.get<String, String>(chapterList[index].url).isPresent) {
        return@submit
      }

      val theUrl = removeDoubleSlash("${chapterList[index].url}/")

      val document = Jsoup
        .connect(theUrl)
        .userAgent(getUserAgent())
        .method(method)
        .sslSocketFactory(SSLHelper.socketFactory())
        .get()

      val title = ("Chapter " + chapterList[index].title).replace("Chapter Chapter", "Chapter ")

      val html = "<h1>$title</h1>\n" + document
        .select(cssQuery)
        .html()
        .replace(chapterList[index].title, "")

      persist.put(chapterList[index].url, html)
    }
  }

  return executorCompletionService
}

private fun doFetchAllChapters(
  chapterList: List<Chapter>,
  executorCompletionService: ExecutorCompletionService<Unit>
) {
  val atomicCounter = AtomicInteger(0)

  for (index in chapterList.indices) {
    println("Downloading Chapter ${atomicCounter.incrementAndGet()}/${chapterList.size}")
    executorCompletionService.take()
  }
}

private fun removeDoubleSlash(url: String): String {
  return url
    .replace("//", "/")
    .replace(Regex("(https?):/"), "$1://")
}

private fun getUserAgent(): String {
  return "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.81 Safari/537.36 Vivaldi/4.3.2439.44"
}
