//! `.toon` seed loader (SPEC C11 / T33).
//!
//! Deliberately tiny: TOON is a tabular block format, and the only thing we need
//! from it is "rows of strings under named columns". No escaping, no nesting, no
//! numbers-with-commas — if a value needs a comma, don't put it in a seed.
//!
//! ```toon
//! categories[2]{slug,name}:
//!   abarrotes,Abarrotes
//!   bebidas,Bebidas
//!
//! products[1]{sku,name,price_cents,stock,min_stock,category_slug}:
//!   A001,Coca-Cola 350ml,350000,48,10,bebidas
//! ```

use std::sync::Arc;

use surrealdb::engine::local::Db;
use surrealdb::Surreal;

/// One `name[n]{cols}:` block.
#[derive(Debug, Clone, PartialEq)]
pub struct Section {
    pub table: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl Section {
    fn col<'a>(&self, row: &'a [String], name: &str) -> Result<&'a str, SeedError> {
        let idx = self
            .columns
            .iter()
            .position(|c| c == name)
            .ok_or_else(|| SeedError(format!("{}: missing column `{name}`", self.table)))?;
        row.get(idx)
            .map(|s| s.as_str())
            .ok_or_else(|| SeedError(format!("{}: short row {row:?}", self.table)))
    }
}

#[derive(Debug, thiserror::Error)]
#[error("seed: {0}")]
pub struct SeedError(String);

/// Parse a `.toon` document into sections. Blank lines and `#` comments are ignored.
pub fn parse(src: &str) -> Result<Vec<Section>, SeedError> {
    let mut out: Vec<Section> = Vec::new();

    for (n, raw) in src.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        match line.strip_suffix(':') {
            // Header: `table[count]{col,col}:`
            Some(head) if head.contains('{') => {
                let (lhs, cols) = head
                    .split_once('{')
                    .ok_or_else(|| SeedError(format!("line {}: bad header", n + 1)))?;
                let cols = cols
                    .strip_suffix('}')
                    .ok_or_else(|| SeedError(format!("line {}: unbalanced {{ }}", n + 1)))?;
                let (table, count) = lhs
                    .split_once('[')
                    .ok_or_else(|| SeedError(format!("line {}: missing [count]", n + 1)))?;
                let count: usize = count
                    .trim_end_matches(']')
                    .parse()
                    .map_err(|_| SeedError(format!("line {}: bad count", n + 1)))?;
                out.push(Section {
                    table: table.trim().to_string(),
                    columns: cols.split(',').map(|c| c.trim().to_string()).collect(),
                    rows: Vec::with_capacity(count),
                });
            }
            // Row
            _ => {
                let section = out
                    .last_mut()
                    .ok_or_else(|| SeedError(format!("line {}: row before header", n + 1)))?;
                let row: Vec<String> = line.split(',').map(|v| v.trim().to_string()).collect();
                if row.len() != section.columns.len() {
                    return Err(SeedError(format!(
                        "line {}: {} values for {} columns",
                        n + 1,
                        row.len(),
                        section.columns.len()
                    )));
                }
                section.rows.push(row);
            }
        }
    }

    Ok(out)
}

/// Write every section into the database. Idempotent: `CREATE` is skipped when
/// the record already exists (re-running seeds is safe).
pub async fn apply(db: &Surreal<Db>, sections: &[Section]) -> Result<usize, SeedError> {
    let mut written = 0;

    for s in sections {
        for row in &s.rows {
            match s.table.as_str() {
                "categories" => {
                    let slug = s.col(row, "slug")?;
                    let name = s.col(row, "name")?;
                    ignore_exists(
                        db.query(
                            "CREATE type::record('category', $slug) SET slug = $slug, name = $name;",
                        )
                        .bind(("slug", slug.to_string()))
                        .bind(("name", name.to_string()))
                        .await,
                    )?;
                }
                "products" => {
                    let sku = s.col(row, "sku")?;
                    let name = s.col(row, "name")?;
                    let price: i64 = num(s.col(row, "price_cents")?)?;
                    let stock: i32 = num(s.col(row, "stock")?)?;
                    let min_stock: i32 = s.col(row, "min_stock").and_then(num).unwrap_or(0);
                    let category = s.col(row, "category_slug").unwrap_or("").to_string();

                    // Two static statements — no value is interpolated into the query text.
                    let (sql, category) = if category.is_empty() {
                        (
                            "CREATE type::record('product', $sku) SET sku = $sku, name = $name, \
                             price = $price, stock = $stock, min_stock = $min_stock;",
                            String::new(),
                        )
                    } else {
                        (
                            "CREATE type::record('product', $sku) SET sku = $sku, name = $name, \
                             price = $price, stock = $stock, min_stock = $min_stock, \
                             category = type::record('category', $category);",
                            category,
                        )
                    };
                    if category.is_empty() {
                        ignore_exists(
                            db.query(sql)
                                .bind(("sku", sku.to_string()))
                                .bind(("name", name.to_string()))
                                .bind(("price", price))
                                .bind(("stock", stock))
                                .bind(("min_stock", min_stock))
                                .await,
                        )?;
                    } else {
                        ignore_exists(
                            db.query(sql)
                                .bind(("sku", sku.to_string()))
                                .bind(("name", name.to_string()))
                                .bind(("price", price))
                                .bind(("stock", stock))
                                .bind(("min_stock", min_stock))
                                .bind(("category", category))
                                .await,
                        )?;
                    }
                }
                other => {
                    return Err(SeedError(format!(
                        "unknown seed table `{other}` (known: categories, products)"
                    )));
                }
            }
            written += 1;
        }
    }

    Ok(written)
}

/// Load a `.toon` file into a runtime handle.
pub async fn apply_file(db: &Arc<Surreal<Db>>, src: &str) -> Result<usize, SeedError> {
    apply(db.as_ref(), &parse(src)?).await
}

/// Seeds are idempotent: an already-existing record is not an error.
fn ignore_exists<T, E: std::fmt::Display>(res: Result<T, E>) -> Result<(), SeedError> {
    match res {
        Ok(_) => Ok(()),
        Err(e) if e.to_string().contains("already exists") => Ok(()),
        Err(e) => Err(SeedError(e.to_string())),
    }
}

fn num<T: std::str::FromStr>(s: &str) -> Result<T, SeedError> {
    s.parse()
        .map_err(|_| SeedError(format!("{s:?} is not a number")))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SRC: &str = "\
# demo
categories[2]{slug,name}:
  abarrotes,Abarrotes
  bebidas,Bebidas

products[2]{sku,name,price_cents,stock,min_stock,category_slug}:
  A001,Coca-Cola 350ml,350000,48,10,bebidas
  A002,Agua Cristal 600ml,250000,120,0,bebidas
";

    #[test]
    fn parses_two_sections() {
        let s = parse(SRC).unwrap();
        assert_eq!(s.len(), 2);
        assert_eq!(s[0].table, "categories");
        assert_eq!(s[0].columns, ["slug", "name"]);
        assert_eq!(s[0].rows.len(), 2);
        assert_eq!(s[1].table, "products");
        assert_eq!(s[1].columns.len(), 6);
    }

    #[test]
    fn rejects_row_width_mismatch() {
        let bad = "products[1]{sku,name}:\n  only-one\n";
        assert!(parse(bad).is_err());
    }

    #[test]
    fn rejects_row_before_header() {
        assert!(parse("  orphan,row\n").is_err());
    }

    #[test]
    fn rejects_bad_count() {
        assert!(parse("products[x]{sku}:\n  A1\n").is_err());
    }
}
