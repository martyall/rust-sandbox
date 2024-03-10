use std::marker::PhantomData;

mod post {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(u64);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Title(String);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Body(String);
}
mod user {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(u64);
}

#[derive(Clone)]
struct Post<T> {
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
    state: PhantomData<T>,
}

struct SNew;
struct SUnModerated;
struct SPublished;
struct SDeleted;

impl Post<SNew> {
    pub fn publish(self) -> Post<SUnModerated> {
        Post {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
            state: PhantomData,
        }
    }
}
