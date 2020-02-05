class Color < Application

  # GET /color
  def index
    list = db.query_all "select id, code, name from color",
      as: {id: Int32, code: String, name: String}
    render json: list
  end

end
