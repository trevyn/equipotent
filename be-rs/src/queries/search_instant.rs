use common_rs::*;
use turbosql::select;

pub async fn search_instant(query: String) -> anyhow::Result<Vec<SearchQueryResultItem>> {
 let match_query = super::convert_query_for_fts5(query);
 // let query = convert_query_for_fts5(query.clone()).split(" ").collect::<Vec<_>>().join(" OR ");

 log::info!("match_query = {:?}", match_query);

 Ok(select!(Vec<SearchQueryResultItem> r#"
  search_highlighted_url,
  title,
  snippet,
  sq.url AS url,
  sq.host AS host,
  bookmark.url IS NOT NULL AS bookmarked,
  IFNULL(hostaffection.affection, 0) AS hostaffection,
  MIN(sq.rank) AS rank

  FROM (
   SELECT
    highlight(resultitem_fts, 1, 'EQUIPOTENTHIGHLIGHTOPEN', 'EQUIPOTENTHIGHLIGHTCLOSE') AS search_highlighted_url,
    highlight(resultitem_fts, 2, 'EQUIPOTENTHIGHLIGHTOPEN', 'EQUIPOTENTHIGHLIGHTCLOSE') AS title,
    highlight(resultitem_fts, 3, 'EQUIPOTENTHIGHLIGHTOPEN', 'EQUIPOTENTHIGHLIGHTCLOSE') AS snippet,
    url,
    host,
    rank
    FROM resultitem_fts
    WHERE resultitem_fts MATCH ?
    LIMIT -1 OFFSET 0  -- prevents "unable to use function highlight in the requested context"
  ) sq
  LEFT JOIN bookmark ON sq.url = bookmark.url
  LEFT JOIN hostaffection ON sq.host = hostaffection.host
  GROUP BY sq.url
  ORDER BY bookmarked DESC, hostaffection DESC, MIN(sq.rank)
  LIMIT 30
 "#, match_query)?)
}
