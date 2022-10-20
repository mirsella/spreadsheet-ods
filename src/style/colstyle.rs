use std::fmt::{Display, Formatter};

use crate::attrmap2::AttrMap2;
use crate::style::units::{Length, PageBreak};
use crate::style::ParseStyleAttr;
use crate::style::{rel_width_string, StyleOrigin, StyleUse};
use crate::OdsError;

style_ref!(ColStyleRef);

/// Describes the style information for a table column.
/// Hardly ever used. It's easier to set the col_width via
/// Sheet::set_col_width
///
#[derive(Debug, Clone)]
pub struct ColStyle {
    /// From where did we get this style.
    origin: StyleOrigin,
    /// Which tag contains this style.
    styleuse: StyleUse,
    /// Style name
    name: String,

    /// General attributes
    // ??? style:auto-update 19.467,
    // ??? style:class 19.470,
    // ignore style:data-style-name 19.473,
    // ignore style:default-outlinelevel 19.474,
    // ignore style:display-name 19.476,
    // ok style:family 19.480,
    // ignore style:list-level 19.499,
    // ignore style:list-style-name 19.500,
    // ignore style:master-page-name 19.501,
    // ok style:name 19.502,
    // ignore style:next-style-name 19.503,
    // ignore style:parent-style-name 19.510,
    // ignore style:percentage-data-style-name 19.511.
    attr: AttrMap2,
    /// Column style properties
    // ok fo:break-after 20.184,
    // ok fo:break-before 20.185,
    // ok style:column-width 20.254,
    // ok style:rel-column-width 20.338
    // ok style:use-optimal-column-width 20.393
    colstyle: AttrMap2,
}

impl ColStyle {
    /// empty
    pub(crate) fn new_empty() -> Self {
        Self {
            origin: Default::default(),
            styleuse: Default::default(),
            name: Default::default(),
            attr: Default::default(),
            colstyle: Default::default(),
        }
    }

    /// New Style.
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            origin: Default::default(),
            styleuse: Default::default(),
            name: name.into(),
            attr: Default::default(),
            colstyle: Default::default(),
        }
    }

    /// Returns a reference.
    pub fn style_ref(&self) -> ColStyleRef {
        ColStyleRef::from(self.name())
    }

    /// Origin. Should always be Content.
    pub fn origin(&self) -> StyleOrigin {
        self.origin
    }

    /// Origin. Should always be Content.
    pub fn set_origin(&mut self, origin: StyleOrigin) {
        self.origin = origin;
    }

    /// Should always be Automatic.
    pub fn styleuse(&self) -> StyleUse {
        self.styleuse
    }

    /// Should always be Automatic.
    pub fn set_styleuse(&mut self, styleuse: StyleUse) {
        self.styleuse = styleuse;
    }

    /// Name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Name.
    pub fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    /// Attributes
    pub(crate) fn attrmap(&self) -> &AttrMap2 {
        &self.attr
    }

    /// Attributes
    pub(crate) fn attrmap_mut(&mut self) -> &mut AttrMap2 {
        &mut self.attr
    }

    /// Style attributes
    pub(crate) fn colstyle(&self) -> &AttrMap2 {
        &self.colstyle
    }

    /// Style attributes
    pub(crate) fn colstyle_mut(&mut self) -> &mut AttrMap2 {
        &mut self.colstyle
    }

    fo_break!(colstyle_mut);

    /// The style:rel-column-width attribute specifies a relative width of a column with a number
    /// value, followed by a ”*” (U+002A, ASTERISK) character. If rc is the relative with of the column, rs
    /// the sum of all relative columns widths, and ws the absolute width that is available for these
    /// columns the absolute width wc of the column is wc=rcws/rs.
    pub fn set_rel_col_width(&mut self, rel: f64) {
        self.colstyle
            .set_attr("style:rel-column-width", rel_width_string(rel));
    }

    /// The style:column-width attribute specifies a fixed width for a column.
    pub fn set_col_width(&mut self, width: Length) {
        if width == Length::Default {
            self.colstyle.clear_attr("style:column-width");
        } else {
            self.colstyle
                .set_attr("style:column-width", width.to_string());
        }
    }

    /// Parses the column width.
    pub fn col_width(&self) -> Result<Length, OdsError> {
        Length::parse_attr_def(self.colstyle.attr("style:column-width"), Length::Default)
    }

    /// The style:use-optimal-column-width attribute specifies that a column width should be
    /// recalculated automatically if content in the column changes.
    pub fn set_use_optimal_col_width(&mut self, opt: bool) {
        self.colstyle
            .set_attr("style:use-optimal-column-width", opt.to_string());
    }

    /// Parses the flag.
    pub fn use_optimal_col_width(&self) -> Result<bool, OdsError> {
        bool::parse_attr_def(self.colstyle.attr("style:use-optimal-column-width"), false)
    }
}
