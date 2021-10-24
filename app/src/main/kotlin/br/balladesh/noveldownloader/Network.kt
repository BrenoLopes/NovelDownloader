package br.balladesh.noveldownloader

import br.balladesh.noveldownloader.providers.Chapter
import org.jsoup.Connection
import org.jsoup.Jsoup
import java.util.concurrent.ExecutorCompletionService
import java.util.concurrent.ExecutorService
import java.util.concurrent.Executors
import java.util.concurrent.atomic.AtomicInteger
import kotlin.system.exitProcess

// Functions to fetch list of chapters
fun downloadChapters(url: String, persist: DiskPersist, cssQuery: String): List<Chapter> {
  val chapters = persist.get<String, List<Chapter>>(url)
    .orElseGet{
      val chapterList = fetchChapterList(url, cssQuery)
      persist.put(url, chapterList)
      chapterList
    }
    .reversed()

  return chapters
}

private fun fetchChapterList(url: String, cssQuery: String): List<Chapter> {
  val theUrl = "$url/"
    .replace("//", "/")
    .replace("https:/", "https://")

  val connection = Jsoup
    .connect(theUrl)
    .method(Connection.Method.POST)
    .userAgent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.81 Safari/537.36")
    .ignoreHttpErrors(true)
    .sslSocketFactory(SSLHelper.socketFactory())
  val response = connection.execute()

  if (response.statusCode() != 200) {
    println("Failed to load the chapters from this provider. Please contact the developer or update your app.")
    exitProcess(0)
  }

  val chapterList = mutableListOf<Chapter>()

  val document = response.parse()
  val chapters = document.select(cssQuery)

  for(chapter in chapters) {
    chapterList.add(Chapter(chapter.attr("href"), chapter.text()))
  }

  return chapterList.toList()
}

// Functions to fetch each chapter
fun fetchAllChapters(chapterList: List<Chapter>, cssSelect: String, persist: DiskPersist)
{
  val threadPollExecutor = Executors.newFixedThreadPool(4)
  val completionService = createFetchChapterJobs(chapterList, threadPollExecutor, cssSelect, persist)
  doFetchAllChapters(chapterList, completionService)

  threadPollExecutor.shutdown()
}

private fun createFetchChapterJobs(
  chapterList: List<Chapter>,
  executorService: ExecutorService,
  cssQuery: String,
  persist: DiskPersist
): ExecutorCompletionService<Unit> {
  val executorCompletionService = ExecutorCompletionService<Unit>(executorService)

  for (index in chapterList.indices) {
    executorCompletionService.submit {
      if (!persist.get<String, String>(chapterList[index].url).isPresent) {
        val theUrl = "${chapterList[index].url}/"
          .replace("//", "/")
          .replace("https:/", "https://")

        val document = Jsoup
          .connect(theUrl)
          .userAgent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.81 Safari/537.36")
          .sslSocketFactory(SSLHelper.socketFactory())
          .get()
        var html = document
          .select(cssQuery)
          .html()
          .replace(chapterList[index].title, "")

        val title = ("Chapter " + chapterList[index].title).replace("Chapter Chapter", "Chapter ")
        html = "<h1>$title</h1>\n${html}"
        persist.put(chapterList[index].url, html)
      }
    }
  }

  return executorCompletionService
}

private fun doFetchAllChapters(chapterList: List<Chapter>, executorCompletionService: ExecutorCompletionService<Unit>) {
  val atomicCounter = AtomicInteger(0)

  for (index in chapterList.indices) {
    println("Downloading Chapter ${atomicCounter.incrementAndGet()}/${chapterList.size}")
    executorCompletionService.take()
  }
}
