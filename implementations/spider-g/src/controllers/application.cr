require "pg"

APPDB = DB.open "postgres://postgres@127.0.0.1/test_db?" +
	"initial_pool_size=10&max_idle_pool_size=100"

abstract class Application < ActionController::Base

  def db
    return APPDB
  end

end
