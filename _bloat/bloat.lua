print("hello lua!")

local function readfile(name)
  local fh = assert(io.open(name))
  local data = assert(fh:read '*a')
  fh:close()
  return data
end

local function writefile(name, content)
  local fh = assert(io.open(name, 'w'))
  assert(fh:write(content))
  fh:close()
end

local function main()
  local template = html.parse(readfile '_bloat/bloat.html')

  for _, article in ipairs(articles) do

    -- Parse markdown article from disk into HTML.
    print("RENDERING " .. article.slug)
    local text = html.from_md(readfile(article.src))

    -- Put the main text of the article in #content node in the template.
    local template = template:clone()
    local slot = template:find '#content'
    template:delete_children(slot)
    template:add_children(slot, text, text:root())

    -- Set title in the template based on <h1> tag in the article.
    local title = template:find 'html head title'
    template:delete_children(title)
    -- TODO: should strip html tags from the text - not allowed really
    template:add_children(title, text, text:find 'h1')
    -- TODO: add suffix text
    -- local suffix = html.parse ' - scribble by akavel'
    -- template:add_children(title, suffix, suffix:find 'body')

    -- FIXME: fix relative links - strip .md etc.
    -- TODO: copy images, css

    -- Write filled template to disk.
    writefile('_html.out/'..article.slug, template:to_string())
  end
end

main()
