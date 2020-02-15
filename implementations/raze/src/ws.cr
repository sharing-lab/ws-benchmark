require "raze"
require "pg"
require "json"

APPDB = DB.open "postgres://postgres@127.0.0.1/test_db?" +
  "initial_pool_size=10&max_idle_pool_size=10"
def db
  return APPDB
end

get "/color" do |env|
  env.response.content_type = "application/json"
  list = db.query_all "select id, code, name from color",
    as: {id: Int32, code: String, name: String}
  list.to_json
end

Raze.run 55514
