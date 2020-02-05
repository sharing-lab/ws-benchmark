lazy val commonSettings = Seq(
  name := "ws-benchmark",
  version := "1.0",
  scalaVersion := "2.12.9",
  scalacOptions ++= Seq(
    "-deprecation",
    "-Xfatal-warnings",
    "-Ywarn-value-discard",
    "-Xlint:missing-interpolator"
  ),
)

lazy val Http4sVersion = "0.18.22"
lazy val DoobieVersion = "0.5.4"
lazy val CirceVersion = "0.9.3"
lazy val PureConfigVersion = "0.10.2"
lazy val LogbackVersion = "1.2.3"

lazy val root = (project in file("."))
  .configs(IntegrationTest)
  .settings(
    commonSettings,
    Defaults.itSettings,
    libraryDependencies ++= Seq(
      "org.http4s"            %% "http4s-blaze-server"  % Http4sVersion,
      "org.http4s"            %% "http4s-circe"         % Http4sVersion,
      "org.http4s"            %% "http4s-dsl"           % Http4sVersion,

      "org.tpolecat"          %% "doobie-core"          % DoobieVersion,
      "org.tpolecat"          %% "doobie-hikari"        % DoobieVersion,
      "org.tpolecat"          %% "doobie-postgres"      % DoobieVersion,

      "io.circe"              %% "circe-generic"        % CirceVersion,
      "io.circe"              %% "circe-literal"        % CirceVersion,
      "io.circe"              %% "circe-optics"         % CirceVersion,

      "com.github.pureconfig" %% "pureconfig"           % PureConfigVersion,

      "ch.qos.logback"        %  "logback-classic"      % LogbackVersion,
    )
  )
