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

local function table_transpose(t)
  local r = {}
  for k, v in pairs(t) do
    r[v] = k
  end
  return r
end

local function main()
  local template = html.parse(readfile '_bloat/bloat.html')

  -- Render articles.
  for _, article in ipairs(articles) do

    -- Parse markdown article from disk into HTML.
    print("RENDERING " .. article.slug)
    local text = html.from_md(readfile(article.src))
    article.html = text

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

  -- Render index.
  -- Sort articles, newest first.
  table.sort(articles, function(a, b) return a.datetime > b.datetime end)
  local index = html.parse(readfile '_bloat/index.html')
  local list = index:find '#articles'
  local template = html.new()
  template:add_children(template:root(), index, list)
  template:delete_children(template:find 'h2 a')
  template:delete_children(template:find 'time')
  template:delete_children(template:find 'ul li a')
  index:delete_children(list)
  for _, art in ipairs(articles) do
    local tags = table_transpose(art.tags)
    if not tags._drafts then
      local template = template:clone()

      local art_h1 = art.html:find 'h1'
      template:add_children(template:find 'h2 a', art.html, art_h1)
      template:set_attr(template:find 'h2 a', 'href', art.slug)

      local datetime = art.datetime:gsub('(%d%d%d%d)(%d%d)(%d%d).*', '%1-%2-%3')
      template:add_text(template:find 'time', datetime)

      local tag_tmpl = html.new()
      tag_tmpl:add_children(tag_tmpl:root(), template, template:find 'ul')
      template:delete_children(template:find 'ul')
      for _, tag in ipairs(art.tags) do
        -- print(tag_tmpl:to_string())
        tag_tmpl:delete_children(tag_tmpl:find 'li a')
        tag_tmpl:add_text(tag_tmpl:find 'li a', '@'..tag)
        tag_tmpl:set_attr(tag_tmpl:find 'li a', 'href', '@'..tag)
        template:add_children(template:find 'ul', tag_tmpl, tag_tmpl:root())
      end

      index:add_children(list, template, template:root())
    end
  end
  writefile('_html.out/index.html', index:to_string())
end

main()
