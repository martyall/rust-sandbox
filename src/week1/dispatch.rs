use std::{borrow::Cow, collections::HashMap, hash::Hash};

pub trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

impl<K: Eq + Hash, V> Storage<K, V> for HashMap<K, V> {
    fn set(&mut self, key: K, val: V) {
        self.insert(key, val);
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }
}

#[derive(Clone)]
pub struct User {
    pub id: u64,
    pub email: Cow<'static, str>,
    pub activated: bool,
}

pub struct UserRepositoryDyn {
    pub users: Box<dyn Storage<u64, User>>,
}

impl UserRepositoryDyn {
    pub fn new(users: Box<dyn Storage<u64, User>>) -> Self {
        UserRepositoryDyn { users }
    }
}

impl Storage<u64, User> for UserRepositoryDyn {
    fn set(&mut self, key: u64, val: User) {
        self.users.set(key, val)
    }

    fn get(&self, key: &u64) -> Option<&User> {
        self.users.get(key)
    }

    fn remove(&mut self, key: &u64) -> Option<User> {
        self.users.remove(key)
    }
}

pub struct UserRepositoryStatic<S: Storage<u64, User>> {
    pub users: S,
}

impl<S: Storage<u64, User>> UserRepositoryStatic<S> {
    pub fn new(users: S) -> Self {
        UserRepositoryStatic { users }
    }
}

impl<S: Storage<u64, User>> Storage<u64, User> for UserRepositoryStatic<S> {
    fn set(&mut self, key: u64, val: User) {
        self.users.set(key, val)
    }

    fn get(&self, key: &u64) -> Option<&User> {
        self.users.get(key)
    }

    fn remove(&mut self, key: &u64) -> Option<User> {
        self.users.remove(key)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dyn() {
        let mut repo = UserRepositoryDyn {
            users: Box::new(HashMap::new()),
        };

        let user = User {
            id: 1,
            email: Cow::from("martyall@gmail.com"),
            activated: true,
        };

        assert_eq!(repo.remove(&1).map(|x| x.id), None);
        repo.set(user.id, user);
        assert_eq!(repo.get(&1).map(|x| x.id), Some(1));
    }

    #[test]
    fn test_static() {
        let mut repo = UserRepositoryStatic {
            users: HashMap::new(),
        };

        let user = User {
            id: 1,
            email: Cow::from("martyall@gmail.com"),
            activated: true,
        };

        assert_eq!(repo.remove(&1).map(|x| x.id), None);
        repo.set(user.id, user);
        assert_eq!(repo.get(&1).map(|x| x.id), Some(1));
    }
}
