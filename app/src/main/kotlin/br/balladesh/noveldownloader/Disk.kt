package br.balladesh.noveldownloader

import java.io.*
import java.nio.file.FileSystems
import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.Paths
import java.security.MessageDigest
import java.util.*
import java.util.concurrent.locks.ReentrantLock
import kotlin.system.exitProcess

class DiskPersist(private val path: String = ".cache")
{
  private val cacheFolder: Path
  private val digester: MessageDigest

  init {
    lock.lock()

    this.cacheFolder = FileSystems.getDefault().getPath(".", this.path)
    this.digester = MessageDigest.getInstance("SHA-256")

    if (!this.doCacheFolderExist()) {
      Files.createDirectory(cacheFolder)
    }

    lock.unlock()
  }

  fun <T, U> put(key: T, value: U) {
    try {
      lock.lock()

      val keyHash = objectToSha3(key)

      val objectOutputStream = ObjectOutputStream(FileOutputStream(pathToUri(keyHash).toFile()))
      objectOutputStream.writeObject(value)
      objectOutputStream.close()
    } catch (e: Exception) {
      println("Failed to serialize object into a file!")
      exitProcess(1)
    } finally {
      lock.unlock()
    }
  }

  @Suppress("UNCHECKED_CAST")
  fun <T, U> get(key: T): Optional<U> {
    try {
      lock.lock()

      val keyHash = objectToSha3(key)

      val objectInputStream = ObjectInputStream(FileInputStream(pathToUri(keyHash).toFile()))
      val data = objectInputStream.readObject()
      objectInputStream.close()

      return Optional.ofNullable(data as U)
    } catch(e: IOException) {
      return Optional.empty()
    } catch (e: Exception) {
      println("Failed to deserialize object from a file!")
      exitProcess(1)
    } finally {
      lock.unlock()
    }
  }

  private fun doCacheFolderExist(): Boolean {
    return Files.exists(this.cacheFolder)
  }

  private fun pathToUri(path: String): Path {
    return Paths.get(FileSystems.getDefault().getPath(".", this.path, path).toUri())
  }

  private fun <T> objectToSha3(obj: T): String {
    try {
      val keyByteArray = obj.hashCode().toString().toByteArray()
      val sha512 = this.digester.digest(keyByteArray)

      val sb = StringBuilder()
      for (element in sha512) {
        val tmp = (element.toInt() and 0xff) + 0x100
        sb.append(tmp.toString(16).substring(1))
      }
      return sb.toString()
    } finally {
      this.digester.reset()
    }
  }

  companion object {
    private val lock = ReentrantLock()
  }
}