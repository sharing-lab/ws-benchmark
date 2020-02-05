require "pg"

APPDB = DB.open "postgres://postgres@localhost:5432/test_db?" +
	"initial_pool_size=10&max_idle_pool_size=10"

abstract class Application < ActionController::Base

  def db
    return APPDB
  end

end
