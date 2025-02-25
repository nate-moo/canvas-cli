use super::Course;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Submission {
    pub assignment_id: u64,
    pub course: Option<Course>,
    pub attempt: u64,
    pub body: Option<String>,
    pub grade: String,
    pub grade_matches_current_submission: bool,
    pub html_url: String,
    pub preview_url: String,
    pub score: f32,
    // pub submission_comments: Option<Vec<SubmissionComment>>,
    pub submission_type: SubmissionType,
    pub submitted_at: DateTime<Utc>,
    pub url: Option<String>,
    pub user_id: u64,
    pub grader_id: Option<u64>,
    pub graded_at: Option<DateTime<Utc>>,
    pub late: bool,
    pub assignment_visible: bool,
    pub excused: bool,
    pub missing: bool,
    // pub late_policy_status: LateStatus,
    pub points_deducted: f32,
    pub seconds_late: u64,
}

impl crate::types::ResponseType for Submission {}

impl std::cmp::PartialEq for Submission {
    fn eq(&self, other: &Self) -> bool {
        self.assignment_id == other.assignment_id && self.attempt == other.attempt
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubmissionType {
    DiscussionTopic,
    OnlineQuiz,
    OnPaper,
    None,
    ExternalTool,
    OnlineTextEntry,
    OnlineUrl,
    OnlineUpload,
    MediaRecording,
    StudentAnnotation,
    NotGraded,
}

#[derive(Debug, Deserialize)]
pub struct SubmissionComment; // TODO: Expand
#[derive(Debug, Deserialize)]
pub struct LateStatus; // TODO: Expand
