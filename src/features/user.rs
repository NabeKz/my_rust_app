pub mod model {
    use async_trait::async_trait;
    use uuid::Uuid;

    #[derive(Clone)]
    pub struct UserId(Uuid);
    #[derive(Clone)]
    pub struct Username(String);

    #[derive(Clone)]
    pub struct User {
        id: UserId,
        name: Username,
    }

    impl UserId {
        pub fn value(self) -> Uuid {
            self.0
        }
    }

    impl Username {
        pub fn new<S: Into<String>>(name: S) -> Self {
            let name = name.into();
            // TODO: validate
            Self(name)
        }
        pub fn value(self) -> String {
            self.0
        }
    }

    impl User {
        pub fn id(&self) -> &UserId {
            &self.id
        }
        pub fn name(&self) -> &Username {
            &self.name
        }
    }

    #[async_trait]
    pub trait UserRepository: Send + Sync + 'static {
        async fn get_users(&self) -> Vec<User>;
        async fn get_user(&self, id: UserId) -> Result<User, String>;
    }
}

pub mod usecase {}
pub mod infra {
    use crate::features::user::model::{User, UserId, UserRepository};
    use async_trait::async_trait;
    pub struct UserRepositoryOnMemory {
        items: Vec<User>,
    }

    #[async_trait]
    impl UserRepository for UserRepositoryOnMemory {
        async fn get_users(&self) -> Vec<User> {
            self.items.clone()
        }

        async fn get_user(&self, id: UserId) -> Result<User, String> {
            let _ = id.value();
            todo!()
        }
    }
}
