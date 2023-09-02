print("hello lua!")

for _, article in ipairs(articles) do
  print()
  for k, v in pairs(article) do
    print(k, '=', v)
  end
end

local f = assert(io.open '_bloat/index.html')
local raw = assert(f:read '*a')
-- print(raw)
local parsed = html.parse(raw)
local node = parsed:find("#content")
-- print(node)
parsed:delete_children(node)

-- local other = html.parse("<b>HELLO OTHER</b>")
local other = html.from_md 'HELLO **WORLD**'
local other_node = other:root();
parsed:add_children(node, other, other_node)

print(parsed:to_string())

