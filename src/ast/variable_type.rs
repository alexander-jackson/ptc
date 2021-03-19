//! Variable types that `ptc` supports.
//!
//! This simply defines an `enum` of `VariableType`s that the compiler can understand and use
//! throughout the code. For example, after stating `x = 1`, `x` will have a stored type of
//! `VariableType::Integer` whenever it is mentioned.

use regex::Regex;

/// Variable types that `ptc` currently supports.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VariableType {
    /// The integer type
    Integer,
    /// The float type
    Float,
    /// The void type, only used for function returns
    Void,
    /// A list of homogeneous types
    List {
        /// The type of each element in the list
        elements: Option<Box<VariableType>>,
    },
}

impl From<&VariableType> for String {
    /// Allows for the conversion of a [`VariableType`] to a String.
    ///
    /// Code generation regularly requires the conversion of internal `VariableType`s to `String`s.
    /// This function allows for new types to be more easily added, as code generation will call
    /// `String::from(v)` and get the string represenation.
    fn from(v: &VariableType) -> String {
        match v {
            VariableType::Integer => String::from("int"),
            VariableType::Float => String::from("float"),
            VariableType::Void => String::from("void"),
            VariableType::List { elements } => match elements {
                Some(t) => format!("list_{}*", String::from(t)),
                None => String::from("list_unknown*"),
            },
        }
    }
}

impl From<&Box<VariableType>> for String {
    /// Allows for the conversion of a &Box<VariableType> to a String.
    ///
    /// When dealing with the `VariableType::List { elements }` enum type, frequently a reference
    /// is given to the inner `elements`. Converting to a String requires using the above version
    /// and thus using this notation to do so. Instead, this can be used.
    fn from(v: &Box<VariableType>) -> String {
        String::from(&**v)
    }
}

impl From<&str> for VariableType {
    /// Allows for easier conversion of typehints to actual concrete types.
    ///
    /// Allowing a `from` implementation allows for an abstraction of the underlying conversions to
    /// be done in here, especially with the contained Regex for List[...] types.
    fn from(s: &str) -> VariableType {
        match s {
            "int" => return VariableType::Integer,
            "float" => return VariableType::Float,
            "None" => return VariableType::Void,
            _ => (),
        }

        lazy_static! {
            static ref RE: Regex = Regex::new(r"^List\[(.*)\]$").unwrap();
        }

        if let Some(caps) = RE.captures(&s) {
            let inner = caps.get(1).unwrap().as_str();

            return VariableType::List {
                elements: Some(Box::new(VariableType::from(inner))),
            };
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::VariableType;

    #[test]
    fn variable_type_from_str_parses_correctly() {
        assert_eq!(VariableType::from("int"), VariableType::Integer);
        assert_eq!(VariableType::from("float"), VariableType::Float);
        assert_eq!(VariableType::from("None"), VariableType::Void);

        assert_eq!(
            VariableType::from("List[float]"),
            VariableType::List {
                elements: Some(Box::new(VariableType::Float)),
            }
        );

        assert_eq!(
            VariableType::from("List[List[int]]"),
            VariableType::List {
                elements: Some(Box::new(VariableType::List {
                    elements: Some(Box::new(VariableType::Integer))
                })),
            }
        );
    }
}
