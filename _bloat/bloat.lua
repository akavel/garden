print("hello lua!")

local TITLE_SUFFIX = " — akavel's digital garden"

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

local function article_date(art)
  return (art.datetime:gsub('(%d%d%d%d)(%d%d)(%d%d).*', '%1-%2-%3'))
end

local function render_article(template, article)
  -- Parse markdown article from disk into HTML.
  local text = html.from_md(readfile(article.src))
  article.html = text

  -- Put the main text of the article in #content node in the template.
  template:find('#content'):set_children(text)
  template:find('#navhome + header > time'):set_text(article_date(article))

  local greenery_kind =
    article.tags.seed and 'seed' or
    article.tags.bud and 'bud' or
    article.tags.ripe and 'ripe' or
    ''
  template:find('#navhome + header > time'):set_attr('class', greenery_kind)

  -- Set tags
  local tag_tmpl = template:find('ul.tags'):eject_children()
  for _, tag in ipairs(article.tags) do
    -- print(tag_tmpl:to_string())
    tag_tmpl:find('li a'):set_text('@'..tag)
    tag_tmpl:find('li a'):set_attr('href', '@'..tag)
    template:find('ul.tags'):add_children(tag_tmpl)
  end

  -- Set title in the template based on <h1> tag in the article.
  local title = template:find 'html head title'
  local h1 = text:find 'h1'
  if h1 then
      title:set_text(h1:get_text())
  else
      title:set_text(article.slug)
  end
  title:add_text(TITLE_SUFFIX)

  -- Choose greenery disclaimer to show
  local hide_greeneries = {seed=true, bud=true, ripe=true}
  hide_greeneries[greenery_kind] = false
  for k, v in pairs(hide_greeneries) do
      if v then
          local node = template:find('.greenery.'..k)
          node:delete_children()
          node:set_attr('class', '')
          node:set_attr('style', 'display:none')
      end
  end

  -- FIXME: fix relative links - strip .md etc.
  -- TODO: copy images, css

  return template
end

local function render_index_item_tags(item_tmpl, art)
  local tag_tmpl = item_tmpl:find('ul.tags'):eject_children()
  for _, tag in ipairs(art.tags) do
    -- print(tag_tmpl:to_string())
    tag_tmpl:find('li a'):set_text('@'..tag)
    tag_tmpl:find('li a'):set_attr('href', '@'..tag)
    item_tmpl:find('ul.tags'):add_children(tag_tmpl)
  end
end

local function render_snip_entry(snip_tmpl, art)
  local item_slot = snip_tmpl:find('li')
  local item = item_slot:eject_children()
  item_slot:set_children(art.html)

  -- delete <p> in the template (TODO: add :delete() method)
  item:find('p'):set_text('')
  item:find('p'):set_attr('style', 'display:none')

  render_index_item_tags(item, art)

  local self_link_slot = item:find('a.self-link')
  self_link_slot:set_text('♧ ' .. article_date(art))
  self_link_slot:set_attr('href', art.slug)

  item_slot:add_children(item)
  return snip_tmpl
end

local function render_index_entry(art_tmpl, art)
  local h1 = art.html:find 'h1'
  if not h1 then
    return nil
  end

  local title_slot = art_tmpl:find('a.title')
  title_slot:set_children(h1)
  local greenery_kind =
    art.tags.seed and 'seed' or
    art.tags.bud and 'bud' or
    art.tags.ripe and 'ripe' or
    ''
  -- TODO: should use :get_attr() instead of hardcoding 'title '
  title_slot:set_attr('class', 'title '..greenery_kind)
  title_slot:set_attr('href', art.slug)

  art_tmpl:find('time'):set_text(article_date(art))

  render_index_item_tags(art_tmpl, art)

  return art_tmpl
end

local function render_articles_list(list_slot, articles)
  local item_templates = list_slot:eject_children()

  local art_tmpl = item_templates:find('#article-wrapper'):eject_children()
  local snips_tmpl = item_templates:find('#snips-wrapper'):eject_children()
  local snip_tmpl = snips_tmpl:find('ul.snips'):eject_children()

  local snips = nil
  for _, art in ipairs(articles) do
    if art.tags.snip then
      snips = snips or snips_tmpl:clone()
      local entry = render_snip_entry(snip_tmpl:clone(), art)
      snips:find('ul.snips'):add_children(entry)
    else -- not snip
      if snips then
        list_slot:add_children(snips)
        snips = nil
      end
      local entry = render_index_entry(art_tmpl:clone(), art)
      if entry then
        list_slot:add_children(entry)
      end
    end
  end
  if snips then
    list_slot:add_children(snips)
    snips = nil
  end
end

local function render_tag(tag_tmpl, tag, articles)
  -- Set title
  local title = '@' .. tag .. TITLE_SUFFIX
  tag_tmpl:find('html head title'):set_text(title)

  -- Set tag name in h1
  tag_tmpl:find('#tag'):set_text('@'..tag)

  -- Build list of articles in tag
  local list_slot = tag_tmpl:find '#articles'
  render_articles_list(list_slot, articles)
  return tag_tmpl
end

local function render_index(filename, articles, modifer_f)
  print('INDEX ' .. filename)
  local index = html.parse(readfile '_bloat/index.html')
  local list_slot = index:find '#list'
  render_articles_list(list_slot, articles)
  if modifer_f then
    modifer_f(index)
  end
  writefile(filename, index:to_string())
end

local function main()
  -- Sort articles, newest first.
  table.sort(articles, function(a, b) return a.datetime > b.datetime end)

  ------
  -- Render articles.
  ------
  local template = html.parse(readfile '_bloat/bloat.html')
  for _, article in ipairs(articles) do
    print("RENDERING " .. article.slug)
    for _, t in ipairs(article.tags) do
      article.tags[t] = true
    end
    local render = render_article(template:clone(), article)
    writefile('_html.out/'..article.slug, render:to_string())
  end

  ------
  -- Render tags index pages.
  ------
  local tags = {}
  for _, article in ipairs(articles) do
    for _, v in ipairs(article.tags) do
      local t = tags[v] or {}
      t[#t+1] = article
      tags[v] = t
    end
  end
  local tag_tmpl = html.parse(readfile '_bloat/tag.html')
  for tag, arts in pairs(tags) do
    print("TAG @" .. tag)
    local render = render_tag(tag_tmpl:clone(), tag, arts)
    writefile('_html.out/@'..tag, render:to_string())
  end

  ------
  -- Render main index pages (tabs/areas)
  ------
  local platter_articles = {}
  for _, a in ipairs(articles) do
    if a.tags.ripe or a.tags.bud then
      platter_articles[#platter_articles+1] = a
    end
  end
  render_index('_html.out/index.html', platter_articles)
  render_index('_html.out/@backstage', articles, function(tmpl)
    tmpl:find('#areas > li.current'):set_attr('class', '')
    tmpl:find('#areas > li + li'):set_attr('class', 'current')
  end)
  render_index('_html.out/about', {}, function(tmpl)
    tmpl:find('#areas > li.current'):set_attr('class', '')
    tmpl:find('#areas > li + li + li'):set_attr('class', 'current')
    local text = html.from_md(readfile('@seed/0000-about.md'))
    tmpl:find('#list'):delete_children()
    tmpl:find('#list + small'):delete_children()
    tmpl:find('article#text'):add_children(text)
    tmpl:find('article#text > h1'):delete_children()
    tmpl:find('article#text > h1'):set_attr('style', 'display:none')
  end)
end

main()
