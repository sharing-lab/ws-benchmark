require "kemal"
require "pg"
require "json"

APPDB = DB.open "postgres://postgres@localhost:5432/test_db?" +
  "initial_pool_size=10&max_idle_pool_size=10"
def db
  return APPDB
end

get "/kapal" do |env|
  env.response.content_type = "application/json"
  list = db.query_all "select id, code, name from color",
    as: {id: Int32, code: String, name: String}
  list.to_json
end

Kemal.run 55504
