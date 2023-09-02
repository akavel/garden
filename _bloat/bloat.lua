print("hello lua!")

for _, article in ipairs(articles) do
  print()
  for k, v in pairs(article) do
    print(k, '=', v)
  end
end

local f = assert(io.open '_bloat/index.html')
local raw = assert(f:read '*a')
local parsed = html.parse(raw)
print(parsed:to_string())
-- print(raw)
