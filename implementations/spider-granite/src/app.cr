require "./config"

# Server defaults
port = 55511
host = "127.0.0.1"

# Load the routes
server = ActionController::Server.new(port, host, false)

# Start the server
server.run do
  puts "Listening on #{server.print_addresses}"
end

# Shutdown message
puts "web server leaps through the veldt\n"
