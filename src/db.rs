use crate::global::*;

use markov_chains::prelude::GenerationParams;

use rusqlite::{Connection, Result};

pub fn create_db() -> Result<()> {
    if !std::path::Path::new(DB_NAME).exists() {
        let conn = Connection::open(DB_NAME)?;
        conn.execute(
            "CREATE TABLE models (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                version TEXT NOT NULL,
                description TEXT
            );
            CREATE TABLE users (
                id INTEGER PRIMARY KEY,
                loaded INTEGER NOT NULL
            );
            CREATE TABLE user_params (
                user_id INTEGER NOT NULL,

                temp REAL NOT NULL,
                temp_a REAL NOT NULL,
                pen REAL NOT NULL,
                pen_w INTEGER NOT NULL,
                k REAL NOT NULL,
                min_len INTEGER NOT NULL,
                max_len INTEGER NOT NULL,
                no_bigrams BOOLEAN NOT NULL,
                no_trigrams BOOLEAN NOT NULL,

                FOREIGN KEY (user_id) REFERENCES users(id)
            );",
            (),
        )?;
    }

    Ok(())
}


pub fn add_model(id: u64, name: String) -> Result<()> {
    let conn = Connection::open(DB_NAME)?;

    conn.execute(
        "INSERT INTO models (id, name, version) VALUES (?, ?, ?)",
        (id, name, MARKOV_CHAINS_VERSION.to_string()),
    )?;

    Ok(())
}

pub fn model_get_ids() -> Result<Vec<u8>> {
    let conn = Connection::open(DB_NAME)?;

    let mut stmt = conn.prepare("SELECT id FROM models")?;

    let models = stmt
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<u8>>>()?;

    Ok(models)
}

pub fn model_get_id(name: String) -> Result<u64> {
    let conn = Connection::open(DB_NAME)?;

    let mut stmt = conn.prepare("SELECT id FROM models WHERE name = ?")?;

    let id = stmt
        .query_map([name], |row| row.get(0))?
        .next()
        .unwrap()?;

    Ok(id)
}

pub fn model_get_name(id: u64) -> Result<String> {
    let conn = Connection::open(DB_NAME)?;

    let mut stmt = conn.prepare("SELECT name FROM models WHERE id = ?")?;

    let name = stmt
        .query_map([id], |row| row.get(0))?
        .next()
        .unwrap()?;

    Ok(name)
}

/*
pub fn server_add_model(server: u64, model: u64) -> Result<()> {
    let conn = Connection::open(DB_NAME)?;

    conn.execute(
        "INSERT INTO server_models (server_id, model_id) VALUES (?, ?)",
        [server, model],
    )?;

    Ok(())
}

pub fn server_get_models(server: u64) -> Result<Vec<u64>> {
    let conn = Connection::open(DB_NAME)?;

    let mut stmt = conn.prepare("SELECT model_id FROM server_models WHERE server_id = ?")?;
    let models = stmt
        .query_map([server], |row| row.get(0))?
        .collect::<Result<Vec<u64>, _>>()?;

    Ok(models)
} */

pub fn add_user(id: u64) -> Result<()> {
    let conn = Connection::open(DB_NAME)?;

    conn.execute(
        "INSERT INTO users (id, loaded) VALUES (?, ?)",
        (id, DEFAULT_MODEL_ID),
    )?;

    let d = GenerationParams::default();

    conn.execute(
        "INSERT INTO user_params (user_id, temp, temp_a, pen, pen_w, k, min_len, max_len, no_bigrams, no_trigrams) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        (id, d.temperature, d.temperature_alpha, d.repeat_penalty, d.repeat_penalty_window, d.k_normal, d.min_len, d.max_len, d.no_bigrams, d.no_trigrams)
    )?;

    Ok(())
}

pub fn user_get_loaded(user: u64) -> Result<u64> {
    let conn = Connection::open(DB_NAME)?;

    let mut stmt = conn.prepare("SELECT loaded FROM users WHERE id = ?")?;

    let loaded = stmt.query_map([user], |row| row.get(0))?
        .next()
        .unwrap()?;

    Ok(loaded)
}

pub fn user_set_loaded(user: u64, loaded: u64) -> Result<()> {
    let conn = Connection::open(DB_NAME)?;

    conn.execute(
        "UPDATE users SET loaded = ? WHERE id = ?",
        (loaded, user),
    )?;

    Ok(())
}

pub fn user_get_params(user: u64) -> Result<GenerationParams> {
    let conn = Connection::open(DB_NAME)?;

    let mut stmt = conn.prepare("SELECT temp, temp_a, pen, pen_w, k, min_len, max_len, no_bigrams, no_trigrams FROM user_params WHERE user_id = ?")?;

    let params = stmt.query_map([user], |row| {
        Ok(GenerationParams {
            temperature: row.get(0)?,
            temperature_alpha: row.get(1)?,
            repeat_penalty: row.get(2)?,
            repeat_penalty_window: row.get(3)?,
            k_normal: row.get(4)?,
            min_len: row.get(5)?,
            max_len: row.get(6)?,
            no_bigrams: row.get(7)?,
            no_trigrams: row.get(8)?,
        })
    })?
    .next()
    .unwrap()?;

    Ok(params)
}

pub fn user_set_params(user: u64, params: GenerationParams) -> Result<()> {
    let conn = Connection::open(DB_NAME)?;

    conn.execute(
        "UPDATE user_params SET temp = ?, temp_a = ?, pen = ?, pen_w = ?, k = ?, min_len = ?, max_len = ?, no_bigrams = ?, no_trigrams = ? WHERE user_id = ?",
        (params.temperature, params.temperature_alpha, params.repeat_penalty, params.repeat_penalty_window, params.k_normal, params.min_len, params.max_len, params.no_bigrams, params.no_trigrams, user)
    )?;

    Ok(())
}

// Get the model headers and size for each model
/*
pub fn get_models_info(models: Vec<u64>) -> Result<(Vec<Vec<String>>)> {
    let conn = Connection::open(DB_NAME)?;

    let mut stmt = conn.prepare("SELECT name, version, description FROM models WHERE id = ?")?;
    let mut info = Vec::new();

    for model in models {
        if let Ok(metadata) =

        info.push(stmt.query_map([model], |row| {
            Ok(vec![
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
            ])
        })?.next().unwrap()?);
    }

    Ok(info)
}
*/
