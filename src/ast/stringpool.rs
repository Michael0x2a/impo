#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StringPoolId(usize);

pub struct StringPool {
    pool: Vec<String>,
}

impl StringPool {
    pub fn new() -> StringPool {
        StringPool { pool: Vec::new() }
    }

    pub fn add(&mut self, s: String) -> StringPoolId {
        let id = StringPoolId(self.pool.len());
        self.pool.push(s);
        id
    }

    pub fn lookup(&self, id: StringPoolId) -> &str {
        &self.pool[id.0]
    }

    pub fn debug(&self) {
        println!("  Pool:");
        for s in self.pool.iter() {
            println!("    {:?}", s);
        }
        println!();
    }
}

pub trait ReadOnlyStringPool {
    fn lookup(&self, id: StringPoolId) -> &str;
    fn debug(&self);
}

impl ReadOnlyStringPool for StringPool {
    fn lookup(&self, id: StringPoolId) -> &str {
        self.lookup(id)
    }

    fn debug(&self) {
        self.debug()
    }
}