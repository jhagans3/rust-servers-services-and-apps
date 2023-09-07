use crate::models::Course;
use actix_web::web::Json;

impl From<Json<Course>> for Course {
    fn from(course: Json<Course>) -> Self {
        Course {
            tutor_id: course.tutor_id,
            course_id: course.course_id,
            course_name: course.course_name.clone(),
            posted_time: course.posted_time,
        }
    }
}
