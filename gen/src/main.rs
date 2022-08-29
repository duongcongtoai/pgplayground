use csv::Writer;
use std::error::Error;

fn main() {
    noti_filter_complex();
}

fn genstu() {
    let headers = ["student_id,grade,name"];
}

fn noti_filter_complex() {
    let headers = ["", "", ""];
    let mut wtr = Writer::from_path("foo.csv").unwrap();

    // join course_id, student_id
    // student_id, name, grade
    // student_id, parent_id
    //
    // noti_id,correlation_id, user_id, status(read/unread), created_at
    // noti_id,content,title,sent_at,scheduled_at, type, created_at

    wtr.write_record(&headers).unwrap();
    wtr.write_record(&["a", "b", "c"]).unwrap();
    wtr.write_record(&["x", "y", "z"]).unwrap();
    wtr.flush().unwrap();
    wtr.write_record(&["x", "y", "z"]).unwrap();
    wtr.flush().unwrap();
}

// info_notifications
// info_notification_msgs
//

/* SELECT
  ifn.notification_id,
  ifn.notification_msg_id,
  ifn.type,
  ifn.data,
  ifn.editor_id,
  ifn.target_groups,
  ifn.receiver_ids,
  ifn.event,
  ifn.status,
  ifn.scheduled_at,
  ifn.owner,
  ifn.is_important,
  ifn.questionnaire_id,
  ifn.created_at,
  ifn.updated_at,
  ifn.deleted_at,
  ifn.sent_at,
  ifn.created_user_id,
  ifn.excluded_generic_receiver_ids,
  ifn.generic_receiver_ids
FROM
  info_notifications ifn
INNER JOIN
  info_notification_msgs ifn_msgs
ON
  ifn_msgs.notification_msg_id = ifn.notification_msg_id
WHERE
  ($1::TEXT[] IS NULL
    OR ifn.notification_id = ANY($1))
  AND ($2::TEXT IS NULL
    OR ifn_msgs.title LIKE CONCAT($10, $2::TEXT, $11))
  AND ($3::TEXT[] IS NULL
    OR ifn.status = ANY($3))
  AND (
    CASE
      WHEN $4::TIMESTAMPTZ IS NULL AND $5::TIMESTAMPTZ IS NULL THEN $12
      WHEN $4::TIMESTAMPTZ IS NULL THEN ifn.scheduled_at <= $5
      WHEN $5::TIMESTAMPTZ IS NULL THEN ifn.scheduled_at >= $4
    ELSE
    ifn.scheduled_at BETWEEN $4
    AND $5
  END
    )
  AND (
    CASE
      WHEN $6::TIMESTAMPTZ IS NULL AND $7::TIMESTAMPTZ IS NULL THEN $13
      WHEN $6::TIMESTAMPTZ IS NULL THEN ifn.s */

// study_plan_items
// student_study_plans
// study_plans
/* SELECT
FROM
  study_plan_items AS i
INNER JOIN
  student_study_plans s
ON
  i.study_plan_id = s.study_plan_id
JOIN
  study_plans sp
ON
  sp.study_plan_id = s.study_plan_id
JOIN
  books b
ON
  b.book_id = i.content_structure ->> $10
JOIN
  chapters c
ON
  c.chapter_id = i.content_structure ->> $11
JOIN
  topics t
ON
  t.topic_id = i.content_structure ->> $12
WHERE
  s.student_id = $1
  AND i.start_date < NOW()
  AND (($2::timestamp IS NULL
      OR $7::integer IS NULL
      OR $8::text IS NULL)
    OR ((i.start_date,
        i.display_order,
        i.study_plan_item_id) > ($2,
        $7,
        $8)))
  AND (($3 BETWEEN i.available_from
      AND i.available_to)
    OR (i.available_from <= $3
      AND i.available_to IS NULL))
  AND (i.end_date IS NULL
    OR i.end_date >= $3)
  AND ($4::TEXT[] IS NULL
    OR sp.course_id = ANY($4))
  AND ($5: */
