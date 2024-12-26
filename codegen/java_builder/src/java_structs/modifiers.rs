use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::hash::Hash;
#[derive(Copy, Clone, Debug, Default)]
pub enum AccessModifiers {
    #[default]
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
impl super::Codegen for Vec<AccessModifiers> {
    fn generate_code(&self) -> String {
        let mut result = "".to_string();
        let mut modifiers = self.clone();
        modifiers.sort_by(|a, b| b.cmp(a));
        //more rules for modifiers
        assert!(
            !(modifiers.contains(&AccessModifiers::Public)
                && modifiers.contains(&AccessModifiers::Protected)),
            "Modifiers {:?} and {:?} should not be used together",
            &AccessModifiers::Public,
            &AccessModifiers::Protected
        );
        assert!(
            !(modifiers.contains(&AccessModifiers::Protected)
                && modifiers.contains(&AccessModifiers::Private)),
            "Modifiers {:?} and {:?} should not be used together",
            &AccessModifiers::Protected,
            &AccessModifiers::Private
        );
        assert!(
            !(modifiers.contains(&AccessModifiers::Public)
                && modifiers.contains(&AccessModifiers::Private)),
            "Modifiers {:?} and {:?} should not be used together",
            &AccessModifiers::Public,
            &AccessModifiers::Private
        );

        modifiers.dedup();
        for m in modifiers.iter() {
            result.push_str(&format!("{} ", <AccessModifiers as Into<String>>::into(*m)));
        }
        result
    }
}
impl Into<String> for AccessModifiers {
    fn into(self) -> String {
        match self {
            AccessModifiers::Public => "public".to_owned(),
            AccessModifiers::Private => "private".to_owned(),
            AccessModifiers::Protected => "protected".to_owned(),
            AccessModifiers::Static => "static".to_owned(),
            AccessModifiers::Abstract => "abstract".to_owned(),
            AccessModifiers::Final => "final".to_owned(),
        }
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
