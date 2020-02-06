# Application dependencies
require "action-controller"

# Filter out sensitive params that shouldn't be logged
filter_params = ["password", "bearer_token"]

require "granite/adapter/pg"
Granite.settings.logger = Logger.new(nil)
Granite::Connections << Granite::Adapter::Pg.new(name: "pg", url: "postgres:" +
  "//postgres@127.0.0.1/test_db?initial_pool_size=10&max_idle_pool_size=10")

# Application code
require "./controllers/application"
require "./controllers/*"
require "./models"

# Server required after application controllers
require "action-controller/server"

# Add handlers that should run before your application
ActionController::Server.before(
  ActionController::LogHandler.new(filter_params),
)
