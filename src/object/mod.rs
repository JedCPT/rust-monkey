

enum ObjectType {
    Int,
    Bool,
    Null
}

trait Object {
    fn get_type(&self) -> ObjectType;
    fn to_string(&self) -> String;
}

struct Integer {
    value: i64
}


impl Object for Integer {
    fn get_type(&self) -> ObjectType {
        return ObjectType::Int;

    }

    fn to_string(&self) -> String {
        return format!("{}", self.value);
    }
}

struct Boolean {
    value: bool
}


impl Object for Boolean {
    fn get_type(&self) -> ObjectType {
        return ObjectType::Bool;

    }

    fn to_string(&self) -> String {
        return format!("{}", self.value);
    }
}

struct Null {
    value: bool
}


impl Object for Null {
    fn get_type(&self) -> ObjectType {
        return ObjectType::Null;

    }

    fn to_string(&self) -> String {
        return format!("null");
    }
}