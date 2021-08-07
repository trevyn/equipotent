mod openai;
mod search_instant;
mod search_scrape;

pub use openai::openai;
pub use search_instant::search_instant;
pub use search_scrape::search_scrape;

fn convert_query_for_fts5(query: String) -> String {
 // some punctuation breaks it
 let mut query = query;
 query = query.replace(".", " ");
 query = query.replace("'", " ");
 query = query.replace(",", " ");
 query = query.replace("-", " ");

 // quotes must be balanced to work, so add a fake one at the end while we're typing
 if query.matches('"').count() % 2 == 1 {
  query.push('"');
 };

 // if there's no " on the end, use SQLite prefix search
 if let Some('"') = query.clone().pop() {
 } else {
  query.push('*');
 }

 query
}
