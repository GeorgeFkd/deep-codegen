use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct Import {
    //import org.codegen.package.class_name
    pub class_name: String,
    pub package_name: String,
    //import static org.codegen.package.class_name
    pub static_import: bool,
}


#[derive(Debug,Clone)]
pub struct GenericParams {
    pub generics:Vec<String>
}

#[derive(Clone)]
pub struct Annotation {
    pub qualified_name: String,
    //name = value
    pub params_list: Option<Vec<(String, String)>>,
}

#[derive(Debug, Clone)]
pub struct TypeName {
    pub name: String,
    pub generic_params: Option<GenericParams>,
}

impl PartialEq<Self> for Annotation {
    fn eq(&self, other: &Self) -> bool {
        self.qualified_name.eq(&other.qualified_name)
    }
}

impl Eq for Annotation {}

impl Hash for Annotation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.qualified_name.hash(state)
    }
}

impl PartialEq<Self> for TypeName {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for TypeName {}

impl Hash for TypeName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl Hash for AccessModifiers {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        <AccessModifiers as Into<String>>::into(*self).hash(state)
    }
}

impl PartialEq<Self> for AccessModifiers {
    fn eq(&self, other: &Self) -> bool {
        <AccessModifiers as Into<String>>::into(*self)
            == <AccessModifiers as Into<String>>::into(*other)
    }
}

impl PartialOrd<Self> for AccessModifiers {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if <AccessModifiers as Into<String>>::into(*self)
            == <AccessModifiers as Into<String>>::into(*other)
        {
            return Some(Ordering::Equal);
        }
        //private,public,protected > abstract > final > static
        if self.eq(&AccessModifiers::Private)
            || self.eq(&AccessModifiers::Public)
            || self.eq(&AccessModifiers::Protected)
        {
            return Some(Ordering::Greater);
        }
        if other.eq(&AccessModifiers::Protected)
            || other.eq(&AccessModifiers::Public)
            || other.eq(&AccessModifiers::Private)
        {
            return Some(Ordering::Less);
        }

        if self.eq(&AccessModifiers::Abstract) {
            return Some(Ordering::Greater);
        }
        if other.eq(&AccessModifiers::Abstract) {
            return Some(Ordering::Less);
        }

        if self.eq(&AccessModifiers::Final) {
            return Some(Ordering::Greater);
        }
        if other.eq(&AccessModifiers::Final) {
            return Some(Ordering::Less);
        }

        Some(Ordering::Equal)
    }
}

impl Eq for AccessModifiers {}



// impl PartialOrd<AccessModifiers> for AccessModifiers {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         if <AccessModifiers as Into<String>>::into(*self)
//             == <AccessModifiers as Into<String>>::into(*other)
//         {
//             return Some(Ordering::Equal);
//         }
//         //private,public,protected > abstract > final > static
//         if self.eq(&AccessModifiers::Private)
//             || self.eq(&AccessModifiers::Public)
//             || self.eq(&AccessModifiers::Protected)
//         {
//             return Some(Ordering::Greater);
//         }
//         if other.eq(&AccessModifiers::Protected)
//             || other.eq(&AccessModifiers::Public)
//             || other.eq(&AccessModifiers::Private)
//         {
//             return Some(Ordering::Less);
//         }
//
//         if self.eq(&AccessModifiers::Abstract) {
//             return Some(Ordering::Greater);
//         }
//         if other.eq(&AccessModifiers::Abstract) {
//             return Some(Ordering::Less);
//         }
//
//         if self.eq(&AccessModifiers::Final) {
//             return Some(Ordering::Greater);
//         }
//         if other.eq(&AccessModifiers::Final) {
//             return Some(Ordering::Less);
//         }
//
//         Some(Ordering::Equal)
//     }
// }

impl Ord for AccessModifiers {
    fn cmp(&self, other: &Self) -> Ordering {
        if <AccessModifiers as Into<String>>::into(*self)
            == <AccessModifiers as Into<String>>::into(*other)
        {
            return Ordering::Equal;
        }
        //private,public,protected > abstract > fina
        if self.eq(&AccessModifiers::Private)
            || self.eq(&AccessModifiers::Public)
            || self.eq(&AccessModifiers::Protected)
        {
            return Ordering::Greater;
        }
        if other.eq(&AccessModifiers::Protected)
            || other.eq(&AccessModifiers::Public)
            || other.eq(&AccessModifiers::Private)
        {
            return Ordering::Less;
        }

        if self.eq(&AccessModifiers::Abstract) {
            return Ordering::Greater;
        }
        if other.eq(&AccessModifiers::Abstract) {
            return Ordering::Less;
        }

        if self.eq(&AccessModifiers::Final) {
            return Ordering::Greater;
        }
        if other.eq(&AccessModifiers::Final) {
            return Ordering::Less;
        }

        Ordering::Equal
    }
}

impl Into<&str> for AccessModifiers {
    fn into(self) -> &'static str {
        match self {
            AccessModifiers::Public => "public",
            AccessModifiers::Private => "private",
            AccessModifiers::Protected => "protected",
            AccessModifiers::Static => "static",
            AccessModifiers::Abstract => "abstract",
            AccessModifiers::Final => "final",
        }
    }
}
impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum AccessModifiers {
    Public,
    Private,
    Protected,
    Static,
    Abstract,
    Final,
    //Will not use those
    //Native
    //Synchronised
    //Transient
    //Volatile
    //strictfp
}


#[derive(Hash, Eq, Clone)]
pub struct Field {
    //might be empty but we dont care
    pub annotation: Vec<Annotation>,
    //i want to make this a hashset to avoid duplicates but i dont think someone would
    //accidentally input duplicate stuff
    pub modifiers: Vec<AccessModifiers>,
    pub name: String,
    pub type_: TypeName,
    //this type can be stricter
    pub initializer: Option<String>,
}

pub type Implements = TypeName;

#[derive(Clone)]
pub struct VariableParam {
    pub name: String,
    pub type_: TypeName,
    pub annotation: Vec<Annotation>,
}