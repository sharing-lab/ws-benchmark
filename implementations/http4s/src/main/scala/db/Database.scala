package db

import cats.effect.IO
import config.DatabaseConfig
import doobie.hikari.HikariTransactor

object Database {
  def transactor(config: DatabaseConfig): IO[HikariTransactor[IO]] = {
    HikariTransactor.newHikariTransactor[IO](config.driver, config.url, config.user, config.password)
  }
}
