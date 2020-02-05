package repository

import cats.effect.IO
import doobie.util.transactor.Transactor
import fs2.Stream
import doobie._
import doobie.implicits._
import model._

class ColorRepository(transactor: Transactor[IO]) {

  def list: Stream[IO, Color] = {
    sql"""
      SELECT id, code, name FROM color
    """.query[Color].stream.transact(transactor)
  }

}
