//! This is the main file for the frontend. It will render the App component.

pub mod app;
pub mod login_form;
pub use login_form::LoginForm;
pub use app::App;
pub mod error_page;
pub use error_page::ErrorPage;
pub mod comment_popup;
pub use comment_popup::CommentPopup;
pub mod comment;
pub use comment::Comment;
pub mod comments_dashboard;
pub use comments_dashboard::CommentsDashboard;