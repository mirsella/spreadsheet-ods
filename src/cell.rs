use crate::draw::{Annotation, DrawFrame};
use crate::validation::ValidationRef;
use crate::value::Value;
use crate::CellStyleRef;

/// A cell can span multiple rows/columns.
#[derive(Debug, Clone, Copy)]
pub struct CellSpan {
    pub(crate) row_span: u32,
    pub(crate) col_span: u32,
}

impl Default for CellSpan {
    fn default() -> Self {
        Self {
            row_span: 1,
            col_span: 1,
        }
    }
}

impl From<CellSpan> for (u32, u32) {
    fn from(span: CellSpan) -> Self {
        (span.row_span, span.col_span)
    }
}

impl From<&CellSpan> for (u32, u32) {
    fn from(span: &CellSpan) -> Self {
        (span.row_span, span.col_span)
    }
}

impl CellSpan {
    /// Default span 1,1
    pub fn new() -> Self {
        Self::default()
    }

    /// Is this empty? Defined as row_span==1 and col_span==1.
    pub fn is_empty(&self) -> bool {
        self.row_span == 1 && self.col_span == 1
    }

    /// Sets the row span of this cell.
    /// Cells below with values will be lost when writing.
    pub fn set_row_span(&mut self, rows: u32) {
        assert!(rows > 0);
        self.row_span = rows;
    }

    /// Returns the row span.
    pub fn row_span(&self) -> u32 {
        self.row_span
    }

    /// Sets the column span of this cell.
    /// Cells to the right with values will be lost when writing.
    pub fn set_col_span(&mut self, cols: u32) {
        assert!(cols > 0);
        self.col_span = cols;
    }

    /// Returns the col span.
    pub fn col_span(&self) -> u32 {
        self.col_span
    }
}

/// One Cell of the spreadsheet.
#[derive(Debug, Clone)]
pub(crate) struct CellData {
    pub(crate) value: Value,
    // Unparsed formula string.
    pub(crate) formula: Option<String>,
    // Cell style name.
    pub(crate) style: Option<String>,
    // Cell repeated.
    pub(crate) repeat: u32,
    // Scarcely used extra data.
    pub(crate) extra: Option<Box<CellDataExt>>,
}

/// Extra cell data.
#[derive(Debug, Clone, Default)]
pub(crate) struct CellDataExt {
    // Content validation name.
    pub(crate) validation_name: Option<String>,
    // Row/Column span.
    pub(crate) span: CellSpan,
    // Matrix span.
    pub(crate) matrix_span: CellSpan,
    // Annotation
    pub(crate) annotation: Option<Annotation>,
    // Draw
    pub(crate) draw_frames: Vec<DrawFrame>,
}

impl Default for CellData {
    fn default() -> Self {
        Self {
            value: Default::default(),
            formula: None,
            style: None,
            repeat: 1,
            extra: None,
        }
    }
}

impl CellData {
    pub(crate) fn extra_mut(&mut self) -> &mut CellDataExt {
        if self.extra.is_none() {
            self.extra = Some(Box::default());
        }
        self.extra.as_mut().expect("celldataext")
    }

    pub(crate) fn cloned_cell_content(&self) -> CellContent {
        let (validation_name, span, matrix_span, annotation, draw_frames) =
            if let Some(extra) = &self.extra {
                (
                    extra.validation_name.clone(),
                    extra.span,
                    extra.matrix_span,
                    extra.annotation.clone(),
                    extra.draw_frames.clone(),
                )
            } else {
                (
                    None,
                    Default::default(),
                    Default::default(),
                    None,
                    Vec::new(),
                )
            };

        CellContent {
            value: self.value.clone(),
            style: self.style.clone(),
            formula: self.formula.clone(),
            repeat: self.repeat,
            validation_name,
            span,
            matrix_span,
            annotation,
            draw_frames,
        }
    }

    pub(crate) fn into_cell_content(self) -> CellContent {
        let (validation_name, span, matrix_span, annotation, draw_frames) =
            if let Some(extra) = self.extra {
                (
                    extra.validation_name,
                    extra.span,
                    extra.matrix_span,
                    extra.annotation,
                    extra.draw_frames,
                )
            } else {
                (
                    None,
                    Default::default(),
                    Default::default(),
                    None,
                    Vec::new(),
                )
            };

        CellContent {
            value: self.value,
            style: self.style,
            formula: self.formula,
            repeat: self.repeat,
            validation_name,
            span,
            matrix_span,
            annotation,
            draw_frames,
        }
    }

    pub(crate) fn cell_content_ref(&self) -> CellContentRef<'_> {
        let (validation_name, span, matrix_span, annotation, draw_frames) =
            if let Some(extra) = &self.extra {
                (
                    extra.validation_name.as_ref(),
                    Some(&extra.span),
                    Some(&extra.matrix_span),
                    extra.annotation.as_ref(),
                    Some(&extra.draw_frames),
                )
            } else {
                (None, None, None, None, None)
            };

        CellContentRef {
            value: &self.value,
            style: self.style.as_ref(),
            formula: self.formula.as_ref(),
            repeat: &self.repeat,
            validation_name,
            span,
            matrix_span,
            annotation,
            draw_frames,
        }
    }
}

/// Holds references to the combined content of a cell.
/// A temporary to hold the data when iterating over a sheet.
#[derive(Debug, Clone, Copy)]
pub struct CellContentRef<'a> {
    /// Reference to the cell value.
    pub value: &'a Value,
    /// Reference to the stylename.
    pub style: Option<&'a String>,
    /// Reference to the cell formula.
    pub formula: Option<&'a String>,
    /// Reference to the repeat count.
    pub repeat: &'a u32,
    /// Reference to a cell validation.
    pub validation_name: Option<&'a String>,
    /// Reference to the cellspan.
    pub span: Option<&'a CellSpan>,
    /// Reference to a matrix cellspan.
    pub matrix_span: Option<&'a CellSpan>,
    /// Reference to an annotation.
    pub annotation: Option<&'a Annotation>,
    /// Reference to draw-frames.
    pub draw_frames: Option<&'a Vec<DrawFrame>>,
}

impl<'a> CellContentRef<'a> {
    /// Returns the value.
    pub fn value(&self) -> &'a Value {
        self.value
    }

    /// Returns the formula.
    pub fn formula(&self) -> Option<&'a String> {
        self.formula
    }

    /// Returns the cell style.
    pub fn style(&self) -> Option<&'a String> {
        self.style
    }

    /// Returns the repeat count.
    pub fn repeat(&self) -> &'a u32 {
        self.repeat
    }

    /// Returns the validation name.
    pub fn validation(&self) -> Option<&'a String> {
        self.validation_name
    }

    /// Returns the row span.
    pub fn row_span(&self) -> u32 {
        if let Some(span) = self.span {
            span.row_span
        } else {
            1
        }
    }

    /// Returns the col span.
    pub fn col_span(&self) -> u32 {
        if let Some(span) = self.span {
            span.col_span
        } else {
            1
        }
    }

    /// Returns the row span for a matrix.
    pub fn matrix_row_span(&self) -> u32 {
        if let Some(matrix_span) = self.matrix_span {
            matrix_span.row_span
        } else {
            1
        }
    }

    /// Returns the col span for a matrix.
    pub fn matrix_col_span(&self) -> u32 {
        if let Some(matrix_span) = self.matrix_span {
            matrix_span.col_span
        } else {
            1
        }
    }

    /// Returns the validation name.
    pub fn annotation(&self) -> Option<&'a Annotation> {
        self.annotation
    }

    /// Returns draw frames.
    pub fn draw_frames(&self) -> Option<&'a Vec<DrawFrame>> {
        self.draw_frames
    }
}

/// A copy of the relevant data for a spreadsheet cell.
#[derive(Debug, Clone, Default)]
pub struct CellContent {
    /// Cell value.
    pub value: Value,
    /// Cell stylename.
    pub style: Option<String>,
    /// Cell formula.
    pub formula: Option<String>,
    /// Cell repeat count.
    pub repeat: u32,
    /// Reference to a validation rule.
    pub validation_name: Option<String>,
    /// Cellspan.
    pub span: CellSpan,
    /// Matrix span.
    pub matrix_span: CellSpan,
    /// Annotation
    pub annotation: Option<Annotation>,
    /// DrawFrames
    pub draw_frames: Vec<DrawFrame>,
}

impl CellContent {
    /// Empty.
    pub fn new() -> Self {
        Default::default()
    }

    ///
    pub(crate) fn into_celldata(mut self) -> CellData {
        let extra = self.into_celldata_ext();
        CellData {
            value: self.value,
            formula: self.formula,
            style: self.style,
            repeat: self.repeat,
            extra,
        }
    }

    /// Move stuff into a CellDataExt.
    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn into_celldata_ext(&mut self) -> Option<Box<CellDataExt>> {
        if self.validation_name.is_some()
            || !self.span.is_empty()
            || !self.matrix_span.is_empty()
            || self.annotation.is_some()
            || !self.draw_frames.is_empty()
        {
            Some(Box::new(CellDataExt {
                validation_name: self.validation_name.take(),
                span: self.span,
                matrix_span: self.matrix_span,
                annotation: self.annotation.take(),
                draw_frames: std::mem::take(&mut self.draw_frames),
            }))
        } else {
            None
        }
    }

    /// Returns the value.
    pub fn value(&self) -> &Value {
        &self.value
    }

    /// Sets the value.
    pub fn set_value<V: Into<Value>>(&mut self, value: V) {
        self.value = value.into();
    }

    /// Returns the formula.
    pub fn formula(&self) -> Option<&String> {
        self.formula.as_ref()
    }

    /// Sets the formula.
    pub fn set_formula<V: Into<String>>(&mut self, formula: V) {
        self.formula = Some(formula.into());
    }

    /// Resets the formula.
    pub fn clear_formula(&mut self) {
        self.formula = None;
    }

    /// Returns the cell style.
    pub fn style(&self) -> Option<&String> {
        self.style.as_ref()
    }

    /// Sets the cell style.
    pub fn set_style(&mut self, style: &CellStyleRef) {
        self.style = Some(style.to_string());
    }

    /// Removes the style.
    pub fn clear_style(&mut self) {
        self.style = None;
    }

    /// Sets the repeat count for the cell.
    /// Value must be > 0.
    pub fn set_repeat(&mut self, repeat: u32) {
        assert!(repeat > 0);
        self.repeat = repeat;
    }

    /// Returns the repeat count for the cell.
    pub fn get_repeat(&mut self) -> u32 {
        self.repeat
    }

    /// Returns the validation name.
    pub fn validation(&self) -> Option<&String> {
        self.validation_name.as_ref()
    }

    /// Sets the validation name.
    pub fn set_validation(&mut self, validation: &ValidationRef) {
        self.validation_name = Some(validation.to_string());
    }

    /// No validation.
    pub fn clear_validation(&mut self) {
        self.validation_name = None;
    }

    /// Sets the row span of this cell.
    /// Cells below with values will be lost when writing.
    pub fn set_row_span(&mut self, rows: u32) {
        assert!(rows > 0);
        self.span.row_span = rows;
    }

    /// Returns the row span.
    pub fn row_span(&self) -> u32 {
        self.span.row_span
    }

    /// Sets the column span of this cell.
    /// Cells to the right with values will be lost when writing.
    pub fn set_col_span(&mut self, cols: u32) {
        assert!(cols > 0);
        self.span.col_span = cols;
    }

    /// Returns the col span.
    pub fn col_span(&self) -> u32 {
        self.span.col_span
    }

    /// Sets the row span of this cell.
    /// Cells below with values will be lost when writing.
    pub fn set_matrix_row_span(&mut self, rows: u32) {
        assert!(rows > 0);
        self.matrix_span.row_span = rows;
    }

    /// Returns the row span.
    pub fn matrix_row_span(&self) -> u32 {
        self.matrix_span.row_span
    }

    /// Sets the column span of this cell.
    /// Cells to the right with values will be lost when writing.
    pub fn set_matrix_col_span(&mut self, cols: u32) {
        assert!(cols > 0);
        self.matrix_span.col_span = cols;
    }

    /// Returns the col span.
    pub fn matrix_col_span(&self) -> u32 {
        self.matrix_span.col_span
    }

    /// Annotation
    pub fn set_annotation(&mut self, annotation: Annotation) {
        self.annotation = Some(annotation);
    }

    /// Annotation
    pub fn clear_annotation(&mut self) {
        self.annotation = None;
    }

    /// Returns the Annotation
    pub fn annotation(&self) -> Option<&Annotation> {
        self.annotation.as_ref()
    }

    /// Draw Frames
    pub fn set_draw_frames(&mut self, draw_frames: Vec<DrawFrame>) {
        self.draw_frames = draw_frames;
    }

    /// Draw Frames
    pub fn draw_frames(&self) -> &Vec<DrawFrame> {
        &self.draw_frames
    }
}
