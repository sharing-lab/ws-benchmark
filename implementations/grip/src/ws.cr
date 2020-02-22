require "grip"
require "pg"

APPDB = DB.open "postgres://postgres@127.0.0.1/test_db?" +
  "initial_pool_size=10&max_idle_pool_size=100"
def db
  return APPDB
end

class IndexHttpConsumer < Grip::HttpConsumer
  def get(req)
    list = db.query_all "select id, code, name from color",
      as: {id: Int32, code: String, name: String}
    json(list)
  end
end

class IdApi < Grip::Application
  scope do
    resource "/color", IndexHttpConsumer
  end
end

id_api = IdApi.new
id_api.run(55513)
