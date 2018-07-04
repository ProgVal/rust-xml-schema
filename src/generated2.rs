#[allow(bad_style)]
#[macro_use] use support;
extern crate xmlparser;

pub use std::collections::HashMap;

pub use std::marker::PhantomData;

pub use support::*;

pub use xmlparser::{Token, ElementEnd};

pub mod xml {
    use super::*;
}

pub mod xmlns {
    use super::*;
}

pub mod xs {
    use super::*;

    #[derive(Debug, PartialEq)]
    pub struct All<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub all_model: super::xs::AllModel<'input>,
    }

    impl_element!(All, "all", {
        (all_model, xs, AllModel),
    });

    #[derive(Debug, PartialEq)]
    pub struct Annotation<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation_content: Vec<super::enums::AnnotationContent<'input>>,
    }

    impl_element!(Annotation, "annotation", {
        (annotation_content, enums, Vec<AnnotationContent>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Any<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(Any, "any", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct AnyAttribute<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(AnyAttribute, "anyAttribute", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Appinfo<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub sequence_any: Vec<super::sequences::SequenceAny<'input>>,
    }

    impl_element!(Appinfo, "appinfo", {
        (sequence_any, sequences, Vec<SequenceAny>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Assertion<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(Assertion, "assertion", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Attribute<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub simple_type_local_simple_type: Option<super::inline_elements::SimpleTypeLocalSimpleType<'input>>,
    }

    impl_element!(Attribute, "attribute", {
        (annotation, xs, Option<Annotation>),
        (simple_type_local_simple_type, inline_elements, Option<SimpleTypeLocalSimpleType>),
    });

    #[derive(Debug, PartialEq)]
    pub struct AttributeGroup<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub attr_decls: super::xs::AttrDecls<'input>,
    }

    impl_element!(AttributeGroup, "attributeGroup", {
        (annotation, xs, Option<Annotation>),
        (attr_decls, xs, AttrDecls),
    });

    #[derive(Debug, PartialEq)]
    pub struct Choice<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub nested_particle: Vec<super::xs::NestedParticle<'input>>,
    }

    impl_element!(Choice, "choice", {
        (annotation, xs, Option<Annotation>),
        (nested_particle, xs, Vec<NestedParticle>),
    });

    #[derive(Debug, PartialEq)]
    pub struct ComplexContent<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub content_def: super::enums::ContentDef<'input>,
    }

    impl_element!(ComplexContent, "complexContent", {
        (annotation, xs, Option<Annotation>),
        (content_def, enums, ContentDef),
    });

    #[derive(Debug, PartialEq)]
    pub struct ComplexType<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub complex_type_model: super::xs::ComplexTypeModel<'input>,
    }

    impl_element!(ComplexType, "complexType", {
        (annotation, xs, Option<Annotation>),
        (complex_type_model, xs, ComplexTypeModel),
    });

    #[derive(Debug, PartialEq)]
    pub struct DefaultOpenContent<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub any_wildcard: super::inline_elements::AnyWildcard<'input>,
    }

    impl_element!(DefaultOpenContent, "defaultOpenContent", {
        (annotation, xs, Option<Annotation>),
        (any_wildcard, inline_elements, AnyWildcard),
    });

    #[derive(Debug, PartialEq)]
    pub struct Documentation<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub sequence_any: Vec<super::sequences::SequenceAny<'input>>,
    }

    impl_element!(Documentation, "documentation", {
        (sequence_any, sequences, Vec<SequenceAny>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Element<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub type_: Option<super::enums::Type<'input>>,
        pub alternative_alt_type: Vec<super::inline_elements::AlternativeAltType<'input>>,
        pub identity_constraint: Vec<super::xs::IdentityConstraint<'input>>,
    }

    impl_element!(Element, "element", {
        (annotation, xs, Option<Annotation>),
        (type_, enums, Option<Type>),
        (alternative_alt_type, inline_elements, Vec<AlternativeAltType>),
        (identity_constraint, xs, Vec<IdentityConstraint>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Enumeration<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(Enumeration, "enumeration", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct ExplicitTimezone<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(ExplicitTimezone, "explicitTimezone", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Facet<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
    }

    impl_element!(Facet, "facet", {
    });

    #[derive(Debug, PartialEq)]
    pub struct Field<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(Field, "field", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct FractionDigits<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(FractionDigits, "fractionDigits", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Group<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub choice_all_choice_sequence: super::enums::ChoiceAllChoiceSequence<'input>,
    }

    impl_element!(Group, "group", {
        (annotation, xs, Option<Annotation>),
        (choice_all_choice_sequence, enums, ChoiceAllChoiceSequence),
    });

    #[derive(Debug, PartialEq)]
    pub struct Import<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(Import, "import", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Include<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(Include, "include", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Key<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub uniqueness_spec: Option<super::sequences::UniquenessSpec<'input>>,
    }

    impl_element!(Key, "key", {
        (annotation, xs, Option<Annotation>),
        (uniqueness_spec, sequences, Option<UniquenessSpec>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Keyref<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub uniqueness_spec: Option<super::sequences::UniquenessSpec<'input>>,
    }

    impl_element!(Keyref, "keyref", {
        (annotation, xs, Option<Annotation>),
        (uniqueness_spec, sequences, Option<UniquenessSpec>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Length<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(Length, "length", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct List<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub simple_type_local_simple_type: Option<super::inline_elements::SimpleTypeLocalSimpleType<'input>>,
    }

    impl_element!(List, "list", {
        (annotation, xs, Option<Annotation>),
        (simple_type_local_simple_type, inline_elements, Option<SimpleTypeLocalSimpleType>),
    });

    #[derive(Debug, PartialEq)]
    pub struct MaxExclusive<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(MaxExclusive, "maxExclusive", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct MaxInclusive<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(MaxInclusive, "maxInclusive", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct MaxLength<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(MaxLength, "maxLength", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct MinExclusive<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(MinExclusive, "minExclusive", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct MinInclusive<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(MinInclusive, "minInclusive", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct MinLength<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(MinLength, "minLength", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Notation<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(Notation, "notation", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct OpenContent<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub any_wildcard: Option<super::inline_elements::AnyWildcard<'input>>,
    }

    impl_element!(OpenContent, "openContent", {
        (annotation, xs, Option<Annotation>),
        (any_wildcard, inline_elements, Option<AnyWildcard>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Override<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub schema_top: Vec<super::xs::SchemaTop<'input>>,
    }

    impl_element!(Override, "override", {
        (annotation, xs, Option<Annotation>),
        (schema_top, xs, Vec<SchemaTop>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Pattern<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(Pattern, "pattern", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Redefine<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub choice_annotation_redefinable: Vec<super::enums::ChoiceAnnotationRedefinable<'input>>,
    }

    impl_element!(Redefine, "redefine", {
        (choice_annotation_redefinable, enums, Vec<ChoiceAnnotationRedefinable>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Restriction<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub simple_restriction_model: super::xs::SimpleRestrictionModel<'input>,
    }

    impl_element!(Restriction, "restriction", {
        (annotation, xs, Option<Annotation>),
        (simple_restriction_model, xs, SimpleRestrictionModel),
    });

    #[derive(Debug, PartialEq)]
    pub struct Schema<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub composition: Vec<super::xs::Composition<'input>>,
        pub open_content: Option<super::sequences::AnnotatedOpenContent<'input>>,
        pub sequence_schema_top_annotation: Vec<super::sequences::SequenceSchemaTopAnnotation<'input>>,
    }

    impl_element!(Schema, "schema", {
        (composition, xs, Vec<Composition>),
        (open_content, sequences, Option<AnnotatedOpenContent>),
        (sequence_schema_top_annotation, sequences, Vec<SequenceSchemaTopAnnotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Selector<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(Selector, "selector", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Sequence<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub nested_particle: Vec<super::xs::NestedParticle<'input>>,
    }

    impl_element!(Sequence, "sequence", {
        (annotation, xs, Option<Annotation>),
        (nested_particle, xs, Vec<NestedParticle>),
    });

    #[derive(Debug, PartialEq)]
    pub struct SimpleContent<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub content_def: super::enums::ContentDef<'input>,
    }

    impl_element!(SimpleContent, "simpleContent", {
        (annotation, xs, Option<Annotation>),
        (content_def, enums, ContentDef),
    });

    #[derive(Debug, PartialEq)]
    pub struct SimpleType<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub simple_derivation: super::xs::SimpleDerivation<'input>,
    }

    impl_element!(SimpleType, "simpleType", {
        (annotation, xs, Option<Annotation>),
        (simple_derivation, xs, SimpleDerivation),
    });

    #[derive(Debug, PartialEq)]
    pub struct TotalDigits<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(TotalDigits, "totalDigits", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Union<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub simple_type_local_simple_type: Vec<super::inline_elements::SimpleTypeLocalSimpleType<'input>>,
    }

    impl_element!(Union, "union", {
        (annotation, xs, Option<Annotation>),
        (simple_type_local_simple_type, inline_elements, Vec<SimpleTypeLocalSimpleType>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Unique<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub uniqueness_spec: Option<super::sequences::UniquenessSpec<'input>>,
    }

    impl_element!(Unique, "unique", {
        (annotation, xs, Option<Annotation>),
        (uniqueness_spec, sequences, Option<UniquenessSpec>),
    });

    #[derive(Debug, PartialEq)]
    pub struct WhiteSpace<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(WhiteSpace, "whiteSpace", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct AllModel<'input> {
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub choice_element_any_group: Vec<super::enums::ChoiceElementAnyGroup<'input>>,
    }

    impl_group_or_sequence!(AllModel,
        (annotation, xs, Option<Annotation>),
        (choice_element_any_group, enums, Vec<ChoiceElementAnyGroup>),
    );

    #[derive(Debug, PartialEq)]
    pub struct Assertions<'input> {
        pub assert_assertion: Vec<super::inline_elements::AssertAssertion<'input>>,
    }

    impl_group_or_sequence!(Assertions,
        (assert_assertion, inline_elements, Vec<AssertAssertion>),
    );

    #[derive(Debug, PartialEq)]
    pub struct AttrDecls<'input> {
        pub attribute: Vec<super::enums::AttrOrAttrGroup<'input>>,
        pub any_attribute: Option<super::xs::AnyAttribute<'input>>,
    }

    impl_group_or_sequence!(AttrDecls,
        (attribute, enums, Vec<AttrOrAttrGroup>),
        (any_attribute, xs, Option<AnyAttribute>),
    );

    #[derive(Debug, PartialEq)]
    pub enum ComplexTypeModel<'input> {
        SimpleContent(Box<super::xs::SimpleContent<'input>>),
        ComplexContent(Box<super::xs::ComplexContent<'input>>),
        CompleteContentModel {
            open_content: Option<Box<super::xs::OpenContent<'input>> >,
            type_def_particle: Option<Box<super::xs::TypeDefParticle<'input>> >,
            attr_decls: Box<super::xs::AttrDecls<'input>>,
            assertions: Box<super::xs::Assertions<'input>>,
        }
        ,
    }

    impl_enum!(ComplexTypeModel,
        impl_singleton_variant!(SimpleContent, xs, Box<SimpleContent>),
        impl_singleton_variant!(ComplexContent, xs, Box<ComplexContent>),
        impl_struct_variant!(CompleteContentModel,
            (open_content, xs, Option<Box<OpenContent> >),
            (type_def_particle, xs, Option<Box<TypeDefParticle> >),
            (attr_decls, xs, Box<AttrDecls>),
            (assertions, xs, Box<Assertions>),
        ),
    );

    #[derive(Debug, PartialEq)]
    pub enum Composition<'input> {
        Include(Box<super::xs::Include<'input>>),
        Import(Box<super::xs::Import<'input>>),
        Redefine(Box<super::xs::Redefine<'input>>),
        Override(Box<super::xs::Override<'input>>),
        Annotation(Box<super::xs::Annotation<'input>>),
    }

    impl_enum!(Composition,
        impl_singleton_variant!(Include, xs, Box<Include>),
        impl_singleton_variant!(Import, xs, Box<Import>),
        impl_singleton_variant!(Redefine, xs, Box<Redefine>),
        impl_singleton_variant!(Override, xs, Box<Override>),
        impl_singleton_variant!(Annotation, xs, Box<Annotation>),
    );

    #[derive(Debug, PartialEq)]
    pub enum IdentityConstraint<'input> {
        Unique(Box<super::xs::Unique<'input>>),
        Key(Box<super::xs::Key<'input>>),
        Keyref(Box<super::xs::Keyref<'input>>),
    }

    impl_enum!(IdentityConstraint,
        impl_singleton_variant!(Unique, xs, Box<Unique>),
        impl_singleton_variant!(Key, xs, Box<Key>),
        impl_singleton_variant!(Keyref, xs, Box<Keyref>),
    );

    #[derive(Debug, PartialEq)]
    pub enum NestedParticle<'input> {
        Element(Box<super::inline_elements::ElementLocalElement<'input>>),
        Group(Box<super::inline_elements::GroupGroupRef<'input>>),
        Choice(Box<super::xs::Choice<'input>>),
        Sequence(Box<super::xs::Sequence<'input>>),
        Any(Box<super::xs::Any<'input>>),
    }

    impl_enum!(NestedParticle,
        impl_singleton_variant!(Element, inline_elements, Box<ElementLocalElement>),
        impl_singleton_variant!(Group, inline_elements, Box<GroupGroupRef>),
        impl_singleton_variant!(Choice, xs, Box<Choice>),
        impl_singleton_variant!(Sequence, xs, Box<Sequence>),
        impl_singleton_variant!(Any, xs, Box<Any>),
    );

    #[derive(Debug, PartialEq)]
    pub enum Particle<'input> {
        Element(Box<super::inline_elements::ElementLocalElement<'input>>),
        Group(Box<super::inline_elements::GroupGroupRef<'input>>),
        All(Box<super::xs::All<'input>>),
        Choice(Box<super::xs::Choice<'input>>),
        Sequence(Box<super::xs::Sequence<'input>>),
        Any(Box<super::xs::Any<'input>>),
    }

    impl_enum!(Particle,
        impl_singleton_variant!(Element, inline_elements, Box<ElementLocalElement>),
        impl_singleton_variant!(Group, inline_elements, Box<GroupGroupRef>),
        impl_singleton_variant!(All, xs, Box<All>),
        impl_singleton_variant!(Choice, xs, Box<Choice>),
        impl_singleton_variant!(Sequence, xs, Box<Sequence>),
        impl_singleton_variant!(Any, xs, Box<Any>),
    );

    #[derive(Debug, PartialEq)]
    pub enum Redefinable<'input> {
        SimpleType(Box<super::xs::SimpleType<'input>>),
        ComplexType(Box<super::xs::ComplexType<'input>>),
        Group(Box<super::xs::Group<'input>>),
        AttributeGroup(Box<super::xs::AttributeGroup<'input>>),
    }

    impl_enum!(Redefinable,
        impl_singleton_variant!(SimpleType, xs, Box<SimpleType>),
        impl_singleton_variant!(ComplexType, xs, Box<ComplexType>),
        impl_singleton_variant!(Group, xs, Box<Group>),
        impl_singleton_variant!(AttributeGroup, xs, Box<AttributeGroup>),
    );

    #[derive(Debug, PartialEq)]
    pub enum SchemaTop<'input> {
        Redefinable(Box<super::xs::Redefinable<'input>>),
        Element(Box<super::xs::Element<'input>>),
        Attribute(Box<super::xs::Attribute<'input>>),
        Notation(Box<super::xs::Notation<'input>>),
    }

    impl_enum!(SchemaTop,
        impl_singleton_variant!(Redefinable, xs, Box<Redefinable>),
        impl_singleton_variant!(Element, xs, Box<Element>),
        impl_singleton_variant!(Attribute, xs, Box<Attribute>),
        impl_singleton_variant!(Notation, xs, Box<Notation>),
    );

    #[derive(Debug, PartialEq)]
    pub enum SimpleDerivation<'input> {
        Restriction(Box<super::xs::Restriction<'input>>),
        List(Box<super::xs::List<'input>>),
        Union(Box<super::xs::Union<'input>>),
    }

    impl_enum!(SimpleDerivation,
        impl_singleton_variant!(Restriction, xs, Box<Restriction>),
        impl_singleton_variant!(List, xs, Box<List>),
        impl_singleton_variant!(Union, xs, Box<Union>),
    );

    #[derive(Debug, PartialEq)]
    pub struct SimpleRestrictionModel<'input> {
        pub simple_type_local_simple_type: Option<super::inline_elements::SimpleTypeLocalSimpleType<'input>>,
        pub choice_facet_any: Vec<super::enums::ChoiceFacetAny<'input>>,
    }

    impl_group_or_sequence!(SimpleRestrictionModel,
        (simple_type_local_simple_type, inline_elements, Option<SimpleTypeLocalSimpleType>),
        (choice_facet_any, enums, Vec<ChoiceFacetAny>),
    );

    #[derive(Debug, PartialEq)]
    pub enum TypeDefParticle<'input> {
        Group(Box<super::inline_elements::GroupGroupRef<'input>>),
        All(Box<super::xs::All<'input>>),
        Choice(Box<super::xs::Choice<'input>>),
        Sequence(Box<super::xs::Sequence<'input>>),
    }

    impl_enum!(TypeDefParticle,
        impl_singleton_variant!(Group, inline_elements, Box<GroupGroupRef>),
        impl_singleton_variant!(All, xs, Box<All>),
        impl_singleton_variant!(Choice, xs, Box<Choice>),
        impl_singleton_variant!(Sequence, xs, Box<Sequence>),
    );
}

pub mod enums {
    use super::*;

    #[derive(Debug, PartialEq)]
    pub enum ChoiceAllChoiceSequence<'input> {
        All(Box<super::inline_elements::AllAllModel<'input>>),
        Choice(Box<super::inline_elements::ChoiceSimpleExplicitGroup<'input>>),
        Sequence(Box<super::inline_elements::SequenceSimpleExplicitGroup<'input>>),
    }

    impl_enum!(ChoiceAllChoiceSequence,
        impl_singleton_variant!(All, inline_elements, Box<AllAllModel>),
        impl_singleton_variant!(Choice, inline_elements, Box<ChoiceSimpleExplicitGroup>),
        impl_singleton_variant!(Sequence, inline_elements, Box<SequenceSimpleExplicitGroup>),
    );

    #[derive(Debug, PartialEq)]
    pub enum ChoiceAnnotationRedefinable<'input> {
        Annotation(Box<super::xs::Annotation<'input>>),
        Redefinable(Box<super::xs::Redefinable<'input>>),
    }

    impl_enum!(ChoiceAnnotationRedefinable,
        impl_singleton_variant!(Annotation, xs, Box<Annotation>),
        impl_singleton_variant!(Redefinable, xs, Box<Redefinable>),
    );

    #[derive(Debug, PartialEq)]
    pub enum AnnotationContent<'input> {
        Appinfo(Box<super::xs::Appinfo<'input>>),
        Documentation(Box<super::xs::Documentation<'input>>),
    }

    impl_enum!(AnnotationContent,
        impl_singleton_variant!(Appinfo, xs, Box<Appinfo>),
        impl_singleton_variant!(Documentation, xs, Box<Documentation>),
    );

    #[derive(Debug, PartialEq)]
    pub enum AttrOrAttrGroup<'input> {
        Attribute(Box<super::inline_elements::AttributeAttribute<'input>>),
        AttributeGroup(Box<super::inline_elements::AttributeGroupAttributeGroupRef<'input>>),
    }

    impl_enum!(AttrOrAttrGroup,
        impl_singleton_variant!(Attribute, inline_elements, Box<AttributeAttribute>),
        impl_singleton_variant!(AttributeGroup, inline_elements, Box<AttributeGroupAttributeGroupRef>),
    );

    #[derive(Debug, PartialEq)]
    pub enum ChoiceElementAnyGroup<'input> {
        Element(Box<super::inline_elements::ElementLocalElement<'input>>),
        Any(Box<super::xs::Any<'input>>),
        Group(Box<super::inline_elements::GroupSequenceAnnotation<'input>>),
    }

    impl_enum!(ChoiceElementAnyGroup,
        impl_singleton_variant!(Element, inline_elements, Box<ElementLocalElement>),
        impl_singleton_variant!(Any, xs, Box<Any>),
        impl_singleton_variant!(Group, inline_elements, Box<GroupSequenceAnnotation>),
    );

    #[derive(Debug, PartialEq)]
    pub enum ChoiceFacetAny<'input> {
        Facet(Box<super::xs::Facet<'input>>),
        Any(Box<super::support::Any<'input>>),
    }

    impl_enum!(ChoiceFacetAny,
        impl_singleton_variant!(Facet, xs, Box<Facet>),
        impl_singleton_variant!(Any, support, Box<Any>),
    );

    #[derive(Debug, PartialEq)]
    pub enum ContentDef<'input> {
        Restriction(Box<super::inline_elements::RestrictionSimpleRestrictionType<'input>>),
        Extension(Box<super::inline_elements::ExtensionSimpleExtensionType<'input>>),
    }

    impl_enum!(ContentDef,
        impl_singleton_variant!(Restriction, inline_elements, Box<RestrictionSimpleRestrictionType>),
        impl_singleton_variant!(Extension, inline_elements, Box<ExtensionSimpleExtensionType>),
    );

    #[derive(Debug, PartialEq)]
    pub enum ChoiceSequenceOpenContentTypeDefParticle<'input> {
        SequenceOpenContentTypeDefParticle {
            open_content: Option<Box<super::xs::OpenContent<'input>> >,
            type_def_particle: Box<super::xs::TypeDefParticle<'input>>,
        }
        ,
    }

    impl_enum!(ChoiceSequenceOpenContentTypeDefParticle,
        impl_struct_variant!(SequenceOpenContentTypeDefParticle,
            (open_content, xs, Option<Box<OpenContent> >),
            (type_def_particle, xs, Box<TypeDefParticle>),
        ),
    );

    #[derive(Debug, PartialEq)]
    pub enum ChoiceSequenceOpenContentTypeDefParticleSimpleRestrictionModel<'input> {
        SequenceOpenContentTypeDefParticle {
            open_content: Option<Box<super::xs::OpenContent<'input>> >,
            type_def_particle: Box<super::xs::TypeDefParticle<'input>>,
        }
        ,
        SimpleRestrictionModel(Box<super::xs::SimpleRestrictionModel<'input>>),
    }

    impl_enum!(ChoiceSequenceOpenContentTypeDefParticleSimpleRestrictionModel,
        impl_struct_variant!(SequenceOpenContentTypeDefParticle,
            (open_content, xs, Option<Box<OpenContent> >),
            (type_def_particle, xs, Box<TypeDefParticle>),
        ),
        impl_singleton_variant!(SimpleRestrictionModel, xs, Box<SimpleRestrictionModel>),
    );

    #[derive(Debug, PartialEq)]
    pub enum ChoiceSimpleRestrictionModel<'input> {
        SimpleRestrictionModel(Box<super::xs::SimpleRestrictionModel<'input>>),
    }

    impl_enum!(ChoiceSimpleRestrictionModel,
        impl_singleton_variant!(SimpleRestrictionModel, xs, Box<SimpleRestrictionModel>),
    );

    #[derive(Debug, PartialEq)]
    pub enum Type<'input> {
        SimpleType(Box<super::inline_elements::SimpleTypeLocalSimpleType<'input>>),
        ComplexType(Box<super::inline_elements::ComplexTypeLocalComplexType<'input>>),
    }

    impl_enum!(Type,
        impl_singleton_variant!(SimpleType, inline_elements, Box<SimpleTypeLocalSimpleType>),
        impl_singleton_variant!(ComplexType, inline_elements, Box<ComplexTypeLocalComplexType>),
    );
}

pub mod sequences {
    use super::*;

    #[derive(Debug, PartialEq)]
    pub struct SequenceAny<'input> {
        pub any: super::support::Any<'input>,
    }

    impl_group_or_sequence!(SequenceAny,
        (any, support, Any),
    );

    #[derive(Debug, PartialEq)]
    pub struct AnnotatedOpenContent<'input> {
        pub default_open_content: super::xs::DefaultOpenContent<'input>,
        pub annotation: Vec<super::xs::Annotation<'input>>,
    }

    impl_group_or_sequence!(AnnotatedOpenContent,
        (default_open_content, xs, DefaultOpenContent),
        (annotation, xs, Vec<Annotation>),
    );

    #[derive(Debug, PartialEq)]
    pub struct SequenceSchemaTopAnnotation<'input> {
        pub schema_top: super::xs::SchemaTop<'input>,
        pub annotation: Vec<super::xs::Annotation<'input>>,
    }

    impl_group_or_sequence!(SequenceSchemaTopAnnotation,
        (schema_top, xs, SchemaTop),
        (annotation, xs, Vec<Annotation>),
    );

    #[derive(Debug, PartialEq)]
    pub struct UniquenessSpec<'input> {
        pub selector: super::xs::Selector<'input>,
        pub field: Vec<super::xs::Field<'input>>,
    }

    impl_group_or_sequence!(UniquenessSpec,
        (selector, xs, Selector),
        (field, xs, Vec<Field>),
    );
}

pub mod inline_elements {
    use super::*;

    #[derive(Debug, PartialEq)]
    pub struct AllAllModel<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub all_model: super::xs::AllModel<'input>,
    }

    impl_element!(AllAllModel, "all", {
        (all_model, xs, AllModel),
    });

    #[derive(Debug, PartialEq)]
    pub struct AlternativeAltType<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub type_: Option<super::enums::Type<'input>>,
    }

    impl_element!(AlternativeAltType, "alternative", {
        (annotation, xs, Option<Annotation>),
        (type_, enums, Option<Type>),
    });

    #[derive(Debug, PartialEq)]
    pub struct AnyWildcard<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(AnyWildcard, "any", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct AssertAssertion<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(AssertAssertion, "assert", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct AttributeAttribute<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub simple_type_local_simple_type: Option<super::inline_elements::SimpleTypeLocalSimpleType<'input>>,
    }

    impl_element!(AttributeAttribute, "attribute", {
        (annotation, xs, Option<Annotation>),
        (simple_type_local_simple_type, inline_elements, Option<SimpleTypeLocalSimpleType>),
    });

    #[derive(Debug, PartialEq)]
    pub struct AttributeGroupAttributeGroupRef<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(AttributeGroupAttributeGroupRef, "attributeGroup", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct ChoiceSimpleExplicitGroup<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub nested_particle: Vec<super::xs::NestedParticle<'input>>,
    }

    impl_element!(ChoiceSimpleExplicitGroup, "choice", {
        (annotation, xs, Option<Annotation>),
        (nested_particle, xs, Vec<NestedParticle>),
    });

    #[derive(Debug, PartialEq)]
    pub struct ComplexTypeLocalComplexType<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub complex_type_model: super::xs::ComplexTypeModel<'input>,
    }

    impl_element!(ComplexTypeLocalComplexType, "complexType", {
        (annotation, xs, Option<Annotation>),
        (complex_type_model, xs, ComplexTypeModel),
    });

    #[derive(Debug, PartialEq)]
    pub struct ElementLocalElement<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub type_: Option<super::enums::Type<'input>>,
        pub alternative_alt_type: Vec<super::inline_elements::AlternativeAltType<'input>>,
        pub identity_constraint: Vec<super::xs::IdentityConstraint<'input>>,
    }

    impl_element!(ElementLocalElement, "element", {
        (annotation, xs, Option<Annotation>),
        (type_, enums, Option<Type>),
        (alternative_alt_type, inline_elements, Vec<AlternativeAltType>),
        (identity_constraint, xs, Vec<IdentityConstraint>),
    });

    #[derive(Debug, PartialEq)]
    pub struct ExtensionSimpleExtensionType<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub attr_decls: super::xs::AttrDecls<'input>,
        pub assertions: super::xs::Assertions<'input>,
    }

    impl_element!(ExtensionSimpleExtensionType, "extension", {
        (annotation, xs, Option<Annotation>),
        (attr_decls, xs, AttrDecls),
        (assertions, xs, Assertions),
    });

    #[derive(Debug, PartialEq)]
    pub struct ExtensionExtensionType<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub open_content: Option<super::xs::OpenContent<'input>>,
        pub type_def_particle: Option<super::xs::TypeDefParticle<'input>>,
        pub attr_decls: super::xs::AttrDecls<'input>,
        pub assertions: super::xs::Assertions<'input>,
    }

    impl_element!(ExtensionExtensionType, "extension", {
        (annotation, xs, Option<Annotation>),
        (open_content, xs, Option<OpenContent>),
        (type_def_particle, xs, Option<TypeDefParticle>),
        (attr_decls, xs, AttrDecls),
        (assertions, xs, Assertions),
    });

    #[derive(Debug, PartialEq)]
    pub struct GroupGroupRef<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(GroupGroupRef, "group", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct GroupSequenceAnnotation<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(GroupSequenceAnnotation, "group", {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct RestrictionComplexRestrictionType<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub choice_sequence_open_content_type_def_particle: Option<super::enums::ChoiceSequenceOpenContentTypeDefParticle<'input>>,
        pub attr_decls: super::xs::AttrDecls<'input>,
        pub assertions: super::xs::Assertions<'input>,
    }

    impl_element!(RestrictionComplexRestrictionType, "restriction", {
        (annotation, xs, Option<Annotation>),
        (choice_sequence_open_content_type_def_particle, enums, Option<ChoiceSequenceOpenContentTypeDefParticle>),
        (attr_decls, xs, AttrDecls),
        (assertions, xs, Assertions),
    });

    #[derive(Debug, PartialEq)]
    pub struct RestrictionSimpleRestrictionType<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub choice_simple_restriction_model: Option<super::enums::ChoiceSimpleRestrictionModel<'input>>,
        pub attr_decls: super::xs::AttrDecls<'input>,
        pub assertions: super::xs::Assertions<'input>,
    }

    impl_element!(RestrictionSimpleRestrictionType, "restriction", {
        (annotation, xs, Option<Annotation>),
        (choice_simple_restriction_model, enums, Option<ChoiceSimpleRestrictionModel>),
        (attr_decls, xs, AttrDecls),
        (assertions, xs, Assertions),
    });

    #[derive(Debug, PartialEq)]
    pub struct SequenceSimpleExplicitGroup<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub nested_particle: Vec<super::xs::NestedParticle<'input>>,
    }

    impl_element!(SequenceSimpleExplicitGroup, "sequence", {
        (annotation, xs, Option<Annotation>),
        (nested_particle, xs, Vec<NestedParticle>),
    });

    #[derive(Debug, PartialEq)]
    pub struct SimpleTypeLocalSimpleType<'input> {
        pub attrs: HashMap<QName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub simple_derivation: super::xs::SimpleDerivation<'input>,
    }

    impl_element!(SimpleTypeLocalSimpleType, "simpleType", {
        (annotation, xs, Option<Annotation>),
        (simple_derivation, xs, SimpleDerivation),
    });
}