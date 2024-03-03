mod domain;

pub struct User {
    email: String,
    token: String,
    username: String,
    bio: Option<String>,
    image: Option<String>,
    followers: Option<Vec<String>>,
    following: Option<Vec<String>>,
}

pub struct UserResponse {
    email: String,
    token: String,
    username: String,
    bio: Option<String>,
    image: Option<String>,
}

pub struct AuthenticateUserRequest {
    email: String,
    password: String,
}

pub struct RegisterUserRequest {
    username: String,
    email: String,
    password: String,
}

pub struct UpdateUserRequest {
    email: Option<String>,
    username: Option<String>,
    password: Option<String>,
    bio: Option<String>,
    image: Option<String>,
}

pub struct ProfileResponse {
    username: String,
    bio: Option<String>,
    image: Option<String>,
    following: bool,
}

pub struct ArticleResponse {
    slug: String,
    title: String,
    description: String,
    body: String,
    tag_list: Option<Vec<String>>,
    created_at: String,
    updated_at: String,
    favourited: bool,
    favourties_count: u32,
    author: ProfileResponse,
}

pub struct CreateArticleRequest {
    title: String,
    description: String,
    body: String,
    tag_list: Option<Vec<String>>,
}

pub struct UpdateArticleRequest {
    title: Option<String>,
    description: Option<String>,
    body: Option<String>,
    tag_list: Option<Vec<String>>,
}

pub struct CommentResponse {
    id: u32,
    created_at: String,
    updated_at: String,
    body: String,
    author: ProfileResponse,
}

pub struct AddCommentRequest {
    body: String,
}

fn main() {
    println!("Hello, world!");
}
