package br.balladesh.noveldownloader

enum class AppState {
  DownloadChapterList,
  DownloadChapter,
  CleanChapter,
  CombineChapters,
  ConvertText,
  Finished,
}

data class Metadata(val chapterListSize: Long, val currentState: AppState = AppState.DownloadChapterList)