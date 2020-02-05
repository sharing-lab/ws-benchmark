class Color < Application

  # GET /color
  def index
    list = db.query_to_hashes "select id, code, name from color"
    render json: list
  end

end
