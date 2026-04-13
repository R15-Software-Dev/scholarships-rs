use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub enum SchemaType {
    /// Indicates a `ValueType::String` value.
    #[default]
    String,
    /// Indicates a `ValueType::Number` value.
    Number,
    /// Indicates a `ValueType::List` containing only strings or numbers.
    PrimitiveList,
    /// Indicates a `ValueType::List` containing only maps.
    MapList,
    /// Indicates a `ValueType::Map`.
    Map,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub enum SchemaContainerStyle {
    #[default]
    Header,
    Capsule,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub enum SchemaHeaderStyle {
    #[default]
    None,
    MainHeader,
    SubsectionHeader,
    Bold,
}

/// # Schema Node
///
/// This is used to create a schema that is used in the [`ValueDisplay`] component. It stores all
/// required information for the display, specifically where to find the data and how to display it.
/// It covers all `ValueType` variants, however for data storage and DX it uses a `SchemaType`
/// instead.
///
/// Some examples are as follows, using the fluent builder notation:
///  - Displaying a single `ValueType::String` from the data map at the key `example_one`:
///     ```
///     SchemaNode::new(SchemaType::String).header("The example string")
///     ```
///  - Displaying a `ValueType::List` which contains only `ValueType::String` entries:
///     ```
///     SchemaNode::new(SchemaType::List)
///         .header("The example list")
///         .item_template(SchemaNode::new(SchemaType::String).header(""))
///     ```
///  - Displaying a `ValueType::Map` which contains a name and age:
///     ```
///     SchemaType::new(SchemaType::Map)
///         .header("The example map")
///         .child("first_name", SchemaNode::new(SchemaType::String).header("First Name:"))
///         .child("age", SchemaNode::new(SchemaType::Number).header("Age:"))
///     ```
///  - Put it all together: a `ValueType::List` which contains `ValueType::Map` entries, each of
///    which also contains the name and age from the previous example:
///     ```
///     SchemaNode::new(SchemaType::List)
///         .header("The example map list")
///         .item_template(
///             SchemaNode::new(SchemaType::Map)
///                 .header("The example map from earlier")
///                 .child("first_name", SchemaNode::new(SchemaType::String).header("First Name:"))
///                 .child("age", SchemaNode::new(SchemaType::Number).header("Age:"))
///         )
///     ```
///
/// This schema allows for definitive typing and styling for every level of the data display.
///
/// ## Important Notes
///
/// This schema does not currently support display of a single nested member, like a single map
/// within a list. This means that you will have to display any listed or mapped information as a
/// list. Consider this when designing the data.
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SchemaNode {
    /// The display style of the data. Refer to [`SchemaContainerStyle`] for more information.
    pub container_style: SchemaContainerStyle,
    /// The datatype of the data found using the `data_member` key.
    pub data_type: SchemaType,
    /// The header of this display.
    pub header: String,
    /// The style of the header's display.
    pub header_style: SchemaHeaderStyle,
    /// When used with a SchemaType::List, this determines the datatype and children for every entry
    /// in the list. Some examples include a list of maps, where we use the `item_template` to define
    /// the SchemaType::Map, and all its children. This is then mapped across all entries in the list,
    /// where any entries that do not match the template are skipped.
    pub item_template: Option<Box<SchemaNode>>,
    /// When used with a SchemaType::Map, this determines what children from the map are displayed
    /// and how. It is ignored with all other SchemaType variants.
    pub children: IndexMap<String, SchemaNode>,
    /// The primary key for iteration over maps. Used when there is a list of maps.
    pub primary_key: Option<String>,
}

impl SchemaNode {
    /// Creates a new [`SchemaBuilder`].
    pub fn builder() -> SchemaBuilder {
        SchemaBuilder::default()
    }
}

#[derive(Default, Debug, Clone)]
pub struct SchemaBuilder {
    container_style: SchemaContainerStyle,
    header: String,
    header_style: SchemaHeaderStyle,
}

impl SchemaBuilder {
    pub fn container(mut self, style: SchemaContainerStyle) -> Self {
        self.container_style = style;
        self
    }

    pub fn header(mut self, header: impl Into<String>) -> Self {
        self.header = header.into();
        self
    }

    pub fn header_style(mut self, header_style: SchemaHeaderStyle) -> Self {
        self.header_style = header_style;
        self
    }

    pub fn primitive_list(self) -> SchemaPrimitiveListBuilder {
        SchemaPrimitiveListBuilder::new(self)
    }

    pub fn map_list(self) -> SchemaMapListBuilder {
        SchemaMapListBuilder::new(self)
    }

    pub fn map(self) -> SchemaMapBuilder {
        SchemaMapBuilder::new(self)
    }

    pub fn string(self) -> SchemaPrimitiveBuilder {
        SchemaPrimitiveBuilder::new(self, SchemaType::String)
    }

    pub fn number(self) -> SchemaPrimitiveBuilder {
        SchemaPrimitiveBuilder::new(self, SchemaType::Number)
    }

    fn into_node(self) -> SchemaNode {
        SchemaNode {
            container_style: self.container_style,
            header: self.header,
            header_style: self.header_style,
            ..Default::default()
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct SchemaPrimitiveListBuilder {
    builder: SchemaBuilder,
}

impl SchemaPrimitiveListBuilder {
    fn new(builder: SchemaBuilder) -> Self {
        Self { builder }
    }
}

#[derive(Default, Debug, Clone)]
pub struct SchemaMapListBuilder {
    builder: SchemaBuilder,
    item_template: Option<Box<SchemaNode>>,
    primary_key: String,
}

impl SchemaMapListBuilder {
    fn new(builder: SchemaBuilder) -> Self {
        Self {
            builder,
            ..Default::default()
        }
    }

    pub fn item_template(mut self, node: SchemaNode) -> Self {
        self.item_template = Some(Box::new(node));
        self
    }

    pub fn primary_key(mut self, primary_key: impl Into<String>) -> Self {
        self.primary_key = primary_key.into();
        self
    }

    pub fn build(self) -> SchemaNode {
        SchemaNode {
            data_type: SchemaType::MapList,
            item_template: self.item_template,
            primary_key: Some(self.primary_key),
            ..self.builder.into_node()
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct SchemaMapBuilder {
    builder: SchemaBuilder,
    children: IndexMap<String, SchemaNode>,
}

impl SchemaMapBuilder {
    fn new(builder: SchemaBuilder) -> Self {
        Self {
            builder,
            ..Default::default()
        }
    }

    pub fn child(mut self, data_member: impl Into<String>, node: SchemaNode) -> Self {
        self.children.insert(data_member.into(), node);
        self
    }

    pub fn build(self) -> SchemaNode {
        SchemaNode {
            data_type: SchemaType::Map,
            children: self.children,
            ..self.builder.into_node()
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct SchemaPrimitiveBuilder {
    builder: SchemaBuilder,
    data_type: SchemaType,
}

impl SchemaPrimitiveBuilder {
    fn new(builder: SchemaBuilder, data_type: SchemaType) -> Self {
        Self { data_type, builder }
    }

    pub fn build(self) -> SchemaNode {
        SchemaNode {
            data_type: self.data_type,
            ..self.builder.into_node()
        }
    }
}
