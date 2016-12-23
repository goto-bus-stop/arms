import DataObject from require 'arms.DataObject'

class Messages extends DataObject
  new: => super
    instructions: ''
    hints: ''
    victory: ''
    loss: ''
    history: ''
    scouts: ''

  instructions: (str) => @set instructions: str
  hints: (str) => @set hints: str
  victory: (str) => @set victory: str
  loss: (str) => @set loss: str
  history: (str) => @set history: str
  scouts: (str) => @set scouts: str

-- Exports
{ :Messages }
