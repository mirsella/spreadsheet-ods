//!
//! One sheet of the spreadsheet.
//!

use crate::cell_::{CellContent, CellContentRef, CellData};
use crate::draw::{Annotation, DrawFrame};
use crate::style::{ColStyleRef, RowStyleRef, TableStyleRef};
use crate::validation::ValidationRef;
use crate::value_::Value;
use crate::xmltree::XmlTag;
use crate::{CellRange, CellStyleRef, ColRange, Length, OdsError, RowRange};
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::FusedIterator;
use std::ops::RangeBounds;

/// Visibility of a column or row.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
#[allow(missing_docs)]
pub enum Visibility {
    #[default]
    Visible,
    Collapsed,
    Filtered,
}

impl Display for Visibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Visibility::Visible => write!(f, "visible"),
            Visibility::Collapsed => write!(f, "collapse"),
            Visibility::Filtered => write!(f, "filter"),
        }
    }
}

/// Row data
#[derive(Debug, Clone)]
pub(crate) struct RowHeader {
    pub(crate) style: Option<String>,
    pub(crate) cellstyle: Option<String>,
    pub(crate) visible: Visibility,
    pub(crate) repeat: u32,
    pub(crate) height: Length,
}

impl Default for RowHeader {
    fn default() -> Self {
        Self {
            style: None,
            cellstyle: None,
            visible: Default::default(),
            repeat: 1,
            height: Default::default(),
        }
    }
}

impl RowHeader {
    pub(crate) fn set_style(&mut self, style: &RowStyleRef) {
        self.style = Some(style.to_string());
    }

    pub(crate) fn clear_style(&mut self) {
        self.style = None;
    }

    pub(crate) fn style(&self) -> Option<&String> {
        self.style.as_ref()
    }

    pub(crate) fn set_cellstyle(&mut self, style: &CellStyleRef) {
        self.cellstyle = Some(style.to_string());
    }

    pub(crate) fn clear_cellstyle(&mut self) {
        self.cellstyle = None;
    }

    pub(crate) fn cellstyle(&self) -> Option<&String> {
        self.cellstyle.as_ref()
    }

    pub(crate) fn set_visible(&mut self, visible: Visibility) {
        self.visible = visible;
    }

    pub(crate) fn visible(&self) -> Visibility {
        self.visible
    }

    pub(crate) fn set_repeat(&mut self, repeat: u32) {
        assert!(repeat > 0);
        self.repeat = repeat;
    }

    pub(crate) fn repeat(&self) -> u32 {
        self.repeat
    }

    pub(crate) fn set_height(&mut self, height: Length) {
        self.height = height;
    }

    pub(crate) fn height(&self) -> Length {
        self.height
    }
}

/// Column data
#[derive(Debug, Clone, Default)]
pub(crate) struct ColHeader {
    pub(crate) style: Option<String>,
    pub(crate) cellstyle: Option<String>,
    pub(crate) visible: Visibility,
    pub(crate) width: Length,
}

impl ColHeader {
    pub(crate) fn set_style(&mut self, style: &ColStyleRef) {
        self.style = Some(style.to_string());
    }

    pub(crate) fn clear_style(&mut self) {
        self.style = None;
    }

    pub(crate) fn style(&self) -> Option<&String> {
        self.style.as_ref()
    }

    pub(crate) fn set_cellstyle(&mut self, style: &CellStyleRef) {
        self.cellstyle = Some(style.to_string());
    }

    pub(crate) fn clear_cellstyle(&mut self) {
        self.cellstyle = None;
    }

    pub(crate) fn cellstyle(&self) -> Option<&String> {
        self.cellstyle.as_ref()
    }

    pub(crate) fn set_visible(&mut self, visible: Visibility) {
        self.visible = visible;
    }

    pub(crate) fn visible(&self) -> Visibility {
        self.visible
    }

    pub(crate) fn set_width(&mut self, width: Length) {
        self.width = width;
    }

    pub(crate) fn width(&self) -> Length {
        self.width
    }
}

/// One sheet of the spreadsheet.
///
/// Contains the data and the style-references. The can also be
/// styles on the whole sheet, columns and rows. The more complicated
/// grouping tags are not covered.
#[derive(Clone, Default)]
pub struct Sheet {
    pub(crate) name: String,
    pub(crate) style: Option<String>,

    pub(crate) data: BTreeMap<(u32, u32), CellData>,

    pub(crate) col_header: BTreeMap<u32, ColHeader>,
    pub(crate) row_header: BTreeMap<u32, RowHeader>,

    pub(crate) display: bool,
    pub(crate) print: bool,

    pub(crate) header_rows: Option<RowRange>,
    pub(crate) header_cols: Option<ColRange>,
    pub(crate) print_ranges: Option<Vec<CellRange>>,

    pub(crate) group_rows: Vec<Grouped>,
    pub(crate) group_cols: Vec<Grouped>,

    pub(crate) sheet_config: SheetConfig,

    pub(crate) extra: Vec<XmlTag>,
}

impl<'a> IntoIterator for &'a Sheet {
    type Item = ((u32, u32), CellContentRef<'a>);
    type IntoIter = CellIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CellIter {
            it_data: self.data.iter(),
            k_data: None,
            v_data: None,
        }
    }
}

/// Iterator over cells.
#[derive(Clone, Debug)]
pub struct CellIter<'a> {
    it_data: std::collections::btree_map::Iter<'a, (u32, u32), CellData>,
    k_data: Option<&'a (u32, u32)>,
    v_data: Option<&'a CellData>,
}

impl CellIter<'_> {
    /// Returns the (row,col) of the next cell.
    pub fn peek_cell(&mut self) -> Option<(u32, u32)> {
        self.k_data.copied()
    }

    fn load_next_data(&mut self) {
        if let Some((k, v)) = self.it_data.next() {
            self.k_data = Some(k);
            self.v_data = Some(v);
        } else {
            self.k_data = None;
            self.v_data = None;
        }
    }
}

impl FusedIterator for CellIter<'_> {}

impl<'a> Iterator for CellIter<'a> {
    type Item = ((u32, u32), CellContentRef<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.k_data.is_none() {
            self.load_next_data();
        }

        if let Some(k_data) = self.k_data {
            if let Some(v_data) = self.v_data {
                let r = Some((*k_data, v_data.cell_content_ref()));
                self.load_next_data();
                r
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// Range iterator.
#[derive(Clone, Debug)]
pub struct Range<'a> {
    range: std::collections::btree_map::Range<'a, (u32, u32), CellData>,
}

impl FusedIterator for Range<'_> {}

impl<'a> Iterator for Range<'a> {
    type Item = ((u32, u32), CellContentRef<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((k, v)) = self.range.next() {
            Some((*k, v.cell_content_ref()))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl DoubleEndedIterator for Range<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some((k, v)) = self.range.next_back() {
            Some((*k, v.cell_content_ref()))
        } else {
            None
        }
    }
}

impl ExactSizeIterator for Range<'_> {}

impl fmt::Debug for Sheet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "name {:?} style {:?}", self.name, self.style)?;
        for (k, v) in self.data.iter() {
            writeln!(f, "  data {:?} {:?}", k, v)?;
        }
        for (k, v) in &self.col_header {
            writeln!(f, "{:?} {:?}", k, v)?;
        }
        for (k, v) in &self.row_header {
            writeln!(f, "{:?} {:?}", k, v)?;
        }
        if let Some(header_rows) = &self.header_rows {
            writeln!(f, "header rows {:?}", header_rows)?;
        }
        if let Some(header_cols) = &self.header_cols {
            writeln!(f, "header cols {:?}", header_cols)?;
        }
        for v in &self.group_cols {
            writeln!(f, "group cols {:?}", v)?;
        }
        for v in &self.group_rows {
            writeln!(f, "group rows {:?}", v)?;
        }
        for xtr in &self.extra {
            writeln!(f, "extras {:?}", xtr)?;
        }
        Ok(())
    }
}

impl Sheet {
    /// Create an empty sheet.
    #[deprecated]
    pub fn new_with_name<S: Into<String>>(name: S) -> Self {
        Self::new(name)
    }

    /// Create an empty sheet.
    ///
    /// The name is shown as the tab-title, but also as a reference for
    /// this sheet in formulas and sheet-metadata. Giving an empty string
    /// here is allowed and a name will be generated, when the document is
    /// opened. But any metadata will not be applied.
    ///
    /// Renaming the sheet works for metadata, but formulas will not be fixed.  
    ///
    pub fn new<S: Into<String>>(name: S) -> Self {
        Sheet {
            name: name.into(),
            data: BTreeMap::new(),
            col_header: Default::default(),
            style: None,
            header_rows: None,
            header_cols: None,
            print_ranges: None,
            group_rows: Default::default(),
            group_cols: Default::default(),
            sheet_config: Default::default(),
            extra: vec![],
            row_header: Default::default(),
            display: true,
            print: true,
        }
    }

    /// Copy all the attributes but not the actual data.
    pub fn clone_no_data(&self) -> Self {
        Self {
            name: self.name.clone(),
            style: self.style.clone(),
            data: Default::default(),
            col_header: self.col_header.clone(),
            row_header: self.row_header.clone(),
            display: self.display,
            print: self.print,
            header_rows: self.header_rows.clone(),
            header_cols: self.header_cols.clone(),
            print_ranges: self.print_ranges.clone(),
            group_rows: self.group_rows.clone(),
            group_cols: self.group_cols.clone(),
            sheet_config: Default::default(),
            extra: self.extra.clone(),
        }
    }

    /// Iterate all cells.
    pub fn iter(&self) -> CellIter<'_> {
        self.into_iter()
    }

    /// Count all cells with any data.
    pub fn cell_count(&self) -> usize {
        self.data.len()
    }

    /// Iterate a range of cells.
    pub fn range<R>(&self, range: R) -> Range<'_>
    where
        R: RangeBounds<(u32, u32)>,
    {
        Range {
            range: self.data.range(range),
        }
    }

    /// Sheet name.
    pub fn set_name<V: Into<String>>(&mut self, name: V) {
        self.name = name.into();
    }

    /// Sheet name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Configuration for the sheet.
    pub fn config(&self) -> &SheetConfig {
        &self.sheet_config
    }

    /// Configuration for the sheet.
    pub fn config_mut(&mut self) -> &mut SheetConfig {
        &mut self.sheet_config
    }

    /// Sets the table-style
    pub fn set_style(&mut self, style: &TableStyleRef) {
        self.style = Some(style.to_string());
    }

    /// Returns the table-style.
    pub fn style(&self) -> Option<&String> {
        self.style.as_ref()
    }

    /// Column style.
    pub fn set_colstyle(&mut self, col: u32, style: &ColStyleRef) {
        self.col_header.entry(col).or_default().set_style(style);
    }

    /// Remove the style.
    pub fn clear_colstyle(&mut self, col: u32) {
        self.col_header.entry(col).or_default().clear_style();
    }

    /// Returns the column style.
    pub fn colstyle(&self, col: u32) -> Option<&String> {
        if let Some(col_header) = self.col_header.get(&col) {
            col_header.style()
        } else {
            None
        }
    }

    /// Default cell style for this column.
    pub fn set_col_cellstyle(&mut self, col: u32, style: &CellStyleRef) {
        self.col_header.entry(col).or_default().set_cellstyle(style);
    }

    /// Remove the style.
    pub fn clear_col_cellstyle(&mut self, col: u32) {
        self.col_header.entry(col).or_default().clear_cellstyle();
    }

    /// Returns the default cell style for this column.
    pub fn col_cellstyle(&self, col: u32) -> Option<&String> {
        if let Some(col_header) = self.col_header.get(&col) {
            col_header.cellstyle()
        } else {
            None
        }
    }

    /// Visibility of the column
    pub fn set_col_visible(&mut self, col: u32, visible: Visibility) {
        self.col_header.entry(col).or_default().set_visible(visible);
    }

    /// Returns the default cell style for this column.
    pub fn col_visible(&self, col: u32) -> Visibility {
        if let Some(col_header) = self.col_header.get(&col) {
            col_header.visible()
        } else {
            Default::default()
        }
    }

    /// Sets the column width for this column.
    pub fn set_col_width(&mut self, col: u32, width: Length) {
        self.col_header.entry(col).or_default().set_width(width);
    }

    /// Returns the column-width.
    pub fn col_width(&self, col: u32) -> Length {
        if let Some(ch) = self.col_header.get(&col) {
            ch.width()
        } else {
            Length::Default
        }
    }

    /// Row style.
    pub fn set_rowstyle(&mut self, row: u32, style: &RowStyleRef) {
        self.row_header.entry(row).or_default().set_style(style);
    }

    /// Remove the style.
    pub fn clear_rowstyle(&mut self, row: u32) {
        self.row_header.entry(row).or_default().clear_style();
    }

    /// Returns the row style.
    pub fn rowstyle(&self, row: u32) -> Option<&String> {
        if let Some(row_header) = self.row_header.get(&row) {
            row_header.style()
        } else {
            None
        }
    }

    /// Default cell style for this row.
    pub fn set_row_cellstyle(&mut self, row: u32, style: &CellStyleRef) {
        self.row_header.entry(row).or_default().set_cellstyle(style);
    }

    /// Remove the style.
    pub fn clear_row_cellstyle(&mut self, row: u32) {
        self.row_header.entry(row).or_default().clear_cellstyle();
    }

    /// Returns the default cell style for this row.
    pub fn row_cellstyle(&self, row: u32) -> Option<&String> {
        if let Some(row_header) = self.row_header.get(&row) {
            row_header.cellstyle()
        } else {
            None
        }
    }

    /// Visibility of the row
    pub fn set_row_visible(&mut self, row: u32, visible: Visibility) {
        self.row_header.entry(row).or_default().set_visible(visible);
    }

    /// Returns the default cell style for this row.
    pub fn row_visible(&self, row: u32) -> Visibility {
        if let Some(row_header) = self.row_header.get(&row) {
            row_header.visible()
        } else {
            Default::default()
        }
    }

    /// Sets the repeat count for this row. Usually this is the last row
    /// with data in a sheet. Setting the repeat count will not change
    /// the row number of following rows. But they will be changed after
    /// writing to an ODS file and reading it again.
    ///
    /// Panics
    ///
    /// Panics if the repeat is 0.
    pub fn set_row_repeat(&mut self, row: u32, repeat: u32) {
        self.row_header.entry(row).or_default().set_repeat(repeat)
    }

    /// Returns the repeat count for this row.
    pub fn row_repeat(&self, row: u32) -> u32 {
        if let Some(row_header) = self.row_header.get(&row) {
            row_header.repeat()
        } else {
            Default::default()
        }
    }

    /// Sets the row-height.
    pub fn set_row_height(&mut self, row: u32, height: Length) {
        self.row_header.entry(row).or_default().set_height(height);
    }

    /// Returns the row-height
    pub fn row_height(&self, row: u32) -> Length {
        if let Some(rh) = self.row_header.get(&row) {
            rh.height()
        } else {
            Length::Default
        }
    }

    /// Returns the maximum used column +1 in the column header
    pub fn used_cols(&self) -> u32 {
        *self.col_header.keys().max().unwrap_or(&0) + 1
    }

    /// Returns the maximum used row +1 in the row header
    pub fn used_rows(&self) -> u32 {
        *self.row_header.keys().max().unwrap_or(&0) + 1
    }

    /// Returns a tuple of (max(row)+1, max(col)+1)
    pub fn used_grid_size(&self) -> (u32, u32) {
        let max = self.data.keys().fold((0, 0), |mut max, (r, c)| {
            max.0 = u32::max(max.0, *r);
            max.1 = u32::max(max.1, *c);
            max
        });

        (max.0 + 1, max.1 + 1)
    }

    /// Is the sheet displayed?
    pub fn set_display(&mut self, display: bool) {
        self.display = display;
    }

    /// Is the sheet displayed?
    pub fn display(&self) -> bool {
        self.display
    }

    /// Is the sheet printed?
    pub fn set_print(&mut self, print: bool) {
        self.print = print;
    }

    /// Is the sheet printed?
    pub fn print(&self) -> bool {
        self.print
    }

    /// Returns true if there is no SCell at the given position.
    pub fn is_empty(&self, row: u32, col: u32) -> bool {
        self.data.get(&(row, col)).is_none()
    }

    /// Basic range operator.
    // pub fn cell_range<R>(&self, range: R)
    // where
    //     R: RangeBounds<(ucell, ucell)>,
    // {
    //     let r = self.data.range(range);
    // }

    /// Returns a copy of the cell content.
    pub fn cell(&self, row: u32, col: u32) -> Option<CellContent> {
        self.data
            .get(&(row, col))
            .map(CellData::cloned_cell_content)
    }

    /// Returns a copy of the cell content.
    pub fn cell_ref(&self, row: u32, col: u32) -> Option<CellContentRef<'_>> {
        self.data.get(&(row, col)).map(CellData::cell_content_ref)
    }

    /// Consumes the CellContent and sets the values.
    pub fn add_cell(&mut self, row: u32, col: u32, cell: CellContent) {
        self.add_cell_data(row, col, cell.into_celldata());
    }

    /// Removes the cell and returns the values as CellContent.
    pub fn remove_cell(&mut self, row: u32, col: u32) -> Option<CellContent> {
        self.data
            .remove(&(row, col))
            .map(CellData::into_cell_content)
    }

    /// Add a new cell. Main use is for reading the spreadsheet.
    pub(crate) fn add_cell_data(&mut self, row: u32, col: u32, cell: CellData) {
        self.data.insert((row, col), cell);
    }

    /// Sets a value for the specified cell. Creates a new cell if necessary.
    pub fn set_styled_value<V: Into<Value>>(
        &mut self,
        row: u32,
        col: u32,
        value: V,
        style: &CellStyleRef,
    ) {
        let cell = self.data.entry((row, col)).or_default();
        cell.value = value.into();
        cell.style = Some(style.to_string());
    }

    /// Sets a value for the specified cell. Creates a new cell if necessary.
    pub fn set_value<V: Into<Value>>(&mut self, row: u32, col: u32, value: V) {
        let cell = self.data.entry((row, col)).or_default();
        cell.value = value.into();
    }

    /// Returns a value
    pub fn value(&self, row: u32, col: u32) -> &Value {
        if let Some(cell) = self.data.get(&(row, col)) {
            &cell.value
        } else {
            &Value::Empty
        }
    }

    /// Sets a formula for the specified cell. Creates a new cell if necessary.
    pub fn set_formula<V: Into<String>>(&mut self, row: u32, col: u32, formula: V) {
        let cell = self.data.entry((row, col)).or_default();
        cell.formula = Some(formula.into());
    }

    /// Removes the formula.
    pub fn clear_formula(&mut self, row: u32, col: u32) {
        if let Some(cell) = self.data.get_mut(&(row, col)) {
            cell.formula = None;
        }
    }

    /// Returns a value
    pub fn formula(&self, row: u32, col: u32) -> Option<&String> {
        if let Some(c) = self.data.get(&(row, col)) {
            c.formula.as_ref()
        } else {
            None
        }
    }

    /// Sets a repeat counter for the cell.
    pub fn set_col_repeat(&mut self, row: u32, col: u32, repeat: u32) {
        let cell = self.data.entry((row, col)).or_default();
        cell.repeat = repeat;
    }

    /// Returns the repeat counter for the cell.
    pub fn col_repeat(&self, row: u32, col: u32) -> u32 {
        if let Some(c) = self.data.get(&(row, col)) {
            c.repeat
        } else {
            1
        }
    }

    /// Sets the cell-style for the specified cell. Creates a new cell if necessary.
    pub fn set_cellstyle(&mut self, row: u32, col: u32, style: &CellStyleRef) {
        let cell = self.data.entry((row, col)).or_default();
        cell.style = Some(style.to_string());
    }

    /// Removes the cell-style.
    pub fn clear_cellstyle(&mut self, row: u32, col: u32) {
        if let Some(cell) = self.data.get_mut(&(row, col)) {
            cell.style = None;
        }
    }

    /// Returns a value
    pub fn cellstyle(&self, row: u32, col: u32) -> Option<&String> {
        if let Some(c) = self.data.get(&(row, col)) {
            c.style.as_ref()
        } else {
            None
        }
    }

    /// Sets a content-validation for this cell.
    pub fn set_validation(&mut self, row: u32, col: u32, validation: &ValidationRef) {
        let cell = self.data.entry((row, col)).or_default();
        cell.extra_mut().validation_name = Some(validation.to_string());
    }

    /// Removes the cell-style.
    pub fn clear_validation(&mut self, row: u32, col: u32) {
        if let Some(cell) = self.data.get_mut(&(row, col)) {
            cell.extra_mut().validation_name = None;
        }
    }

    /// Returns a content-validation name for this cell.
    pub fn validation(&self, row: u32, col: u32) -> Option<&String> {
        if let Some(CellData { extra: Some(c), .. }) = self.data.get(&(row, col)) {
            c.validation_name.as_ref()
        } else {
            None
        }
    }

    /// Sets the rowspan of the cell. Must be greater than 0.
    pub fn set_row_span(&mut self, row: u32, col: u32, span: u32) {
        let cell = self.data.entry((row, col)).or_default();
        cell.extra_mut().span.set_row_span(span);
    }

    /// Rowspan of the cell.
    pub fn row_span(&self, row: u32, col: u32) -> u32 {
        if let Some(CellData { extra: Some(c), .. }) = self.data.get(&(row, col)) {
            c.span.row_span()
        } else {
            1
        }
    }

    /// Sets the colspan of the cell. Must be greater than 0.
    pub fn set_col_span(&mut self, row: u32, col: u32, span: u32) {
        assert!(span > 0);
        let cell = self.data.entry((row, col)).or_default();
        cell.extra_mut().span.set_col_span(span);
    }

    /// Colspan of the cell.
    pub fn col_span(&self, row: u32, col: u32) -> u32 {
        if let Some(CellData { extra: Some(c), .. }) = self.data.get(&(row, col)) {
            c.span.col_span()
        } else {
            1
        }
    }

    /// Sets the rowspan of the cell. Must be greater than 0.
    pub fn set_matrix_row_span(&mut self, row: u32, col: u32, span: u32) {
        let cell = self.data.entry((row, col)).or_default();
        cell.extra_mut().matrix_span.set_row_span(span);
    }

    /// Rowspan of the cell.
    pub fn matrix_row_span(&self, row: u32, col: u32) -> u32 {
        if let Some(CellData { extra: Some(c), .. }) = self.data.get(&(row, col)) {
            c.matrix_span.row_span()
        } else {
            1
        }
    }

    /// Sets the colspan of the cell. Must be greater than 0.
    pub fn set_matrix_col_span(&mut self, row: u32, col: u32, span: u32) {
        assert!(span > 0);
        let cell = self.data.entry((row, col)).or_default();
        cell.extra_mut().matrix_span.set_col_span(span);
    }

    /// Colspan of the cell.
    pub fn matrix_col_span(&self, row: u32, col: u32) -> u32 {
        if let Some(CellData { extra: Some(c), .. }) = self.data.get(&(row, col)) {
            c.matrix_span.col_span()
        } else {
            1
        }
    }

    /// Sets a annotation for this cell.
    pub fn set_annotation(&mut self, row: u32, col: u32, annotation: Annotation) {
        let cell = self.data.entry((row, col)).or_default();
        cell.extra_mut().annotation = Some(annotation);
    }

    /// Removes the annotation.
    pub fn clear_annotation(&mut self, row: u32, col: u32) {
        if let Some(cell) = self.data.get_mut(&(row, col)) {
            cell.extra_mut().annotation = None;
        }
    }

    /// Returns a content-validation name for this cell.
    pub fn annotation(&self, row: u32, col: u32) -> Option<&Annotation> {
        if let Some(CellData { extra: Some(c), .. }) = self.data.get(&(row, col)) {
            c.annotation.as_ref()
        } else {
            None
        }
    }

    /// Returns a content-validation name for this cell.
    pub fn annotation_mut(&mut self, row: u32, col: u32) -> Option<&mut Annotation> {
        if let Some(CellData { extra: Some(c), .. }) = self.data.get_mut(&(row, col)) {
            c.annotation.as_mut()
        } else {
            None
        }
    }

    /// Add a drawframe to a specific cell.
    pub fn add_draw_frame(&mut self, row: u32, col: u32, draw_frame: DrawFrame) {
        let cell = self.data.entry((row, col)).or_default();
        cell.extra_mut().draw_frames.push(draw_frame);
    }

    /// Removes all drawframes.
    pub fn clear_draw_frames(&mut self, row: u32, col: u32) {
        if let Some(cell) = self.data.get_mut(&(row, col)) {
            cell.extra_mut().draw_frames = Vec::new();
        }
    }

    /// Returns the draw-frames.
    pub fn draw_frames(&self, row: u32, col: u32) -> Option<&Vec<DrawFrame>> {
        if let Some(CellData { extra: Some(c), .. }) = self.data.get(&(row, col)) {
            Some(c.draw_frames.as_ref())
        } else {
            None
        }
    }

    /// Returns a content-validation name for this cell.
    pub fn draw_frames_mut(&mut self, row: u32, col: u32) -> Option<&mut Vec<DrawFrame>> {
        if let Some(CellData { extra: Some(c), .. }) = self.data.get_mut(&(row, col)) {
            Some(c.draw_frames.as_mut())
        } else {
            None
        }
    }

    /// Defines a range of rows as header rows.
    pub fn set_header_rows(&mut self, row_start: u32, row_end: u32) {
        self.header_rows = Some(RowRange::new(row_start, row_end));
    }

    /// Clears the header-rows definition.
    pub fn clear_header_rows(&mut self) {
        self.header_rows = None;
    }

    /// Returns the header rows.
    pub fn header_rows(&self) -> &Option<RowRange> {
        &self.header_rows
    }

    /// Defines a range of columns as header columns.
    pub fn set_header_cols(&mut self, col_start: u32, col_end: u32) {
        self.header_cols = Some(ColRange::new(col_start, col_end));
    }

    /// Clears the header-columns definition.
    pub fn clear_header_cols(&mut self) {
        self.header_cols = None;
    }

    /// Returns the header columns.
    pub fn header_cols(&self) -> &Option<ColRange> {
        &self.header_cols
    }

    /// Print ranges.
    pub fn add_print_range(&mut self, range: CellRange) {
        self.print_ranges.get_or_insert_with(Vec::new).push(range);
    }

    /// Remove print ranges.
    pub fn clear_print_ranges(&mut self) {
        self.print_ranges = None;
    }

    /// Return the print ranges.
    pub fn print_ranges(&self) -> Option<&Vec<CellRange>> {
        self.print_ranges.as_ref()
    }

    /// Split horizontally on a cell boundary. The splitting is fixed in
    /// position.
    pub fn split_col_header(&mut self, col: u32) {
        self.config_mut().hor_split_mode = SplitMode::Heading;
        self.config_mut().hor_split_pos = col + 1;
        self.config_mut().position_right = col + 1;
        self.config_mut().cursor_x = col + 1;
    }

    /// Split vertically on a cell boundary. The splitting is fixed in
    /// position.
    pub fn split_row_header(&mut self, row: u32) {
        self.config_mut().vert_split_mode = SplitMode::Heading;
        self.config_mut().vert_split_pos = row + 1;
        self.config_mut().position_bottom = row + 1;
        self.config_mut().cursor_y = row + 1;
    }

    /// Split horizontally with a pixel width. The split can be moved around.
    /// For more control look at SheetConfig.
    pub fn split_horizontal(&mut self, col: u32) {
        self.config_mut().hor_split_mode = SplitMode::Split;
        self.config_mut().hor_split_pos = col;
    }

    /// Split vertically with a pixel width. The split can be moved around.
    /// For more control look at SheetConfig.
    pub fn split_vertical(&mut self, col: u32) {
        self.config_mut().vert_split_mode = SplitMode::Split;
        self.config_mut().vert_split_pos = col;
    }

    /// Add a column group.
    ///
    /// Panic
    ///
    /// Column groups can be contained within another, but they can't overlap.
    /// From must be less than or equal to.
    pub fn add_col_group(&mut self, from: u32, to: u32) {
        assert!(from <= to);
        let grp = Grouped::new(from, to, true);
        for v in &self.group_cols {
            assert!(grp.contains(v) || v.contains(&grp) || grp.disjunct(v));
        }
        self.group_cols.push(grp);
    }

    /// Remove a column group.
    pub fn remove_col_group(&mut self, from: u32, to: u32) {
        if let Some(idx) = self
            .group_cols
            .iter()
            .position(|v| v.from == from && v.to == to)
        {
            self.group_cols.remove(idx);
        }
    }

    /// Change the expansion/collapse of a col group.
    ///
    /// Does nothing if no such group exists.
    pub fn set_col_group_displayed(&mut self, from: u32, to: u32, display: bool) {
        if let Some(idx) = self
            .group_cols
            .iter()
            .position(|v| v.from == from && v.to == to)
        {
            self.group_cols[idx].display = display;

            for r in from..=to {
                self.set_col_visible(
                    r,
                    if display {
                        Visibility::Visible
                    } else {
                        Visibility::Collapsed
                    },
                );
            }
        }
    }

    /// Count of column groups.
    pub fn col_group_count(&self) -> usize {
        self.group_cols.len()
    }

    /// Returns the nth column group.
    pub fn col_group(&self, idx: usize) -> Option<&Grouped> {
        return self.group_cols.get(idx);
    }

    /// Returns the nth column group.
    pub fn col_group_mut(&mut self, idx: usize) -> Option<&mut Grouped> {
        return self.group_cols.get_mut(idx);
    }

    /// Iterate the column groups.
    pub fn col_group_iter(&self) -> impl Iterator<Item = &Grouped> {
        self.group_cols.iter()
    }

    /// Add a row group.
    ///
    /// Panic
    ///
    /// Row groups can be contained within another, but they can't overlap.
    /// From must be less than or equal to.
    pub fn add_row_group(&mut self, from: u32, to: u32) {
        assert!(from <= to);
        let grp = Grouped::new(from, to, true);
        for v in &self.group_rows {
            assert!(grp.contains(v) || v.contains(&grp) || grp.disjunct(v));
        }
        self.group_rows.push(grp);
    }

    /// Remove a row group.
    pub fn remove_row_group(&mut self, from: u32, to: u32) {
        if let Some(idx) = self
            .group_rows
            .iter()
            .position(|v| v.from == from && v.to == to)
        {
            self.group_rows.remove(idx);
        }
    }

    /// Change the expansion/collapse of a row group.
    ///
    /// Does nothing if no such group exists.
    pub fn set_row_group_displayed(&mut self, from: u32, to: u32, display: bool) {
        if let Some(idx) = self
            .group_rows
            .iter()
            .position(|v| v.from == from && v.to == to)
        {
            self.group_rows[idx].display = display;

            for r in from..=to {
                self.set_row_visible(
                    r,
                    if display {
                        Visibility::Visible
                    } else {
                        Visibility::Collapsed
                    },
                );
            }
        }
    }

    /// Count of row groups.
    pub fn row_group_count(&self) -> usize {
        self.group_rows.len()
    }

    /// Returns the nth row group.
    pub fn row_group(&self, idx: usize) -> Option<&Grouped> {
        return self.group_rows.get(idx);
    }

    /// Iterate row groups.
    pub fn row_group_iter(&self) -> impl Iterator<Item = &Grouped> {
        self.group_rows.iter()
    }
}

/// Describes a row/column group.
#[derive(Debug, PartialEq, Clone)]
pub struct Grouped {
    /// Inclusive from row/col.
    pub from: u32,
    /// Inclusive to row/col.
    pub to: u32,
    /// Visible/Collapsed state.
    pub display: bool,
}

impl Grouped {
    /// New group.
    pub fn new(from: u32, to: u32, display: bool) -> Self {
        Self { from, to, display }
    }

    /// Inclusive start.
    pub fn from(&self) -> u32 {
        self.from
    }

    /// Inclusive start.
    pub fn set_from(&mut self, from: u32) {
        self.from = from;
    }

    /// Inclusive end.
    pub fn to(&self) -> u32 {
        self.to
    }

    /// Inclusive end.
    pub fn set_to(&mut self, to: u32) {
        self.to = to
    }

    /// Group is displayed?
    pub fn display(&self) -> bool {
        self.display
    }

    /// Change the display state for the group.
    ///
    /// Note: Changing this does not change the visibility of the rows/columns.
    /// Use Sheet::set_display_col_group() and Sheet::set_display_row_group() to make
    /// all necessary changes.
    pub fn set_display(&mut self, display: bool) {
        self.display = display;
    }

    /// Contains the other group.
    pub fn contains(&self, other: &Grouped) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    /// The two groups are disjunct.
    pub fn disjunct(&self, other: &Grouped) -> bool {
        self.from < other.from && self.to < other.from || self.from > other.to && self.to > other.to
    }
}

/// There are two ways a sheet can be split. There are fixed column/row header
/// like splits, and there is a moveable split.
///
#[derive(Clone, Copy, Debug)]
#[allow(missing_docs)]
pub enum SplitMode {
    None = 0,
    Split = 1,
    Heading = 2,
}

impl TryFrom<i16> for SplitMode {
    type Error = OdsError;

    fn try_from(n: i16) -> Result<Self, Self::Error> {
        match n {
            0 => Ok(SplitMode::None),
            1 => Ok(SplitMode::Split),
            2 => Ok(SplitMode::Heading),
            _ => Err(OdsError::Ods(format!("Invalid split mode {}", n))),
        }
    }
}

/// Per sheet configurations.
#[derive(Clone, Debug)]
pub struct SheetConfig {
    /// Active column.
    pub cursor_x: u32,
    /// Active row.
    pub cursor_y: u32,
    /// Splitting the table.
    pub hor_split_mode: SplitMode,
    /// Splitting the table.
    pub vert_split_mode: SplitMode,
    /// Position of the split.
    pub hor_split_pos: u32,
    /// Position of the split.
    pub vert_split_pos: u32,
    /// SplitMode is Pixel
    /// - 0-4 indicates the quadrant where the focus is.
    /// SplitMode is Cell
    /// - No real function.
    pub active_split_range: i16,
    /// SplitMode is Pixel
    /// - First visible column in the left quadrant.
    /// SplitMode is Cell
    /// - The first visible column in the left quadrant.
    ///   AND every column left of this one is simply invisible.
    pub position_left: u32,
    /// SplitMode is Pixel
    /// - First visible column in the right quadrant.
    /// SplitMode is Cell
    /// - The first visible column in the right quadrant.
    pub position_right: u32,
    /// SplitMode is Pixel
    /// - First visible row in the top quadrant.
    /// SplitMode is Cell
    /// - The first visible row in the top quadrant.
    ///   AND every row up from this one is simply invisible.
    pub position_top: u32,
    /// SplitMode is Pixel
    /// - The first visible row in teh right quadrant.
    /// SplitMode is Cell
    /// - The first visible row in the bottom quadrant.
    pub position_bottom: u32,
    /// If 0 then zoom_value denotes a percentage.
    /// If 2 then zoom_value is 50%???
    pub zoom_type: i16,
    /// Value of zoom.
    pub zoom_value: i32,
    /// Value of pageview zoom.
    pub page_view_zoom_value: i32,
    /// Grid is showing.
    pub show_grid: bool,
}

impl Default for SheetConfig {
    fn default() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            hor_split_mode: SplitMode::None,
            vert_split_mode: SplitMode::None,
            hor_split_pos: 0,
            vert_split_pos: 0,
            active_split_range: 2,
            position_left: 0,
            position_right: 0,
            position_top: 0,
            position_bottom: 0,
            zoom_type: 0,
            zoom_value: 100,
            page_view_zoom_value: 60,
            show_grid: true,
        }
    }
}
