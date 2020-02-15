require "athena"
require "pg"

APPDB = DB.open "postgres://postgres@127.0.0.1/test_db?" +
  "initial_pool_size=10&max_idle_pool_size=10"
def db
  return APPDB
end

class ExampleController < ART::Controller
  @[ART::Get("/color")]
  def root : String
    "At the index"
  end
end

ART.run(55514)
