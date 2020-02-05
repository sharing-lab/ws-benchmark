package service

import cats.effect.IO
import org.http4s.{HttpService, MediaType, Uri}
import org.http4s.dsl.Http4sDsl
import org.http4s.circe._
import io.circe.generic.auto._
import io.circe.syntax._
import fs2.Stream
import repository._
import org.http4s.headers.{Location, `Content-Type`}

class ColorService(repo: ColorRepository) extends Http4sDsl[IO] {

  val service = HttpService[IO] {

    case GET -> Root / "color" =>
      Ok(Stream("[") ++ repo.list.map(_.asJson.noSpaces).intersperse(",") ++ Stream("]"), `Content-Type`(MediaType.`application/json`))

  }
}
