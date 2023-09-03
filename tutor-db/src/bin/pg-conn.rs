use chrono::NaiveDateTime;
use sqlx::postgres::PgPoolOptions;
use std::error::Error;

#[derive(Debug)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgresql://root:root@localhost:5432/ezytutors";
    let db_pool = PgPoolOptions::new().max_connections(5).connect(url).await?;

    sqlx::migrate!("./migrations").run(&db_pool).await?;

    let course_rows = sqlx::query_as!(
        Course,
        "SELECT course_id, tutor_id, course_name, posted_time 
FROM ezy_course_c4 
WHERE course_id = $1",
        1
    )
    .fetch_all(&db_pool)
    .await?;

    println!("{course_rows:?}");

    let mut courses_list = Vec::new();

    for row in course_rows {
        courses_list.push(Course {
            course_id: row.course_id,
            tutor_id: row.tutor_id,
            course_name: row.course_name,
            posted_time: Some(chrono::NaiveDateTime::from(row.posted_time.unwrap())),
        })
    }
    println!("Course with id 1 = {:?}", courses_list);

    Ok(())
}
