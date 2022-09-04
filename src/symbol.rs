use std::collections::HashMap;

pub enum Symbol<'a> {
    Native {
        name: &'a str,
    }
}

pub const Number: Symbol = Symbol::Native { name: "Number" };
pub const Decimal: Symbol = Symbol::Native { name: "Decimal" };
pub const Array: Symbol = Symbol::Native { name: "Array" };

pub struct SymbolTable<'a> {
    members: HashMap<&'a str, Symbol<'a>>
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> SymbolTable<'a> {
        let mut table = SymbolTable { members: HashMap::new() };

        table.define(Number);
        table.define(Decimal);
        table.define(Array);

        table
    }

    pub fn define(&mut self, symbol: Symbol<'a>) {
        let symName: &'a str;

        match symbol {
            Symbol::Native { name } => symName = name,
        }

        self.members.insert(symName, symbol);
    }

    pub fn lookup(&self, name: &'a str) -> Option<&Symbol<'a>> {
        self.members.get(name)
    }
}