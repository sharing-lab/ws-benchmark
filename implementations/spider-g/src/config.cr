# Application dependencies
require "action-controller"

# Filter out sensitive params that shouldn't be logged
filter_params = ["password", "bearer_token"]

# Application code
require "./controllers/application"
require "./controllers/*"

# Server required after application controllers
require "action-controller/server"

# Add handlers that should run before your application
ActionController::Server.before(
  ActionController::LogHandler.new(filter_params),
)
