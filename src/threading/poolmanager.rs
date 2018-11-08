use super::pool::Pool;

pub struct PoolManager {
    pool: Pool
}

impl PoolManager {
    pub fn new() -> PoolManager {
        PoolManager {
            pool: Pool::new()
        }
    }

    pub fn invoke_all() -> boolean {

        panic!("unimplemented")
    }
}

#[cfg(test)]
mod tests {
    use super::PoolManager;

    #[test]
    fn can_instantiate_poolmanager() {
        let p = PoolManager::new();
    }
}
