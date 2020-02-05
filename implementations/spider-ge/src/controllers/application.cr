require "pg"

APPDB = DB.open "postgres://postgres@localhost:5432/test_db?" +
	"initial_pool_size=10&max_idle_pool_size=10"

abstract class Application < ActionController::Base

  def db
    return APPDB
  end

end

alias SimpleTypes = (String|Int8|Int16|Int32|Int64|Nil)

class DB::ResultSet
  def get_hash
    row = Hash(String, SimpleTypes).new
    self.each_column do |col_name|
      row[col_name] = self.read(SimpleTypes)
    end
    row
  end
end

class DB::Database

  def query_to_hashes(query, *args)
    rows = [] of Hash(String, SimpleTypes)
    self.query(query, *args) { |rs| rs.each { rows << rs.get_hash } }
    rows
  end

  def query_to_hash(query, *args)
    row = Hash(String, SimpleTypes).new
    self.query_one(query, *args) { |rs| row = rs.get_hash }
    row
  end

end
