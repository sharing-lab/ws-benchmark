require "./config"

# Server defaults
port = 55501
host = "192.168.1.12"

# Load the routes
server = ActionController::Server.new(port, host, false)

# Start the server
server.run do
  puts "Listening on #{server.print_addresses}"
end

# Shutdown message
puts "web server leaps through the veldt\n"
