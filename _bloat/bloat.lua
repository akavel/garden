print("hello lua!")

for _, article in ipairs(articles) do
  print()
  for k, v in pairs(article) do
    print(k, '=', v)
  end
end
