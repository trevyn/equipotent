use common_rs::*;
use turbosql::{execute, select};

pub async fn search_scrape(
 query: &str,
 items: Vec<ResultItem>,
) -> anyhow::Result<Vec<SearchQueryResultItem>> {
 ResultItem::insert_batch(&items);

 for item in &items {
  execute!(
   "INSERT INTO resultitem_fts(url, title, snippet, host) VALUES (?, ?, ?, ?)",
   item.url,
   item.title,
   item.snippet,
   item.host
  )?;
 }

 // re-do search against database

 let match_query =
  super::convert_query_for_fts5(query.to_string()).split(' ').collect::<Vec<_>>().join(" OR ");

 log::info!("match_query = {:?}", match_query);

 Ok(select!(Vec<SearchQueryResultItem> r#"
  search_highlighted_url,
  sq.title AS title,
  sq.snippet AS snippet,
  sq.url AS url,
  resultitem.host AS host,
  bookmark.url IS NOT NULL AS bookmarked,
  IFNULL(hostaffection.affection, 0) AS hostaffection,
  MIN(resultitem.source_result_pos) AS rank
  FROM (
   SELECT highlight(resultitem_fts, 1, 'EQUIPOTENTHIGHLIGHTOPEN', 'EQUIPOTENTHIGHLIGHTCLOSE') AS search_highlighted_url,
   highlight(resultitem_fts, 2, 'EQUIPOTENTHIGHLIGHTOPEN', 'EQUIPOTENTHIGHLIGHTCLOSE') AS title,
   highlight(resultitem_fts, 3, 'EQUIPOTENTHIGHLIGHTOPEN', 'EQUIPOTENTHIGHLIGHTCLOSE') AS snippet,
   url
   FROM resultitem_fts(?)
   WHERE resultitem_fts.url IN (SELECT DISTINCT url FROM resultitem WHERE source_query = ?)
   LIMIT -1 OFFSET 0  -- prevents "unable to use function highlight in the requested context"
  ) sq
  LEFT JOIN resultitem ON resultitem.url = sq.url AND resultitem.source_query = ?
  LEFT JOIN bookmark ON sq.url = bookmark.url
  LEFT JOIN hostaffection ON resultitem.host = hostaffection.host
  GROUP BY sq.url
  ORDER BY bookmarked DESC, hostaffection DESC, rank
  LIMIT 30
 "#,  match_query, query, query)?)
}
