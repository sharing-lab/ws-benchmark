class Color < Application

  # GET /color
  def index
    list = ColorRecord.all
    render json: list
  end

end
