require "granite/adapter/pg"

class ColorRecord < Granite::Base
  connection pg

  table color
  column id : Int32, primary: true
  column code : String
  column name : String
end