class DataObject
  new: (data) => @data = data

  set: (props) =>
    for k, v in pairs props do @data[k] = v
    @

  to_json: => @data

{ :DataObject }
