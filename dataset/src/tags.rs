use std::error::Error;

use rusqlite::Connection;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{loader::Loader, options::TagsModeOptions};

async fn generate_filtered_sql_via_memory(
    artworks_db_path: &String,
    tag_vector: &Vec<(u64, Vec<String>)>,
    output_path: &String,
) -> Result<(), Box<dyn Error>> {
    let source_conn = Connection::open(artworks_db_path)?;
    let mut stmt = source_conn.prepare("SELECT id, danbooru_id FROM artworks")?;

    let mut danbooru_to_local_id = std::collections::HashMap::new();
    let rows = stmt.query_map([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)))?;
    for row in rows {
        let (local_id, danbooru_id) = row?;
        danbooru_to_local_id.insert(danbooru_id, local_id);
    }

    let mut mem_conn = Connection::open_in_memory()?;

    mem_conn.execute_batch(
        "CREATE TABLE artworks (
            id INTEGER PRIMARY KEY,
            danbooru_id INTEGER UNIQUE
        );
        CREATE TABLE tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tag_name TEXT UNIQUE
        );
        CREATE TABLE artwork_tag (
            artwork_id INTEGER,
            tag_id INTEGER,
            FOREIGN KEY(artwork_id) REFERENCES artworks(id),
            FOREIGN KEY(tag_id) REFERENCES tags(id)
        );",
    )?;

    {
        let tx = mem_conn.transaction()?;

        {
            let mut stmt = tx.prepare("INSERT INTO artworks (id, danbooru_id) VALUES (?1, ?2)")?;
            for (&danbooru_id, &local_id) in &danbooru_to_local_id {
                stmt.execute(rusqlite::params![local_id as i64, danbooru_id as i64])?;
            }
        }

        {
            let mut insert_tag = tx.prepare("INSERT OR IGNORE INTO tags (tag_name) VALUES (?1)")?;
            let mut get_tag_id = tx.prepare("SELECT id FROM tags WHERE tag_name = ?1")?;
            let mut insert_link =
                tx.prepare("INSERT INTO artwork_tag (artwork_id, tag_id) VALUES (?1, ?2)")?;

            for (danbooru_id, tags) in tag_vector {
                if let Some(&local_artwork_id) = danbooru_to_local_id.get(&(*danbooru_id as i64)) {
                    for tag_name in tags {
                        insert_tag.execute(rusqlite::params![tag_name])?;
                        let tag_id: i64 =
                            get_tag_id.query_row(rusqlite::params![tag_name], |row| row.get(0))?;
                        insert_link.execute(rusqlite::params![local_artwork_id, tag_id])?;
                    }
                }
            }
        }

        tx.commit()?;
    }

    mem_conn.execute(
        "DELETE FROM tags WHERE id IN (
            SELECT t.id FROM tags t
            LEFT JOIN artwork_tag at ON t.id = at.tag_id
            GROUP BY t.id
            HAVING COUNT(at.artwork_id) < 100
        );",
        [],
    )?;

    mem_conn.execute(
        "DELETE FROM artwork_tag WHERE tag_id NOT IN (SELECT id FROM tags);",
        [],
    )?;

    mem_conn.execute(
        "DELETE FROM artworks WHERE id IN (
            SELECT a.id FROM artworks a
            LEFT JOIN artwork_tag at ON a.id = at.artwork_id
            GROUP BY a.id
            HAVING COUNT(at.tag_id) < 5
        );",
        [],
    )?;

    mem_conn.execute(
        "DELETE FROM artwork_tag WHERE artwork_id NOT IN (SELECT id FROM artworks);",
        [],
    )?;

    let mut file = File::options()
        .create(true)
        .write(true)
        .append(true)
        .open(output_path)
        .await?;

    {
        let mut stmt = mem_conn.prepare("SELECT tag_name FROM tags")?;
        let tags_iter = stmt.query_map([], |row| row.get::<_, String>(0))?;
        for tag_res in tags_iter {
            let tag = tag_res?;
            let escaped = tag.replace('\'', "''");
            let line = format!(
                "INSERT OR IGNORE INTO tags (tag_name) VALUES ('{}');\n",
                escaped
            );
            file.write_all(line.as_bytes()).await?;
        }
    }

    file.write_all(b"\n").await?;

    {
        let mut stmt = mem_conn.prepare(
            "SELECT at.artwork_id, t.tag_name
             FROM artwork_tag at
             JOIN tags t ON t.id = at.tag_id",
        )?;
        let links_iter = stmt.query_map([], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })?;

        for link_res in links_iter {
            let (art_id, tag_name) = link_res?;
            let escaped_tag = tag_name.replace('\'', "''");
            let line = format!(
                "INSERT INTO artwork_tag (artwork_id, tag_id) SELECT {}, id FROM tags WHERE tag_name = '{}';\n",
                art_id, escaped_tag
            );
            file.write_all(line.as_bytes()).await?;
        }
    }

    file.flush().await?;

    Ok(())
}

fn get_artwork_ids(connection_string: &String) -> Result<Vec<u64>, Box<dyn Error>> {
    let conn = Connection::open(connection_string)?;
    let mut stmt = conn.prepare("SELECT danbooru_id FROM artworks")?;
    let ids_iter = stmt.query_map([], |row| row.get::<_, i64>(0).map(|id| id as u64))?;

    let mut ids = Vec::new();
    for id_result in ids_iter {
        ids.push(id_result?);
    }
    Ok(ids)
}

pub async fn run(
    options: &TagsModeOptions,
    loader: &Loader,
    output: &String,
) -> Result<(), Box<dyn Error>> {
    let ids = get_artwork_ids(&options.database)?;

    let tag_vector = loader.load_tags_by_ids(ids.as_slice()).await?;

    generate_filtered_sql_via_memory(&options.database, &tag_vector, output).await?;

    Ok(())
}
