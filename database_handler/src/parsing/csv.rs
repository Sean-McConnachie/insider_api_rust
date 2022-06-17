use std::str::FromStr;
use crate::database_errors::DbError;


pub fn csv_to_type<T: FromStr>(values: Option<String>, parameter_name: &str) -> Result<Vec<T>, DbError> {
    Ok(match values {
        Some(v) => {
            if v.is_empty() {
                vec![]
            } else {
                match v
                    .split(',')
                    .map(|s| s.parse::<T>())
                    .collect::<Result<Vec<T>, _>>()
                {
                    Ok(v) => v,
                    Err(_) => return Err(DbError::ParseError(format!("Failed to parse parameter `{}`", parameter_name))),
                }
            }
        },
        None => vec![]
    })
}

pub fn convert_to_type<T: FromStr>(value: String, parameter_name: &str) -> Result<T, DbError> {
    match value.parse::<T>() {
        Ok(v) => Ok(v),
        Err(_) => Err(DbError::ParseError(format!("Failed to parse parameter `{}`", parameter_name)))
    }
}

pub fn date_option(value: String, parameter_name: &str) -> Result<Option<i64>, DbError> {
    if value.is_empty() {
        Ok(None)
    } else {
        Ok(Some(convert_to_type(value, parameter_name)?))
    }

}