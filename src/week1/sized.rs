use super::dispatch::{Storage, User};

trait Command {}

trait CommandHandler<C: Command> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &mut Self::Context) -> Self::Result;
}

struct CreateUser {
    user: User,
}

impl Command for CreateUser {}

pub struct UserError {
    pub reason: String,
}

impl CommandHandler<CreateUser> for User {
    type Context = dyn Storage<u64, User>;
    type Result = Result<(), UserError>;

    fn handle_command(&self, cmd: &CreateUser, user_repo: &mut Self::Context) -> Self::Result {
        // Here we operate with the `UserRepository`
        // via its trait object `&dyn UserRepository`

        if !cmd.user.activated {
            Result::Err(UserError {
                reason: "user isn't active".to_string(),
            })
        } else {
            user_repo.set(cmd.user.id, cmd.user.clone());
            Result::Ok(())
        }
    }
}
