package br.balladesh.noveldownloader.providers

import br.balladesh.noveldownloader.CmdParams
import br.balladesh.noveldownloader.DiskPersist
import br.balladesh.noveldownloader.SSLHelper
import org.buildobjects.process.ProcBuilder
import org.jsoup.Connection
import org.jsoup.Jsoup
import java.io.BufferedReader
import java.io.FileOutputStream
import java.io.InputStreamReader
import java.io.OutputStreamWriter
import java.lang.Exception
import java.nio.file.FileSystems
import java.nio.file.Path
import java.util.*
import java.util.concurrent.ExecutorCompletionService
import java.util.concurrent.ExecutorService
import java.util.concurrent.Executors
import java.util.concurrent.atomic.AtomicInteger
import kotlin.system.exitProcess

class BoxNovel : NovelProvider
{
  private val persist = DiskPersist()

  override fun getProviderDomainName(): String {
    return "boxnovel.com"
  }

  override fun doSupportDomain(url: String): Boolean {
    return url.contains("boxnovel.com")
  }

  override fun downloadNovel(params: CmdParams) {
    val chapterList = this.downloadChapters(params.url)

    val fetchJobs = this.createFetchChapterJobs(chapterList)
    this.fetchAllChapters(chapterList, fetchJobs.second)

    val inputPath = this.mergeChapters(chapterList)

    this.convertIntoMarkdownFile(params.calibrePath, inputPath, params.output)

    fetchJobs.first.shutdown()
  }

  private fun downloadChapters(url: String): List<Chapter> {
    val chapters = persist.get<String, List<Chapter>>(url)
      .orElseGet{
        val chapterList = fetchChapterList(url)
        persist.put(url, chapterList)
        chapterList
      }
      .reversed()

    return chapters
  }

  private fun fetchChapterList(url: String): List<Chapter> {
    val theUrl = "$url/".replace("//", "/").replace("https:/", "https://")

    val connection = Jsoup
      .connect("${theUrl}ajax/chapters/")
      .method(Connection.Method.POST)
      .sslSocketFactory(SSLHelper.socketFactory())
    val response = connection.execute()

    if (response.statusCode() != 200) {
      println("Failed to load the chapters from this provider. Please contact the developer or update your app.")
      exitProcess(0)
    }

    val chapterList = mutableListOf<Chapter>()

    val document = response.parse()
    val chapters = document.select(".wp-manga-chapter a")

    for(chapter in chapters) {
      chapterList.add(Chapter(chapter.attr("href"), chapter.text()))
    }

    return chapterList.toList()
  }

  private fun createFetchChapterJobs(chapterList: List<Chapter>): Pair<ExecutorService, ExecutorCompletionService<Unit>> {
    val threadPollExecutor = Executors.newFixedThreadPool(4)
    val executorCompletionService = ExecutorCompletionService<Unit>(threadPollExecutor)

    for (index in chapterList.indices) {
      executorCompletionService.submit {
        if (!persist.get<String, String>(chapterList[index].url).isPresent) {
          val document = Jsoup.connect(chapterList[index].url).sslSocketFactory(SSLHelper.socketFactory()).get()
          var html = document
            .select(".read-container .reading-content .text-left")
            .html()
            .replace(chapterList[index].title, "")

          val title = ("Chapter " + chapterList[index].title).replace("Chapter Chapter", "Chapter ")
          html = "<h1>$title</h1>\n${html}"
          persist.put(chapterList[index].url, html)
        }
      }
    }

    return Pair(threadPollExecutor, executorCompletionService)
  }

  private fun fetchAllChapters(chapterList: List<Chapter>, executorCompletionService: ExecutorCompletionService<Unit>) {
    val atomicCounter = AtomicInteger(0)

    for (index in chapterList.indices) {
      println("Downloading Chapter ${atomicCounter.incrementAndGet()}/${chapterList.size}")
      executorCompletionService.take()
    }
  }

  private fun mergeChapters(chapterList: List<Chapter>): Path {
    val path = FileSystems.getDefault().getPath("tmp.html")
    val osw = OutputStreamWriter(FileOutputStream(path.toFile()), "utf-8")

    for (chapter in chapterList) {
      val html = persist.get<String, String>(chapter.url).orElse("")
      osw.write(html + "\n")
    }

    println("Closing")
    osw.close()

    return path
  }

  private fun convertIntoMarkdownFile(calibreLocation: String, inputPath: Path, outputName: String) {
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
  }
}