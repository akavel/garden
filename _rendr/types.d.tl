global record Html
  userdata
  to_string: function(self): string
  clone: function(self): Html
  find: function(self, selector: string): Html
  delete_children: function(self)
  add_children: function(self, from: Html)
  set_children: function(self, from: Html)
  eject_children: function(self): Html
  add_text: function(self, string)
  set_text: function(self, string)
  get_text: function(self): string
  set_attr: function(self, k: string, v: string)
end

global record html
  parse: function(html: string): Html
  from_md: function(md: string): Html
  new: function(): Html
end

