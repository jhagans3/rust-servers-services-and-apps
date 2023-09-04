use crate::models::Course;
use sqlx::PgPool;

pub async fn get_courses_for_tutor_db(pool: &PgPool, tutor_id: i32) -> Vec<Course> {
    sqlx::query_as!(
        Course,
        "SELECT course_id, tutor_id, course_name, posted_time 
FROM ezy_course_c4 
WHERE tutor_id = $1",
        tutor_id
    )
    .fetch_all(pool)
    .await
    .unwrap()
}

pub async fn get_course_details_db(pool: &PgPool, tutor_id: i32, course_id: i32) -> Course {
    sqlx::query_as!(
        Course,
        "SELECT course_id, tutor_id, course_name, posted_time 
FROM ezy_course_c4 
WHERE course_id = $1 and tutor_id = $2",
        course_id,
        tutor_id
    )
    .fetch_one(pool)
    .await
    .unwrap()
}

pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Course {
    sqlx::query_as!(
        Course,
        "INSERT INTO ezy_course_c4 (course_id, tutor_id, course_name)
	VALUES ($1, $2, $3)
	RETURNING tutor_id, course_id,course_name, posted_time",
        new_course.course_id,
        new_course.tutor_id,
        new_course.course_name,
    )
    .fetch_one(pool)
    .await
    .unwrap()
}
