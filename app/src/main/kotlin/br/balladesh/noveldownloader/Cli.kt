package br.balladesh.noveldownloader

import br.balladesh.noveldownloader.providers.NovelProviderFactory
import org.apache.commons.cli.*
import kotlin.system.exitProcess

data class CmdParams(val url: String, val output: String, val calibrePath: String)

fun parseCommandLine(args: Array<String>): CmdParams {
  val appName = "noveldownloader"
  val appVersion = "1.0-SNAPSHOT"
  val appDescription = "This application downloads all the chapters from a novel's table of content page and put it " +
    "all inside a single file. You can then create an ebook format with it later by using it as input in calibre and " +
    "modifying the look and feel based in your preferences."

  val options = Options()
    .addOption(
      Option
        .builder("u")
        .longOpt("url")
        .hasArg()
        .argName("link")
        .desc("Novel Link (See --supported-websites for all websites supported)")
        .type(String::class.java)
        .build()
    )
    .addOption(
      Option
        .builder()
        .longOpt("calibre-dir")
        .argName("path")
        .desc("The directory's path you installed Calibre2. [Default <C:\\Program Files\\Calibre2> on Windows]")
        .hasArg()
        .type(String::class.java)
        .build()
    )
    .addOption(
      Option
        .builder("o")
        .longOpt("output")
        .argName("output-name")
        .desc("The name of the file the chapters will be saved.")
        .hasArg()
        .type(String::class.java)
        .build()
    )
    .addOptionGroup(
      OptionGroup()
        .addOption(
          Option
            .builder()
            .longOpt("supported-websites")
            .desc("List from all supported websites")
            .hasArg(false)
            .build()
        )
        .addOption(
          Option.builder("h")
            .longOpt("help")
            .hasArg(false)
            .desc("Display this message")
            .build()
        )
    )

  try {
    val parsedArgs = DefaultParser()
      .parse(options, args)

    if (parsedArgs.hasOption("supported-websites")) {
      println("-------------Supported novel websites-----------------")
      NovelProviderFactory().displayAllProviderDomainNames()
      exitProcess(0)
    }

    if (parsedArgs.hasOption("h"))
      throw Exception("Show Help")

    val url = parsedArgs.getOptionValue("u")
    var calibrePath = ""
    var outputName = "novel.txt"

    if (System.getProperty("os.name").lowercase().contains("win"))
      calibrePath = "C:\\Program Files\\Calibre2"

    if (parsedArgs.hasOption("calibre-dir"))
      calibrePath = parsedArgs.getOptionValue("calibre-dir")

    if (parsedArgs.hasOption("o"))
      outputName = parsedArgs.getOptionValue("o")

    return CmdParams(url, outputName, calibrePath)
  } catch (e: Exception) {
    displayHelpMessage(appName, appVersion, appDescription, options)
    exitProcess(0)
  }
}

fun displayHelpMessage(appName: String, appVersion: String, appDescription: String, options: Options) {
  println(String.format("%s %s", appName, appVersion))

  var charCount = 0
  val tmp = appDescription.split(" ")

  val formatter = HelpFormatter()

  // Print the word if the sum of all previous words plus the new one do not exceed char line limit
  for (i in tmp.indices) {
    val count = tmp[i].length + charCount + 1
    if (count > formatter.width) {
      print('\n' + tmp[i] + " ")
      charCount = tmp[i].length
    } else {
      print(tmp[i] + " ")
      charCount += tmp[i].length
    }
  }
  println()

  formatter.printHelp("noveldownloader -u <link> [-o <output> | --calibre-dir <path>] | --supported-websites | -h", options)
}
