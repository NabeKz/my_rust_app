pub mod model {
    use std::future::Future;
    use uuid::Uuid;

    pub struct UserId(Uuid);
    pub struct Username(String);
    pub struct User {
        id: UserId,
        name: Username,
    }

    pub fn get_users() -> impl Future<Output = Vec<User>> {
        async { vec![] }
    }

    pub fn get_user_by_id(id: UserId) -> impl Future<Output = Option<User>> {
        async move {
            let _ = id; // id を使用
            None
        }
    }

    // 動作確認用のテスト関数
    pub async fn test_usage() {
        let users = get_users().await;
        println!("Got {} users", users.len());

        let user_id = UserId(uuid::Uuid::new_v4());
        let user = get_user_by_id(user_id).await;
        println!("User found: {}", user.is_some());
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
}
