macro_rules! styles_styles {
    ($style:ident, $styleref:ident) => {
        impl Style for $style {
            type Ref = $styleref;

            /// Origin of the style, either styles.xml oder content.xml
            fn origin(&self) -> StyleOrigin {
                self.origin
            }

            /// Changes the origin.
            fn set_origin(&mut self, origin: StyleOrigin) {
                self.origin = origin;
            }

            /// Usage for the style.
            fn styleuse(&self) -> StyleUse {
                self.styleuse
            }

            /// Usage for the style.
            fn set_styleuse(&mut self, styleuse: StyleUse) {
                self.styleuse = styleuse;
            }

            /// Stylename
            fn name(&self) -> &str {
                &self.name
            }

            /// Stylename
            fn set_name<S: Into<String>>(&mut self, name: S) {
                self.name = name.into();
            }

            /// Returns the name as a CellStyleRef.
            fn style_ref(&self) -> <Self as Style>::Ref {
                <Self as Style>::Ref::from(self.name())
            }

            /// Display name.
            fn display_name(&self) -> Option<&String> {
                self.attr.attr("style:display-name")
            }

            /// Display name.
            fn set_display_name<S: Into<String>>(&mut self, name: S) {
                self.attr.set_attr("style:display-name", name.into());
            }

            /// The parent style this derives from.
            fn parent_style(&self) -> Option<<Self as Style>::Ref> {
                self.attr
                    .attr("style:parent-style-name")
                    .map(|v| $styleref::from(v))
            }

            /// The parent style this derives from.
            fn set_parent_style(&mut self, name: &<Self as Style>::Ref) {
                self.attr
                    .set_attr("style:parent-style-name", name.to_string());
            }

            fn class(&self) -> Option<&String> {
                self.attr.attr("style:class")
            }

            fn set_class<S: Into<String>>(&mut self, class: S) {
                self.attr.set_attr("style:class", class.into());
            }

            fn auto_update(&self) -> Option<bool> {
                self.attr.attr("style:auto-update").map(|v| v == "true")
            }

            fn set_auto_update(&mut self, auto: bool) {
                self.attr.set_attr("style:auto-update", auto.to_string());
            }
        }
    };
}

macro_rules! fo_background_color {
    ($acc:ident) => {
        /// The fo:background-color attribute specifies a background color for characters, paragraphs,
        /// text sections, frames, page bodies, headers, footers, table cells, table rows and tables. This can
        /// be transparent or a color. If the value is set to transparent, it switches off any background
        /// image that is specified by a <style:background-image> 17.3.
        ///
        /// If a value for a draw:fill attribute is provided in a style, any background image that is specified
        /// by a <style:background-image> element and any background color that is specified with the
        /// fo:background-color attribute are switched off.
        pub fn set_background_color(&mut self, color: Rgb<u8>) {
            self.$acc()
                .set_attr("fo:background-color", color_string(color));
        }
    };
}

macro_rules! fo_border {
    ($acc:ident) => {
        /// Border style all four sides. See §7.29.3 of [XSL].
        pub fn set_border(&mut self, width: Length, border: Border, color: Rgb<u8>) {
            self.$acc()
                .set_attr("fo:border", border_string(width, border, color));
        }

        /// Border style. See §7.29.4 of [XSL]
        pub fn set_border_bottom(&mut self, width: Length, border: Border, color: Rgb<u8>) {
            self.$acc()
                .set_attr("fo:border-bottom", border_string(width, border, color));
        }

        /// Border style. See §7.29.6 of [XSL].
        pub fn set_border_left(&mut self, width: Length, border: Border, color: Rgb<u8>) {
            self.$acc()
                .set_attr("fo:border-left", border_string(width, border, color));
        }

        /// Border style. See §7.29.7 of [XSL].
        pub fn set_border_right(&mut self, width: Length, border: Border, color: Rgb<u8>) {
            self.$acc()
                .set_attr("fo:border-right", border_string(width, border, color));
        }

        /// Border style. See §7.29.10 of [XSL].
        pub fn set_border_top(&mut self, width: Length, border: Border, color: Rgb<u8>) {
            self.$acc()
                .set_attr("fo:border-top", border_string(width, border, color));
        }
    };
}

macro_rules! fo_padding {
    ($acc:ident) => {
        /// Padding for all sides. See §7.29.15 of [XSL].
        ///
        /// The fo:padding attribute is usable with the following elements:
        /// <style:graphicproperties> 17.21,
        /// <style:header-footer-properties> 17.5,
        /// <style:pagelayout-properties> 17.2,
        /// <style:paragraph-properties> 17.6 and
        /// <style:tablecell-properties> 17.18.
        pub fn set_padding(&mut self, padding: Length) {
            self.$acc().set_attr("fo:padding", padding.to_string());
        }

        /// Padding. See §7.7.36 of [XSL].
        ///
        /// The fo:padding-bottom attribute is usable with the following elements:
        /// <style:graphicproperties> 17.21,
        /// <style:header-footer-properties> 17.5,
        /// <style:pagelayout-properties> 17.2,
        /// <style:paragraph-properties> 17.6 and
        /// <style:tablecell-properties> 17.18.
        pub fn set_padding_bottom(&mut self, padding: Length) {
            self.$acc()
                .set_attr("fo:padding-bottom", padding.to_string());
        }

        /// Padding. See §7.7.37 of [XSL].
        ///
        /// The fo:padding-left attribute is usable with the following elements:
        /// <style:graphicproperties> 17.21,
        /// <style:header-footer-properties> 17.5,
        /// <style:pagelayout-properties> 17.2,
        /// <style:paragraph-properties> 17.6 and
        /// <style:tablecell-properties> 17.18.
        pub fn set_padding_left(&mut self, padding: Length) {
            self.$acc().set_attr("fo:padding-left", padding.to_string());
        }

        /// Padding. See §7.7.38 of [XSL].
        ///
        /// The fo:padding-right attribute is usable with the following elements:
        /// <style:graphicproperties> 17.21,
        /// <style:header-footer-properties> 17.5,
        /// <style:pagelayout-properties> 17.2,
        /// <style:paragraph-properties> 17.6 and
        /// <style:tablecell-properties> 17.18.
        pub fn set_padding_right(&mut self, padding: Length) {
            self.$acc()
                .set_attr("fo:padding-right", padding.to_string());
        }

        /// Padding. See §7.7.35 of [XSL].
        ///
        /// The fo:padding-top attribute is usable with the following elements:
        /// <style:graphicproperties> 17.21,
        /// <style:header-footer-properties> 17.5,
        /// <style:pagelayout-properties> 17.2,
        /// <style:paragraph-properties> 17.6 and
        /// <style:tablecell-properties> 17.18.
        pub fn set_padding_top(&mut self, padding: Length) {
            self.$acc().set_attr("fo:padding-top", padding.to_string());
        }
    };
}

macro_rules! fo_wrap_option {
    ($acc:ident) => {
        // fo:wrap-option 20.230,
        /// See §7.15.13 of [XSL].
        /// If wrapping is disabled, it is implementation-defined whether the overflow text is visible or hidden.
        /// If the text is hidden consumers may support a scrolling to access the text.
        pub fn set_wrap_option(&mut self, wrap: WrapOption) {
            self.$acc().set_attr("fo:wrap-option", wrap.to_string());
        }
    };
}

macro_rules! fo_border_line_width {
    ($acc:ident) => {
        /// The style:border-line-width attribute specifies the widths of borders defined by the FO
        /// border properties (see 20.183) for borders where the value of these properties is double.
        /// The value of the style:border-line-width attribute is a list of three white space-separated
        /// lengths, as follows:
        /// * The first value specifies the width of the inner line
        /// * The second value specifies the distance between the two lines
        /// * The third value specifies the width of the outer line
        ///
        /// The style:border-line-width attribute is usable with the following elements:
        /// <style:graphic-properties> 17.21,
        /// <style:header-footer-properties> 17.5,
        /// <style:page-layout-properties> 17.2,
        /// <style:paragraph-properties> 17.6 and
        /// <style:table-cell-properties> 17.18.
        pub fn set_border_line_width(&mut self, inner: Length, spacing: Length, outer: Length) {
            self.$acc().set_attr(
                "style:border-line-width",
                border_line_width_string(inner, spacing, outer),
            );
        }

        /// The style:border-line-width-bottom attribute specifies the widths of the bottom border
        /// for borders defined by the FO border properties (see 20.183) if the property for the bottom border
        /// has the value double.
        /// The value of the style:border-line-width-bottom attribute is a list of three white spaceseparated lengths, as follows:
        /// * The first value specifies the width of the inner line
        /// * The second value specifies the distance between the two lines
        /// * The third value specifies the width of the outer line
        ///
        /// The style:border-line-width attribute is usable with the following elements:
        /// <style:graphic-properties> 17.21,
        /// <style:header-footer-properties> 17.5,
        /// <style:page-layout-properties> 17.2,
        /// <style:paragraph-properties> 17.6 and
        /// <style:table-cell-properties> 17.18.
        pub fn set_border_line_width_bottom(
            &mut self,
            inner: Length,
            spacing: Length,
            outer: Length,
        ) {
            self.$acc().set_attr(
                "style:border-line-width-bottom",
                border_line_width_string(inner, spacing, outer),
            );
        }

        /// The style:border-line-width-left attribute specifies the widths of the left border for
        /// borders defined by the FO border properties (see 20.183) if the property for the left border has the
        /// value double.
        /// The value of the style:border-line-width-left attribute is a list of three white spaceseparated lengths, as follows:
        /// * The first value specifies the width of the inner line
        /// * The second value specified the distance between the two lines
        /// * The third value specifies the width of the outer line
        ///
        /// The style:border-line-width attribute is usable with the following elements:
        /// <style:graphic-properties> 17.21,
        /// <style:header-footer-properties> 17.5,
        /// <style:page-layout-properties> 17.2,
        /// <style:paragraph-properties> 17.6 and
        /// <style:table-cell-properties> 17.18.
        pub fn set_border_line_width_left(
            &mut self,
            inner: Length,
            spacing: Length,
            outer: Length,
        ) {
            self.$acc().set_attr(
                "style:border-line-width-left",
                border_line_width_string(inner, spacing, outer),
            );
        }

        /// The style:border-line-width-right attribute specifies the widths of the right border for
        /// borders defined by the FO border properties (see 20.183) if the property for the right border has
        /// the value double.
        /// The value of the style:border-line-width-right attribute is a list of three white spaceseparated lengths, as follows:
        /// * The first value specifies the width of the inner line
        /// * The second value specified the distance between the two lines
        /// * The third value specifies the width of the outer line
        ///
        /// The style:border-line-width attribute is usable with the following elements:
        /// <style:graphic-properties> 17.21,
        /// <style:header-footer-properties> 17.5,
        /// <style:page-layout-properties> 17.2,
        /// <style:paragraph-properties> 17.6 and
        /// <style:table-cell-properties> 17.18.
        pub fn set_border_line_width_right(
            &mut self,
            inner: Length,
            spacing: Length,
            outer: Length,
        ) {
            self.$acc().set_attr(
                "style:border-line-width-right",
                border_line_width_string(inner, spacing, outer),
            );
        }

        /// The style:border-line-width-top attribute specifies the widths of the top border for
        /// borders defined by the FO border properties (see 20.183) if the property for the top border has the
        /// value double.
        /// The value of the style:border-line-width-top attribute is a list of three white spaceseparated lengths, as follows:
        /// * The first value specifies the width of the inner line
        /// * The second value specified the distance between the two lines
        /// * The third value specifies the width of the outer line
        ///
        /// The style:border-line-width attribute is usable with the following elements:
        /// <style:graphic-properties> 17.21,
        /// <style:header-footer-properties> 17.5,
        /// <style:page-layout-properties> 17.2,
        /// <style:paragraph-properties> 17.6 and
        /// <style:table-cell-properties> 17.18.
        pub fn set_border_line_width_top(&mut self, inner: Length, spacing: Length, outer: Length) {
            self.$acc().set_attr(
                "style:border-line-width-top",
                border_line_width_string(inner, spacing, outer),
            );
        }
    };
}

macro_rules! style_cell_protect {
    ($acc:ident) => {
        /// The style:cell-protect attribute specifies how a cell is protected.
        ///
        /// This attribute is only evaluated if the current table is protected.
        ///
        /// The defined values for the style:cell-protect attribute are:
        /// * formula-hidden: if cell content is a formula, it is not displayed. It can be replaced by
        /// changing the cell content.
        /// Note: Replacement of cell content includes replacement with another formula or
        /// other cell content.
        /// * hidden-and-protected: cell content is not displayed and cannot be edited. If content is a
        /// formula, the formula result is not displayed.
        /// * none: formula responsible for cell content is neither hidden nor protected.
        /// * protected: cell content cannot be edited.
        /// * protected formula-hidden: cell content cannot be edited. If content is a formula, it is not
        /// displayed. A formula result is displayed.
        pub fn set_cell_protect(&mut self, protect: CellProtect) {
            self.$acc()
                .set_attr("style:cell-protect", protect.to_string());
        }
    };
}

macro_rules! style_decimal_places {
    ($acc:ident) => {
        // style:decimal-places 20.258,
        /// The style:decimal-places attribute specifies the maximum number of decimal places that
        /// are displayed if numbers are formatted by a data style that has no setting for number of decimal
        /// places itself.
        /// This attribute is only evaluated if it is contained in a default style.
        pub fn set_decimal_places(&mut self, dec: u8) {
            self.$acc()
                .set_attr("style:decimal-places", dec.to_string());
        }
    };
}

macro_rules! style_diagonal {
    ($acc:ident) => {
        // style:diagonal-bl-tr 20.259,
        /// The style:diagonal-bl-tr attribute specifies the style of border to use for a bottom-left to
        /// top-right diagonal in a spreadsheet cell.
        pub fn set_diagonal_bl_tr(&mut self, width: Length, border: Border, color: Rgb<u8>) {
            self.$acc()
                .set_attr("style:diagonal-bl-tr", border_string(width, border, color));
        }

        // style:diagonal-bl-tr-widths 20.260,
        /// The style:diagonal-bl-tr-widths attribute specifies the width between a double line
        /// border to use for a bottom-left to top-right diagonal in a spreadsheet cell.
        pub fn set_diagonal_bl_tr_widths(&mut self, inner: Length, spacing: Length, outer: Length) {
            self.$acc().set_attr(
                "style:diagonal-bl-tr-widths",
                border_line_width_string(inner, spacing, outer),
            );
        }

        // style:diagonal-tl-br 20.261,
        /// The style:diagonal-tl-br attribute specifies the style of border to use for a left-top to
        /// bottom-right diagonal in a spreadsheet cell.
        pub fn set_diagonal_tl_br(&mut self, width: Length, border: Border, color: Rgb<u8>) {
            self.$acc()
                .set_attr("style:diagonal-tl-br", border_string(width, border, color));
        }

        // style:diagonal-tl-br-widths 20.262,
        /// The style:diagonal-tl-br-widths attribute specifies the width between a double line
        /// border to use for a top-left to bottom-right diagonal in a spreadsheet cell.
        pub fn set_diagonal_tl_br_widths(&mut self, inner: Length, spacing: Length, outer: Length) {
            self.$acc().set_attr(
                "style:diagonal-tl-br-widths",
                border_line_width_string(inner, spacing, outer),
            );
        }
    };
}

macro_rules! style_direction {
    ($acc:ident) => {
        /// The style:direction attribute specifies the direction of characters.
        /// The style:direction attribute modifies the direction of text rendering as specified by a
        /// style:writing-mode attribute. 20.404
        ///
        /// The defined values for the style:direction attribute are:
        /// * ltr – left to right, text is rendered in the direction specified by the style:writing-mode
        /// attribute
        /// * ttb – top to bottom, characters are vertically stacked but not rotated
        pub fn set_direction(&mut self, direction: WritingDirection) {
            self.$acc()
                .set_attr("style:direction", direction.to_string());
        }
    };
}

macro_rules! style_glyph_orientation_vertical {
    ($acc:ident) => {
        /// The style:glyph-orientation-vertical attribute specifies a vertical glyph orientation.
        /// See §10.7.3 of [SVG]. The attribute specifies an angle or automatic mode. The only defined angle
        /// is 0 degrees, which disables this feature.
        ///
        /// Note: OpenDocument v1.1 did not support angle specifications that contain an
        /// angle unit identifier. Angle unit identifiers should be omitted for compatibility with
        /// OpenDocument v1.1.
        pub fn set_glyph_orientation_vertical(&mut self, glyph_orientation: GlyphOrientation) {
            self.$acc().set_attr(
                "style:glyph-orientation-vertical",
                glyph_orientation.to_string(),
            );
        }
    };
}

macro_rules! style_print_content {
    ($acc:ident) => {
        // style:print-content 20.331,
        /// The style:print-content attribute specifies if content is printed.
        /// The style:print-content attribute specifies if cell content is printed.
        /// The style:print-content attribute is usable with the following element:
        /// * <style:tablecell-properties> 17.18.
        pub fn set_print_content(&mut self, print: bool) {
            self.$acc()
                .set_attr("style:print-content", print.to_string());
        }
    };
}

macro_rules! style_repeat_content {
    ($acc:ident) => {
        // style:repeat-content 20.342,
        /// The style:repeat-content attribute specifies whether text content of a cell is displayed as
        /// many times as there is space left in the cell's writing direction. The attribute has no effect for cell
        /// content that contains a line break.
        /// The defined values for the style:repeat-content attribute are:
        /// * false: text content of a cell should not be displayed as many times as there is space left in
        /// the cell's writing direction.
        /// * true: text content of a cell should be displayed as many times as there is space left in the
        /// cell's writing direction.
        pub fn set_repeat_content(&mut self, print: bool) {
            self.$acc()
                .set_attr("style:repeat-content", print.to_string());
        }
    };
}

macro_rules! style_rotation_align {
    ($acc:ident) => {
        // style:rotationalign 20.346,
        /// The style:rotation-align attribute specifies how the edge of the text in a cell is aligned
        /// after a rotation.
        /// The values of the style:rotation-align attribute are none, bottom, top or center.
        pub fn set_rotation_align(&mut self, align: RotationAlign) {
            self.$acc()
                .set_attr("style:rotation-align", align.to_string());
        }
    };
}

macro_rules! style_rotation_scale {
    ($acc:ident) => {
        // style:text-scale 20.387,
        /// The style:text-rotation-scale attribute specifies whether for rotated text the width of the
        /// text should be scaled to fit into the current line height or the width of the text should remain fixed,
        /// therefore changing the current line height.
        /// The defined values for the style:text-rotation-scale attribute are:
        /// * fixed: width of text should remain fixed.
        /// * line-height: width of text should be scaled to fit the current line height.
        pub fn set_rotation_scale(&mut self, scale: RotationScale) {
            self.$acc()
                .set_attr("style:text-rotation-scale", scale.to_string());
        }
    };
}

macro_rules! style_rotation_angle {
    ($acc:ident) => {
        // style:rotation-angle 20.347,
        /// The style:rotation-angle attribute specifies the rotation angle of content.
        pub fn set_rotation_angle(&mut self, angle: Angle) {
            self.$acc()
                .set_attr("style:rotation-angle", angle.to_string());
        }
    };
}

macro_rules! style_shadow {
    ($acc:ident) => {
        /// The style:shadow attribute specifies a shadow effect.
        /// The defined values for this attribute are those defined in §7.16.5 of [XSL], except the value
        /// inherit.
        ///
        /// The shadow effect is not applied to the text content of an element, but depending on the element
        /// where the attribute appears, to a paragraph, a text box, a page body, a header, a footer, a table or
        /// a table cell.
        ///
        /// The style:shadow attribute is usable with the following elements:
        /// <style:graphicproperties> 17.21,
        /// <style:header-footer-properties> 17.5,
        /// <style:pagelayout-properties> 17.2,
        /// <style:paragraph-properties> 17.6,
        /// <style:tablecell-properties> 17.18 and
        /// <style:table-properties> 17.15.
        pub fn set_shadow(
            &mut self,
            x_offset: Length,
            y_offset: Length,
            blur: Option<Length>,
            color: Rgb<u8>,
        ) {
            self.$acc().set_attr(
                "style:shadow",
                shadow_string(x_offset, y_offset, blur, color),
            );
        }
    };
}

macro_rules! style_shrink_to_fit {
    ($acc:ident) => {
        // style:shrinkto-fit 20.360,
        /// The style:shrink-to-fit attribute specifies whether content is reduced in size to fit within a
        /// cell or drawing object. Shrinking means that the font size of the content is decreased to fit the
        /// content into a cell or drawing object. The attribute has no effect on cells where the cell content
        /// already fits into the cell.
        ///
        /// The defined values for the style:shrink-to-fit attribute are:
        /// * false: content should not be reduced in size to fit within a cell or drawing object.
        /// * true: content should be reduced in size to fit within a cell or drawing object
        pub fn set_shrink_to_fit(&mut self, shrink: bool) {
            self.$acc()
                .set_attr("style:shrink-to-fit", shrink.to_string());
        }
    };
}

macro_rules! style_text_align_source {
    ($acc:ident) => {
        // style:text-align-source 20.364,
        /// The style:text-align-source attribute specifies the source of a text-align attribute.
        /// The defined values for the style:text-align-source attribute are:
        /// * fix: content alignment uses the value of the fo:text-align 20.223 attribute.
        /// * value-type: content alignment uses the value-type of the cell.
        ///
        /// The default alignment for a cell value-type string is left, for other value-types it is right.
        pub fn set_text_align_source(&mut self, align: TextAlignSource) {
            self.cellstyle
                .set_attr("style:text-align-source", align.to_string());
        }
    };
}

macro_rules! style_vertical_align {
    ($acc:ident) => {
        // style:vertical-align 20.396,
        /// The style:vertical-align attribute specifies the vertical position of a character. By default
        /// characters are aligned according to their baseline.
        ///
        /// The defined values for the style:vertical-align attribute are:
        /// * auto: automatically, which sets the vertical alignment to suit the text rotation. Text that is
        /// rotated 0 or 90 degrees is aligned to the baseline, while text that is rotated 270 degrees is
        /// aligned to the center of the line.
        /// * baseline: to the baseline of the character.
        /// * bottom: to the bottom of the line.
        /// * middle: to the center of the line.
        /// * top: to the top of the line.
        pub fn set_vertical_align(&mut self, align: CellAlignVertical) {
            self.cellstyle
                .set_attr("style:vertical-align", align.to_string());
        }
    };
}

macro_rules! fo_break {
    ($acc:ident) => {
        /// See §7.19.2 of [XSL]. The values odd-page and even-page are not supported.
        /// This attribute shall not be used at the same time as fo:break-after.
        /// In the OpenDocument XSL-compatible namespace, the fo:break-before attribute does not
        /// support even-page, inherit and odd-page values.
        pub fn set_break_before(&mut self, pagebreak: PageBreak) {
            self.$acc()
                .set_attr("fo:break-before", pagebreak.to_string());
        }

        /// See §7.19.1 of [XSL]. The values odd-page and even-page are not supported.
        /// This attribute shall not be used at the same time as fo:break-before.
        /// In the OpenDocument XSL-compatible namespace, the fo:break-after attribute does not
        /// support even-page, inherit and odd-page values.
        pub fn set_break_after(&mut self, pagebreak: PageBreak) {
            self.$acc()
                .set_attr("fo:break-after", pagebreak.to_string());
        }
    };
}

macro_rules! fo_hyphenation {
    ($acc:ident) => {
        /// See §7.15.1 of [XSL].
        pub fn set_hyphenation_keep(&mut self, hyphenation: Hyphenation) {
            self.$acc()
                .set_attr("fo:hyphenation-keep", hyphenation.to_string());
        }

        /// See §7.15.2 of [XSL].
        /// The defined values for the fo:hyphenation-ladder-count attribute are:
        /// • no-limit:
        /// • a value of type positiveInteger
        pub fn set_hyphenation_ladder_count(&mut self, hyphenation: HyphenationLadderCount) {
            self.$acc()
                .set_attr("fo:hyphenation-ladder-count", hyphenation.to_string());
        }
    };
}

macro_rules! fo_keep_together {
    ($acc:ident) => {
        /// See §7.19.3 of [XSL].
        /// In the OpenDocument XSL-compatible namespace, the fo:keep-together attribute does not
        /// support the integer value.
        ///
        /// The fo:keep-together attribute is usable with the following elements:
        /// <style:paragraphproperties> 17.6 and
        /// <style:table-row-properties> 17.17.
        pub fn set_keep_together(&mut self, keep_together: TextKeep) {
            self.$acc()
                .set_attr("fo:keep-together", keep_together.to_string());
        }
    };
}

macro_rules! fo_keep_with_next {
    ($acc:ident) => {
        /// See §7.19.4 of [XSL].
        /// In the OpenDocument XSL-compatible namespace, the fo:keep-with-next attribute does not
        /// support the integer value.
        pub fn set_keep_with_next(&mut self, keep_with_next: TextKeep) {
            self.$acc()
                .set_attr("fo:keep-with-next", keep_with_next.to_string());
        }
    };
}

macro_rules! fo_line_height {
    ($acc:ident) => {
        /// See §7.15.4 of [XSL].
        /// The value normal activates the default line height calculation. The value of this attribute can be a
        /// length, a percentage, normal.
        ///
        /// In the OpenDocument XSL-compatible namespace, the fo:line-height attribute does not
        /// support the inherit, number, and space values.
        /// The defined values for the fo:line-height attribute are:
        /// * a value of type nonNegativeLength 18.3.20
        /// * normal: disables the effects of style:line-height-at-least 20.317 and
        /// style:line-spacing 20.318.
        /// * a value of type percent 18.3.23
        ///
        /// The fo:line-height attribute is usable with the following element:
        /// <style:paragraphproperties> 17.6.
        pub fn set_line_height(&mut self, line_height: LineHeight) {
            self.$acc()
                .set_attr("fo:line-height", line_height.to_string());
        }
    };
}

macro_rules! fo_margin {
    ($acc:ident) => {
        /// See §7.29.14 of [XSL].
        /// In the OpenDocument XSL-compatible namespace, the fo:margin attribute does not support
        /// auto and inherit values.
        ///
        /// The fo:margin attribute is usable with the following elements:
        /// <style:graphicproperties> 17.21,
        /// <style:header-footer-properties> 17.5,
        /// <style:pagelayout-properties> 17.2,
        /// <style:paragraph-properties> 17.6 and
        /// <style:tableproperties> 17.15.
        pub fn set_margin(&mut self, margin: Margin) {
            self.$acc().set_attr("fo:margin", margin.to_string());
        }

        /// See §7.10.2 of [XSL].
        /// If this attribute is contained in a <style:paragraph-properties> 17.6 element, its value may
        /// be a percentage that refers to the corresponding margin of a parent style.
        /// In the OpenDocument XSL-compatible namespace, the fo:margin-bottom attribute does not
        /// support the auto and inherit values.
        ///
        /// The fo:margin-bottom attribute is usable with the following elements:
        /// <style:graphicproperties> 17.21,
        /// <style:header-footer-properties> 17.5,
        /// <style:pagelayout-properties> 17.2,
        /// <style:paragraph-properties> 17.6 and
        /// <style:tableproperties> 17.15.
        pub fn set_margin_bottom(&mut self, margin: Margin) {
            self.$acc().set_attr("fo:margin-bottom", margin.to_string());
        }

        /// See §7.10.3 of [XSL].
        /// If this attribute is contained in a <style:paragraph-properties> 17.6 element, its value may
        /// be a percentage that refers to the corresponding margin of a parent style.
        /// Tables that align to the left or to the center ignore right margins, and tables that align to the right
        /// or to the center ignore left margins.
        ///
        /// The fo:margin-left attribute is usable with the following elements:
        /// <style:graphicproperties> 17.21,
        /// <style:header-footer-properties> 17.5,
        /// <style:pagelayout-properties> 17.2,
        /// <style:paragraph-properties> 17.6,
        /// <style:sectionproperties> 17.11 and
        /// <style:table-properties> 17.15.
        pub fn set_margin_left(&mut self, margin: Margin) {
            self.$acc().set_attr("fo:margin-left", margin.to_string());
        }

        /// See §7.10.4 of [XSL].
        /// If this attribute is contained in a <style:paragraph-properties> 17.6 element, its value may
        /// be a percentage that refers to the corresponding margin of a parent style.
        /// Tables that align to the left or to the center ignore right margins, and tables that align to the right
        /// or to the center ignore left margins.
        ///
        /// The fo:margin-right attribute is usable with the following elements:
        /// <style:graphicproperties> 17.21,
        /// <style:header-footer-properties> 17.5,
        /// <style:pagelayout-properties> 17.2,
        /// <style:paragraph-properties> 17.6,
        /// <style:sectionproperties> 17.11 and
        /// <style:table-properties> 17.15.
        pub fn set_margin_right(&mut self, margin: Margin) {
            self.$acc().set_attr("fo:margin-right", margin.to_string());
        }

        /// See §7.10.1 of [XSL].
        /// If this attribute is contained in a <style:paragraph-properties> 17.6 element, its value may
        /// be a percentage that refers to the corresponding margin of a parent style.
        /// In the OpenDocument XSL-compatible namespace, the fo:margin-top attribute does not
        /// support the inherit value.
        ///
        /// The fo:margin-top attribute is usable with the following elements:
        /// <style:graphicproperties> 17.21,
        /// <style:header-footer-properties> 17.5,
        /// <style:pagelayout-properties> 17.2,
        /// <style:paragraph-properties> 17.6 and
        /// <style:tableproperties> 17.15.
        pub fn set_margin_top(&mut self, margin: Margin) {
            self.$acc().set_attr("fo:margin-top", margin.to_string());
        }
    };
}

macro_rules! fo_orphans {
    ($acc:ident) => {
        /// See §7.19.6 of [XSL].
        ///
        /// The fo:orphans attribute is usable with the following element:
        /// <style:paragraphproperties> 17.6.
        pub fn set_orphans(&mut self, orphans: u32) {
            self.$acc().set_attr("fo:orphans", orphans.to_string());
        }
    };
}

macro_rules! fo_text_align {
    ($acc:ident) => {
        /// See §7.15.9 of [XSL].
        /// If there are no values specified for the fo:text-align and style:justify-single-word
        /// 20.301 attributes within the same formatting properties element, the values of those attributes is
        /// set to start and false respectively.
        ///
        /// In the OpenDocument XSL-compatible namespace, the fo:text-align attribute does not
        /// support the inherit, inside, outside, or string values.
        ///
        /// The fo:text-align attribute is usable with the following elements:
        /// <style:list-levelproperties> 17.19 and
        /// <style:paragraph-properties> 17.6.
        pub fn set_text_align(&mut self, align: TextAlign) {
            self.$acc().set_attr("fo:text-align", align.to_string());
        }
    };
}

macro_rules! fo_text_align_last {
    ($acc:ident) => {
        /// See §7.15.10 of [XSL].
        /// This attribute is ignored if it not accompanied by an fo:text-align 20.223 attribute.
        /// If no value is specified for this attribute, the value is set to start.
        ///
        /// The fo:text-align-last attribute is usable with the following element:
        /// <style:paragraph-properties> 17.6.
        pub fn set_text_align_last(&mut self, align: TextAlignLast) {
            self.$acc()
                .set_attr("fo:text-align-last", align.to_string());
        }
    };
}

macro_rules! fo_text_indent {
    ($acc:ident) => {
        /// The fo:text-indent attribute specifies a positive or negative indent for the first line of a
        /// paragraph. See §7.15.11 of [XSL]. The attribute value is a length. If the attribute is contained in a
        /// common style, the attribute value may be also a percentage that refers to the corresponding text
        /// indent of a parent style.
        ///
        /// The fo:text-indent attribute is usable with the following element:
        /// <style:paragraphproperties> 17.6.
        ///
        /// The values of the fo:text-indent attribute are a value of type length 18.3.18 or a value of
        /// type percent 18.3.23.
        pub fn set_text_indent(&mut self, indent: Indent) {
            self.$acc().set_attr("fo:text-indent", indent.to_string());
        }
    };
}

macro_rules! fo_widows {
    ($acc:ident) => {
        /// See §7.19.7 of [XSL].
        /// The fo:widows attribute specifies the minimum number of lines that shall be displayed at the top
        /// of a page to avoid paragraph widows.
        /// In the OpenDocument XSL-compatible namespace, the fo:widows attribute does not support
        /// the inherit value.
        ///
        /// The fo:widows attribute is usable with the following element:
        /// <style:paragraphproperties> 17.6.
        ///
        /// The fo:widows attribute has the data type nonNegativeInteger 18.2
        pub fn set_widows(&mut self, num: u32) {
            self.$acc().set_attr("fo:widows", num.to_string());
        }
    };
}

macro_rules! style_autotext_indent {
    ($acc:ident) => {
        /// The style:auto-text-indent attribute specifies that the first line of a paragraph is indented
        /// by a value that is based on the current font size.
        /// If this attribute has a value of true and is used together with a fo:text-indent 20.225
        /// attribute the fo:text-indent attribute is ignored.
        ///
        /// The style:auto-text-indent attribute is usable with the following element:
        /// <style:paragraph-properties> 17.6.
        ///
        /// The style:auto-text-indent attribute has the data type boolean 18.3.3.
        pub fn set_auto_text_indent(&mut self, indent: bool) {
            self.$acc()
                .set_attr("style:auto-text-indent", indent.to_string());
        }
    };
}

macro_rules! style_background_transparency {
    ($acc:ident) => {
        /// The style:background-transparency attribute specifies the transparency of a paragraph's
        /// background color.
        /// The style:background-transparency attribute is usable with the following elements:
        /// <style:graphic-properties> 17.21 and <style:paragraph-properties> 17.6.
        pub fn set_background_transpareny(&mut self, percent: Percent) {
            self.$acc()
                .set_attr("style:background-transparency", percent.to_string());
        }
    };
}

macro_rules! style_contextual_spacing {
    ($acc:ident) => {
        /// The fo:margin-bottom 20.206 attribute of a paragraph and the fo:margin-top 20.209
        /// attribute of the next paragraph are ignored, so that the space between the paragraphs is zero,
        /// if all of the following conditions hold:
        /// * The style:contextual-spacing attribute of both paragraphs has the value true.
        /// * The paragraphs belong to the same content area.
        /// * The text:style-name 19.880 attribute of the paragraphs refer to the same common
        /// paragraph style. In case a text:style-name attribute refers to an automatic style, the value
        /// of the style:parent-style-name 19.510 attribute of the automatic style is taken for the
        /// style comparison. If a paragraph has a conditional style, the value of its text:cond-stylename 19.781 attribute is taken for the style comparison.
        /// The default value for this attribute is false.
        ///
        /// The style:contextual-spacing attribute is usable with the following element:
        /// <style:paragraph-properties> 17.6.
        pub fn set_contextual_spacing(&mut self, spacing: bool) {
            self.$acc()
                .set_attr("style:contextual-spacing", spacing.to_string());
        }
    };
}

macro_rules! style_font_independent_line_spacing {
    ($acc:ident) => {
        /// The style:font-independent-line-spacing attribute specifies if font independent line
        /// spacing is used.
        ///
        /// The defined values for the style:font-independent-line-spacing attribute are:
        /// * false: font metric of the font is taken into account.
        /// * true: line height is calculated only from the font height as specified by the font size attributes
        /// fo:font-size 20.190, style:font-size-asian 20.284 and style:font-sizecomplex 20.285.
        ///
        /// The style:font-independent-line-spacing attribute is usable with the following
        /// element: <style:paragraph-properties> 17.6.
        pub fn set_font_independent_line_spacing(&mut self, spacing: bool) {
            self.$acc()
                .set_attr("style:font-independent-line-spacing", spacing.to_string());
        }
    };
}

macro_rules! style_join_border {
    ($acc:ident) => {
        /// The style:join-border property specifies whether a border for one paragraph is to be
        /// extended around the following paragraph.
        ///
        /// In addition to the value of this attribute, joining of borders requires meeting these conditions:
        /// 1) Values of attributes fo:border-top 20.183.6, fo:border-bottom 20.183.3,
        /// fo:border-left 20.183.4 and fo:border-right 20.183.5 are the same. These values
        /// can also be given by the fo:border 20.183.2 attribute.
        /// 2) Values of attributes style:border-line-width-top 20.252, style:border-linewidth-bottom 20.249, style:border-line-width-left 20.250 and
        /// style:border-line-width-right 20.251 are the same. These values can also be given
        /// by the style:border-line-width 20.248 attribute.
        /// 3) Values of attributes fo:padding-left 20.219 and fo:padding-right 20.220 are the
        /// same. These values can also be given by the fo:padding 20.217 attribute.
        /// 4) Values of the fo:margin-right 20.208 attributes are the same. These values can also be
        /// given by the fo:margin 20.205 attribute.
        /// 5) Values of the fo:margin-left 20.207 attribute, which can also be given by the fo:margin,
        /// and fo:text-indent 19.246 attributes, that meet one of these conditions:
        /// 1. All values are the same.
        /// 2. Values of the fo:margin-left attributes are the same and values of the fo:textindent attributes are non-negative.
        /// 3. Value of the fo:margin-left attribute of one paragraph whose value of the fo:textindent attribute is non-negative is the same as the sum of values of the fo:marginleft and fo:text-indent attributes of the other paragraph whose value of the
        /// fo:text-indent attribute is negative.
        /// 4. Both values of the fo:text-indent attributes are negative and the sums of values of the
        /// fo:margin-left and fo:text-indent attributes are equal.
        ///
        /// The default value of this attribute is true.
        ///
        /// The defined values for the style:join-border attribute are:
        /// * false: borders should not be joined.
        /// * true: borders should be joined.
        ///
        /// The style:join-border attribute is usable with the following element:
        /// <style:paragraphproperties> 17.6.
        pub fn set_join_border(&mut self, join: bool) {
            self.$acc().set_attr("style:join-border", join.to_string());
        }
    };
}

macro_rules! style_justify_single_word {
    ($acc:ident) => {
        /// The style:justify-single-word attribute specifies whether a single word should be justified
        /// when the last line in a paragraph is justified.
        /// Specifying a style:justify-single-word attribute without specifying a fo:text-align
        /// 20.223 and fo:text-align-last 20.224 attribute has no effect. Unspecified, both
        /// fo:textalign and fo:text-align-last have the value start.
        ///
        /// The defined values for the style:justify-single-word attribute are:
        /// * false: single word should not be justified when the last line in a paragraph is justified.
        /// * true: single word should be justified when last line in a paragraph is justified.
        ///
        /// The style:justify-single-word attribute is usable with the following element:
        /// <style:paragraph-properties> 17.6.
        pub fn set_justify_single_word(&mut self, justify: bool) {
            self.$acc()
                .set_attr("style:justify-single-word", justify.to_string());
        }
    };
}

macro_rules! style_line_break {
    ($acc:ident) => {
        /// The style:line-break attribute specifies line breaking rules.
        /// The defined values for the style:line-break attribute are:
        /// * normal: line breaks may occur between any characters.
        /// * strict: line breaks shall not occur before or after implementation-defined characters.
        ///
        /// The style:line-break attribute is usable with the following element: <style:paragraphproperties> 17.6.
        ///
        /// The values of the style:line-break attribute are normal or strict.
        pub fn set_line_break(&mut self, linebreak: LineBreak) {
            self.$acc()
                .set_attr("style:line-break", linebreak.to_string());
        }
    };
}

macro_rules! style_line_height_at_least {
    ($acc:ident) => {
        /// The style:line-height-at-least attribute specifies a minimum line height. The value of
        /// this attribute is a length.
        /// The effect of this attribute is disabled when fo:line-height 20.204 has the value of normal.
        ///
        /// The style:line-height-at-least attribute is usable with the following element:
        /// <style:paragraph-properties> 17.6.
        pub fn set_line_height_at_least(&mut self, height: Length) {
            self.$acc()
                .set_attr("style:line-height-at-least", height.to_string());
        }
    };
}

macro_rules! style_line_spacing {
    ($acc:ident) => {
        /// The style:line-spacing attribute specifies a fixed distance between two lines.
        /// The effect of this attribute is disabled when fo:line-height 20.204 has the value of normal.
        ///
        /// The style:line-spacing attribute is usable with the following element:
        /// <style:paragraph-properties> 17.6.
        pub fn set_line_spacing(&mut self, spacing: Length) {
            self.$acc()
                .set_attr("style:line-spacing", spacing.to_string());
        }
    };
}

macro_rules! style_page_number {
    ($acc:ident) => {
        /// The style:page-number attribute specifies the page number that should be used for a new
        /// page when either a paragraph or table style specifies a master page that should be applied
        /// beginning from the start of a paragraph or table.
        ///
        /// The defined values for the style:page-number attribute are:
        /// * auto: a page has the page number of the previous page, incremented by one.
        /// * a value of type nonNegativeInteger 18.2: specifies a page number.
        ///
        /// The style:page-number attribute is usable with the following elements:
        /// <style:paragraph-properties> 17.6 and
        /// <style:table-properties> 17.15.
        ///
        /// The values of the style:page-number attribute are a value of type nonNegativeInteger
        /// 18.2 or auto.
        pub fn set_page_number(&mut self, page_number: PageNumber) {
            self.$acc()
                .set_attr("style:page-number", page_number.to_string());
        }
    };
}

macro_rules! style_punctuation_wrap {
    ($acc:ident) => {
        /// The style:punctuation-wrap attribute specifies whether a punctuation mark, if one is
        /// present, can be hanging, that is, whether it can placed in the margin area at the end of a full line of
        /// text.
        ///
        /// The defined values for the style:punctuation-wrap attribute are:
        /// * hanging: a punctuation mark can be placed in the margin area at the end of a full line of text.
        /// * simple: a punctuation mark cannot be placed in the margin area at the end of a full line of
        /// text.
        ///
        /// The style:punctuation-wrap attribute is usable with the following element:
        /// <style:paragraph-properties> 17.6.
        pub fn set_punctuation_wrap(&mut self, wrap: PunctuationWrap) {
            self.$acc()
                .set_attr("style:punctuation-wrap", wrap.to_string());
        }
    };
}

macro_rules! style_register_true {
    ($acc:ident) => {
        /// The style:register-true attribute specifies whether the lines on both sides of a printed page
        /// align. The text baselines of text in page columns or text box columns also align.
        /// The defined values for the style:register-true attribute are:
        /// * false: lines on both sides of a printed text need not align.
        /// * true: lines on both sides of a printed text should align.
        ///
        /// The style:register-true attribute is usable with the following element:
        /// <style:paragraph-properties> 17.6.
        ///
        /// The style:register-true attribute has the data type boolean 18.3.3
        pub fn set_register_true(&mut self, register: bool) {
            self.$acc()
                .set_attr("style:register-true", register.to_string());
        }
    };
}

macro_rules! style_snap_to_layout_grid {
    ($acc:ident) => {
        /// The style:snap-to-layout-grid attribute specifies whether the layout of a paragraph
        /// should consider the layout grid settings of the page where it appears.
        ///
        /// The defined values for the style:snap-to-layout-grid attribute are:
        /// * false: layout of a paragraph should not consider the layout grid settings of the page where it
        /// appears.
        /// * true: layout of a paragraph should consider the layout grid settings of the page where it
        /// appears.
        ///
        /// The style:snap-to-layout-grid attribute is usable with the following element:
        /// <style:paragraph-properties> 17.6.
        pub fn set_snap_to_layout_grid(&mut self, snap: bool) {
            self.$acc()
                .set_attr("style:snap-to-layout-grid", snap.to_string());
        }
    };
}

macro_rules! style_tab_stop_distance {
    ($acc:ident) => {
        /// The style:tab-stop-distance attribute specifies the distance between default tab stops. A
        /// default tab stop is repeated automatically after the specified distance. Default tab stops are only
        /// evaluated if they are specified within a default style.
        ///
        /// The style:tab-stop-distance attribute is usable with the following element:
        /// <style:paragraph-properties> 17.6.
        pub fn set_tab_stop_distance(&mut self, tab: Length) {
            self.$acc()
                .set_attr("style:tab-stop-distance", tab.to_string());
        }
    };
}

macro_rules! style_text_autospace {
    ($acc:ident) => {
        /// The style:text-autospace attribute specifies whether to add space between portions of
        /// Asian, Western, and complex texts.
        ///
        /// The defined values for the style:text-autospace attribute are:
        /// * ideograph-alpha: space should be added between portions of Asian, Western and
        /// complex texts.
        /// * none: space should not be added between portions of Asian, Western and complex texts.
        ///
        /// The style:text-autospace attribute is usable with the following element:
        /// <style:paragraph-properties> 17.6.
        pub fn set_text_autospace(&mut self, space: TextAutoSpace) {
            self.$acc()
                .set_attr("style:text-autospace", space.to_string());
        }
    };
}

macro_rules! style_vertical_align_para {
    ($acc:ident) => {
        /// The style:vertical-align attribute specifies the vertical position of a character. By default
        /// characters are aligned according to their baseline.
        ///
        /// The defined values for the style:vertical-align attribute are:
        /// • auto: automatically, which sets the vertical alignment to suit the text rotation. Text that is
        /// rotated 0 or 90 degrees is aligned to the baseline, while text that is rotated 270 degrees is
        /// aligned to the center of the line.
        /// • baseline: to the baseline of the character.
        /// • bottom: to the bottom of the line.
        /// • middle: to the center of the line.
        /// • top: to the top of the line.
        ///
        /// The style:vertical-align attribute is usable with the following element:
        /// <style:paragraph-properties> 17.6.
        pub fn set_vertical_align_para(&mut self, align: ParaAlignVertical) {
            self.$acc()
                .set_attr("style:vertical-align", align.to_string());
        }
    };
}

macro_rules! style_writing_mode {
    ($acc:ident) => {
        /// See §7.27.7 of [XSL].
        ///
        /// The defined value for the style:writing-mode attribute is page: writing mode is inherited from
        /// the page that contains the element where this attribute appears.
        pub fn set_writing_mode(&mut self, writing_mode: WritingMode) {
            self.$acc()
                .set_attr("style:writing-mode", writing_mode.to_string());
        }
    };
}

macro_rules! style_writing_mode_automatic {
    ($acc:ident) => {
        /// The style:writing-mode-automatic attribute specifies whether a consumer may
        /// recalculate the writing mode of a paragraph based on its content whenever the content is edited.
        ///
        /// The writing-mode should be specified in a style:writing-mode attribute.
        ///
        /// If the fo:text-align with value start, text alignment can be adapted to the writing mode.
        /// The defined values for the style:writing-mode-automatic attribute are:
        /// • false: consumers should not recalculate writing mode of a paragraph whenever its content
        /// is edited.
        /// • true: consumers should recalculate writing mode of a paragraph whenever its content is
        /// edited.
        ///
        /// The style:writing-mode-automatic attribute is usable with the following element:
        /// <style:paragraph-properties> 17.6.
        pub fn set_writing_mode_automatic(&mut self, auto: bool) {
            self.$acc()
                .set_attr("style:writing-mode-automatic", auto.to_string());
        }
    };
}

macro_rules! style_line_number {
    ($acc:ident) => {
        /// The text:line-number attribute specifies a new start value for line numbering, if a
        /// text:number-lines 20.434 attribute, with the value true, appears on the same
        /// <style:paragraph-properties> 17.6 element. Otherwise, this attribute shall be ignored.
        ///
        /// The text:line-number attribute is usable with the following element: <style:paragraphproperties> 17.6.
        /// The text:line-number attribute has the data type nonNegativeInteger 18.2.
        pub fn set_line_number(&mut self, line: u32) {
            self.$acc().set_attr("text:line-number", line.to_string());
        }
    };
}

macro_rules! style_number_lines {
    ($acc:ident) => {
        /// The text:number-lines attribute specifies whether lines are numbered.
        /// The defined values for the text:number-lines attribute are:
        /// * false: lines should not be numbered.
        /// * true: lines should be numbered.
        ///
        /// The text:number-lines attribute is usable with the following element:
        /// <style:paragraphproperties> 17.6.
        pub fn set_number_lines(&mut self, lines: bool) {
            self.$acc().set_attr("text:number-lines", lines.to_string());
        }
    };
}

// ok fo:background-color 20.182,
// ok fo:color 20.187,

// ** Kind of a switch for the group of attributes used. I don't understand what
// ** this is for, ignored for now.
// ignore style:script-type 20.358,

// ok fo:country 20.188,
// ok fo:language 20.202,
// ok fo:script 20.222,
// obsolete style:font-charset 20.268,
// obsolete fo:font-family 20.189,
// obsolete style:font-family-generic 20.273,
// ok style:font-name 20.277,
// obsolete style:fontpitch 20.280,
// ok fo:fontsize 20.190,
// ok style:font-size-rel 20.286,
// ok fo:font-style 20.191,
// obsolete style:font-style-name 20.291,
// ok fo:font-weight 20.193,
// obsolete style:rfc-language-tag 20.343,
// ok fo:font-variant 20.192,

// ok style:country-asian 20.256,
// ok style:language-asian 20.302,
// ok style:script-asian 20.356,
// obsolete style:font-charset-asian 20.269,
// obsolete style:font-family-asian 20.271,
// obsolete style:font-family-generic-asian 20.274,
// ok style:font-name-asian 20.278,
// obsolete style:font-pitch-asian 20.281,
// ok style:font-size-asian 20.284,
// ok style:font-size-rel-asian 20.287,
// ok style:font-style-asian 20.289,
// obsolete style:fontstyle-name-asian 20.292,
// ok style:fontweight-asian 20.294,
// obsolete style:rfc-language-tag-asian 20.344,

// ok style:country-complex 20.257,
// ok style:language-complex 20.303,
// ok style:script-complex 20.357,
// obsolete style:font-charset-complex 20.270,
// obsolete style:font-family-complex 20.272,
// obsolete style:font-family-generic-complex 20.275,
// ok style:font-name-complex 20.279,
// obsolete style:font-pitch-complex 20.282,
// ok style:font-size-complex 20.285,
// ok style:font-size-rel-complex 20.288,
// ok style:font-style-complex 20.290,
// obsolete style:font-style-name-complex 20.293,
// ok style:font-weight-complex 20.295,
// obsolete style:rfclanguage-tag-complex 20.345,

// ok fo:hyphenate 20.195,
// ok fo:hyphenation-push-char-count 20.198,
// ok fo:hyphenation-remain-char-count 20.199,
// ok fo:letter-spacing 20.203,
// ok fo:text-shadow 20.226,
// ok fo:text-transform 20.227,
// ok style:font-relief 20.283,
// ok style:letter-kerning 20.316,
// never style:text-blinking 20.366,
// ok style:text-combine 20.367,
// ok style:text-combine-end-char 20.369,
// ok style:text-combine-start-char 20.368,
// ok style:text-emphasize 20.370,
// ok style:text-line-through-color 20.371,
// ok style:text-line-through-mode 20.372,
// ok style:text-line-through-style 20.373,
// ok style:text-line-through-text 20.374,
// ok style:text-line-through-text-style 20.375,
// ok style:text-line-through-type 20.376,
// ok style:text-line-through-width 20.377,
// ok style:text-outline 20.378,
// ok style:text-overline-color 20.379,
// ok style:text-overline-mode 20.380,
// ok style:text-overline-style 20.381,
// ok style:text-overline-type 20.382,
// ok style:text-overline-width 20.383,
// ok style:text-position 20.384,
// ok style:text-rotation-angle 20.385,
// ok style:text-rotation-scale 20.386,
// ok style:text-scale 20.387,
// ok style:text-underline-color 20.388,
// ok style:text-underline-mode 20.389,
// ok style:text-underline-style 20.390,
// ok style:text-underline-type 20.391,
// ok style:text-underline-width 20.392,
// ok style:use-window-font-color 20.395,
// ok text:condition 20.426,
// ok text:display 20.427.

macro_rules! text_locale {
    ($acc:ident) => {
        /// Sets the attributes for fo:language, fo:country and fo:script
        /// to the given locale.
        ///
        /// These attributes are evaluated for any [UNICODE] characters whose script type is latin.
        pub fn set_locale(&mut self, locale: Locale) {
            if locale != Locale::UND {
                self.attr
                    .set_attr("fo:language", locale.id.language.to_string());
                if let Some(region) = locale.id.region {
                    self.attr.set_attr("fo:country", region.to_string());
                } else {
                    self.attr.clear_attr("fo:country");
                }
                if let Some(script) = locale.id.script {
                    self.attr.set_attr("fo:script", script.to_string());
                } else {
                    self.attr.clear_attr("fo:script");
                }
            } else {
                self.attr.clear_attr("fo:language");
                self.attr.clear_attr("fo:country");
                self.attr.clear_attr("fo:script");
            }
        }
    };
}

macro_rules! text {
    ($acc:ident) => {
        /// See §7.17.1 of [XSL].
        /// In the OpenDocument XSL-compatible namespace, the fo:color attribute does not support the
        /// inherit value.
        pub fn set_color(&mut self, color: Rgb<u8>) {
            self.$acc().set_attr("fo:color", color_string(color));
        }

        // LATIN

        /// The style:font-name attribute specifies a font that is declared by a <style:font-face>
        /// 16.23 element with a style:name 19.502 attribute whose name is the same as that of the
        /// style:font-name attribute value.
        ///
        /// This attribute is evaluated for any [UNICODE] character whose script type is latin. 20.358
        pub fn set_font_name<S: Into<String>>(&mut self, name: S) {
            self.$acc().set_attr("style:font-name", name.into());
        }

        /// See §7.8.4 of [XSL].
        /// This attribute is evaluated for any [UNICODE] character whose script type is latin. 20.358
        /// The value of this attribute is either an absolute length or a percentage as described in §7.8.4 of
        /// [XSL]. In contrast to XSL, percentage values can be used within common styles only and are
        /// based on the font height of the parent style rather than to the font height of the attributes
        /// neighborhood. Absolute font heights and relative font heights are not supported.
        ///
        /// Note: The style:font-size-asian attribute (20.284) is evaluated for
        /// [UNICODE] characters whose type is asian. The style:font-sizecomplex attribute (20.285) is evaluated for [UNICODE] characters whose type is
        /// complex.
        ///
        pub fn set_font_size(&mut self, size: Length) {
            self.$acc().set_attr("fo:font-size", size.to_string());
        }

        /// Font size as a percentage. See set_font_size.
        pub fn set_font_size_percent(&mut self, size: f64) {
            self.$acc().set_attr("fo:font-size", percent_string(size));
        }

        /// The style:font-size-rel attribute specifies a relative font size change.
        /// This attribute is evaluated for any [UNICODE] character whose script type is latin. 20.358
        /// This attribute specifies a relative font size change as a length. It cannot be used within automatic
        /// styles. This attribute changes the font size based on the font size of the parent style.
        pub fn set_font_size_rel(&mut self, size: Length) {
            self.$acc().set_attr("fo:font-size-rel", size.to_string());
        }

        /// Font size as a percentage. See set_font_size_rel.
        pub fn set_font_size_rel_percent(&mut self, size: f64) {
            self.$acc()
                .set_attr("fo:font-size-rel", percent_string(size));
        }

        /// See §7.8.7 of [XSL].
        /// This attribute is evaluated for any [UNICODE] character whose script type is latin. 20.358
        pub fn set_font_style(&mut self, style: FontStyle) {
            self.$acc().set_attr("fo:font-style", style.to_string());
        }

        /// Set the font-style to italic.
        pub fn set_font_italic(&mut self) {
            self.$acc().set_attr("fo:font-style", "italic".to_string());
        }

        /// See §7.8.9 of [XSL].
        /// This attribute is evaluated for any [UNICODE] character whose script type is latin. 20.358
        pub fn set_font_weight(&mut self, weight: FontWeight) {
            self.$acc().set_attr("fo:font-weight", weight.to_string());
        }

        /// Sets the font-weight to bold. See set_font_weight.
        pub fn set_font_bold(&mut self) {
            self.$acc()
                .set_attr("fo:font-weight", FontWeight::Bold.to_string());
        }

        /// See §7.8.8 of [XSL].
        pub fn set_font_variant(&mut self, var: FontVariant) {
            self.$acc().set_attr("fo:font-variant", var.to_string());
        }

        /// Combined font attributes.
        pub fn set_font_attr(&mut self, size: Length, bold: bool, italic: bool) {
            self.set_font_size(size);
            if bold {
                self.set_font_bold();
            }
            if italic {
                self.set_font_italic();
            }
        }

        // ASIAN

        /// Sets the attributes for fo:language, fo:country and fo:script
        /// to the given locale.
        ///
        /// These attributes are evaluated for any [UNICODE] characters whose script type is asian.
        pub fn set_locale_asian(&mut self, locale: Locale) {
            if locale != Locale::UND {
                self.attr
                    .set_attr("style:language-asian", locale.id.language.to_string());
                if let Some(region) = locale.id.region {
                    self.attr
                        .set_attr("style:country-asian", region.to_string());
                } else {
                    self.attr.clear_attr("style:country-asian");
                }
                if let Some(script) = locale.id.script {
                    self.attr.set_attr("style:script-asian", script.to_string());
                } else {
                    self.attr.clear_attr("style:script-asian");
                }
            } else {
                self.attr.clear_attr("style:language-asian");
                self.attr.clear_attr("style:country-asian");
                self.attr.clear_attr("style:script-asian");
            }
        }

        /// The style:font-name attribute specifies a font that is declared by a <style:font-face>
        /// 16.23 element with a style:name 19.502 attribute whose name is the same as that of the
        /// style:font-name attribute value.
        ///
        /// This attribute is evaluated for any [UNICODE] character whose script type is asian. 20.358
        pub fn set_font_name_asian<S: Into<String>>(&mut self, name: S) {
            self.$acc().set_attr("style:font-name-asian", name.into());
        }

        /// See §7.8.4 of [XSL].
        /// This attribute is evaluated for any [UNICODE] character whose script type is asian. 20.358
        /// The value of this attribute is either an absolute length or a percentage as described in §7.8.4 of
        /// [XSL]. In contrast to XSL, percentage values can be used within common styles only and are
        /// based on the font height of the parent style rather than to the font height of the attributes
        /// neighborhood. Absolute font heights and relative font heights are not supported.
        ///
        /// Note: The style:font-size-asian attribute (20.284) is evaluated for
        /// [UNICODE] characters whose type is asian. The style:font-sizecomplex attribute (20.285) is evaluated for [UNICODE] characters whose type is
        /// complex.
        ///
        pub fn set_font_size_asian(&mut self, size: Length) {
            self.$acc()
                .set_attr("style:font-size-asian", size.to_string());
        }

        /// Font size as a percentage. See set_font_size.
        pub fn set_font_size_percent_asian(&mut self, size: f64) {
            self.$acc()
                .set_attr("style:font-size-asian", percent_string(size));
        }

        /// The style:font-size-rel attribute specifies a relative font size change.
        /// This attribute is evaluated for any [UNICODE] character whose script type is asian. 20.358
        /// This attribute specifies a relative font size change as a length. It cannot be used within automatic
        /// styles. This attribute changes the font size based on the font size of the parent style.
        pub fn set_font_size_rel_asian(&mut self, size: Length) {
            self.$acc()
                .set_attr("style:font-size-rel-asian", size.to_string());
        }

        /// Font size as a percentage. See set_font_size_rel.
        pub fn set_font_size_rel_percent_asian(&mut self, size: f64) {
            self.$acc()
                .set_attr("style:font-size-rel-asian", percent_string(size));
        }

        /// See §7.8.7 of [XSL].
        /// This attribute is evaluated for any [UNICODE] character whose script type is asian. 20.358
        pub fn set_font_style_asian(&mut self, style: FontStyle) {
            self.$acc()
                .set_attr("style:font-style-asian", style.to_string());
        }

        /// Set the font-style to italic.
        pub fn set_font_italic_asian(&mut self) {
            self.$acc()
                .set_attr("style:font-style-asian", "italic".to_string());
        }

        /// See §7.8.9 of [XSL].
        /// This attribute is evaluated for any [UNICODE] character whose script type is asian. 20.358
        pub fn set_font_weight_asian(&mut self, weight: FontWeight) {
            self.$acc()
                .set_attr("style:fontweight-asian", weight.to_string());
        }

        /// Sets the font-weight to bold. See set_font_weight.
        pub fn set_font_bold_asian(&mut self) {
            self.$acc()
                .set_attr("style:fontweight-asian", FontWeight::Bold.to_string());
        }

        /// Combined font attributes.
        pub fn set_font_attr_asian(&mut self, size: Length, bold: bool, italic: bool) {
            self.set_font_size_asian(size);
            if bold {
                self.set_font_bold_asian();
            }
            if italic {
                self.set_font_italic_asian();
            }
        }

        // COMPLEX

        /// Sets the attributes for fo:language, fo:country and fo:script
        /// to the given locale.
        ///
        /// These attributes are evaluated for any [UNICODE] characters whose script type is complex.
        pub fn set_locale_complex(&mut self, locale: Locale) {
            if locale != Locale::UND {
                self.attr
                    .set_attr("style:language-complex", locale.id.language.to_string());
                if let Some(region) = locale.id.region {
                    self.attr
                        .set_attr("style:country-complex", region.to_string());
                } else {
                    self.attr.clear_attr("style:country-complex");
                }
                if let Some(script) = locale.id.script {
                    self.attr
                        .set_attr("style:script-complex", script.to_string());
                } else {
                    self.attr.clear_attr("style:script-complex");
                }
            } else {
                self.attr.clear_attr("style:language-complex");
                self.attr.clear_attr("style:country-complex");
                self.attr.clear_attr("style:script-complex");
            }
        }

        /// The style:font-name attribute specifies a font that is declared by a <style:font-face>
        /// 16.23 element with a style:name 19.502 attribute whose name is the same as that of the
        /// style:font-name attribute value.
        ///
        /// This attribute is evaluated for any [UNICODE] character whose script type is complex. 20.358
        pub fn set_font_name_complex<S: Into<String>>(&mut self, name: S) {
            self.$acc().set_attr("style:font-name-complex", name.into());
        }

        /// See §7.8.4 of [XSL].
        /// This attribute is evaluated for any [UNICODE] character whose script type is complex. 20.358
        /// The value of this attribute is either an absolute length or a percentage as described in §7.8.4 of
        /// [XSL]. In contrast to XSL, percentage values can be used within common styles only and are
        /// based on the font height of the parent style rather than to the font height of the attributes
        /// neighborhood. Absolute font heights and relative font heights are not supported.
        ///
        /// Note: The style:font-size-asian attribute (20.284) is evaluated for
        /// [UNICODE] characters whose type is asian. The style:font-sizecomplex attribute (20.285) is evaluated for [UNICODE] characters whose type is
        /// complex.
        ///
        pub fn set_font_size_complex(&mut self, size: Length) {
            self.$acc()
                .set_attr("style:font-size-complex", size.to_string());
        }

        /// Font size as a percentage. See set_font_size_complex.
        pub fn set_font_size_percent_complex(&mut self, size: f64) {
            self.$acc()
                .set_attr("style:font-size-complex", percent_string(size));
        }

        /// The style:font-size-rel attribute specifies a relative font size change.
        /// This attribute is evaluated for any [UNICODE] character whose script type is complex. 20.358
        /// This attribute specifies a relative font size change as a length. It cannot be used within automatic
        /// styles. This attribute changes the font size based on the font size of the parent style.
        pub fn set_font_size_rel_complex(&mut self, size: Length) {
            self.$acc()
                .set_attr("style:font-size-rel-complex", size.to_string());
        }

        /// Font size as a percentage. See set_font_size_rel.
        pub fn set_font_size_rel_percent_complex(&mut self, size: f64) {
            self.$acc()
                .set_attr("style:font-size-rel-complex", percent_string(size));
        }

        /// See §7.8.7 of [XSL].
        /// This attribute is evaluated for any [UNICODE] character whose script type is complex. 20.358
        pub fn set_font_style_complex(&mut self, style: FontStyle) {
            self.$acc()
                .set_attr("style:font-style-complex", style.to_string());
        }

        /// Set the font-style to italic.
        /// This attribute is evaluated for any [UNICODE] character whose script type is complex. 20.358
        pub fn set_font_italic_complex(&mut self) {
            self.$acc()
                .set_attr("style:font-style-complex", "italic".to_string());
        }

        /// See §7.8.9 of [XSL].
        /// This attribute is evaluated for any [UNICODE] character whose script type is complex. 20.358
        pub fn set_font_weight_complex(&mut self, weight: FontWeight) {
            self.$acc()
                .set_attr("style:font-weight-complex", weight.to_string());
        }

        /// Sets the font-weight to bold. See set_font_weight.
        pub fn set_font_bold_complex(&mut self) {
            self.$acc()
                .set_attr("style:font-weight-complex", FontWeight::Bold.to_string());
        }

        /// Combined font attributes.
        pub fn set_font_attr_complex(&mut self, size: Length, bold: bool, italic: bool) {
            self.set_font_size_complex(size);
            if bold {
                self.set_font_bold_complex();
            }
            if italic {
                self.set_font_italic_complex();
            }
        }

        // Other stuff.

        /// See §7.9.4 of [XSL].
        pub fn set_hyphenate(&mut self, hyphenate: bool) {
            self.$acc().set_attr("fo:hyphenate", hyphenate.to_string());
        }

        /// See §7.10.6 of [XSL]
        pub fn hyphenation_push_char_count(&mut self, count: u32) {
            self.$acc()
                .set_attr("fo:hyphenation-push-char-count", count.to_string());
        }

        /// See §7.10.7 of [XSL]
        pub fn hyphenation_remain_char_count(&mut self, count: u32) {
            self.$acc()
                .set_attr("fo:hyphenation-remain-char-count", count.to_string());
        }

        /// See §7.16.2 of [XSL].
        /// Sets the letter spacing.
        pub fn set_letter_spacing(&mut self, spacing: Length) {
            self.$acc()
                .set_attr("fo:letter-spacing", spacing.to_string());
        }

        /// Sets the letter spacing to normal.
        pub fn set_letter_spacing_normal(&mut self) {
            self.$acc()
                .set_attr("fo:letter-spacing", "normal".to_string());
        }

        /// The fo:text-shadow attribute specifies the text shadow style to use.
        pub fn set_text_shadow(
            &mut self,
            x_offset: Length,
            y_offset: Length,
            blur: Option<Length>,
            color: Rgb<u8>,
        ) {
            self.$acc().set_attr(
                "fo:text-shadow",
                shadow_string(x_offset, y_offset, blur, color),
            );
        }

        /// See §7.16.6 of [XSL].
        /// If fo:text-transform and fo:font-variant 20.192 attributes are used simultaneously and
        /// have different values than normal and none, the result is undefined.
        /// Note: In consumers, the fo:text-transform and fo:font-variant
        /// attributes are mutually exclusive
        pub fn set_text_transform(&mut self, trans: TextTransform) {
            self.$acc().set_attr("fo:text-transform", trans.to_string());
        }

        /// The style:font-relief attribute specifies whether a font should be embossed, engraved, or
        /// neither.
        /// The defined values for the style:font-relief attribute are:
        /// * embossed: characters are embossed.
        /// * engraved: characters are engraved.
        /// * none: characters are neither embossed or engraved.
        pub fn set_font_relief(&mut self, relief: TextRelief) {
            self.$acc()
                .set_attr("style:font-relief", relief.to_string());
        }

        /// The style:text-position attribute specifies whether text is positioned above or below the
        /// baseline and to specify the relative font height that is used for this text.
        /// This attribute can have one or two values.
        /// The first value shall be present and specifies the vertical text position as a percentage of the
        /// current font height or it takes one of the values sub or super. Negative percentages or the sub
        /// value place the text below the baseline. Positive percentages or the super value place the text
        /// above the baseline. If sub or super is specified, the consumer chooses an appropriate text
        /// position.
        /// The second value may be present and specifies the font height as a percentage of the current
        /// font-height. If this value is not specified, an appropriate font height is used.
        pub fn set_text_position(&mut self, pos: TextPosition, scale: Option<Percent>) {
            self.$acc()
                .set_attr("style:text-position", text_position(pos, scale));
        }

        /// The style:letter-kerning attribute specifies whether kerning between characters is enabled
        /// or disabled.
        pub fn set_letter_kerning(&mut self, kerning: bool) {
            self.$acc()
                .set_attr("style:letter-kerning", kerning.to_string());
        }

        /// The style:text-combine attribute specifies whether to combine characters so that they are
        /// displayed within two lines.
        ///
        /// The defined values for the style:text-combine attribute are:
        /// * letters: Display text in Kumimoji. Up to five (5) characters are combined within two lines
        /// and are displayed with a reduced size in a single wide-cell character. Additional characters
        /// are displayed as normal text.
        /// * lines: Displays text in Warichu. All characters with the style:text-combine attribute that
        /// immediately follow each other are displayed within two lines of approximately the same length.
        /// A line break may occur between any two characters to meet this constraint.
        /// * none: characters should not be combined.
        pub fn set_text_combine(&mut self, pos: TextCombine) {
            self.$acc().set_attr("style:text-combine", pos.to_string());
        }

        /// The style:text-combine-start-char attribute specifies the start character that is displayed
        /// before a portion of text whose style:text-combine 20.367 attribute has a value of lines.
        pub fn set_text_combine_start_char(&mut self, c: char) {
            self.$acc()
                .set_attr("style:text-combine-start-char", c.to_string());
        }

        /// The style:text-combine-end-char attribute specifies the end character that is displayed
        /// after a portion of text whose style:text-combine 20.367 attribute has a value of lines.
        pub fn set_text_combine_end_char(&mut self, c: char) {
            self.$acc()
                .set_attr("style:text-combine-end-char", c.to_string());
        }

        /// The style:text-emphasize attribute specifies emphasis in a text composed of [UNICODE]
        /// characters whose script type is asian. 20.358
        /// The value of this attribute consists of two white space-separated values.
        /// The first value represents the style to use for emphasis.
        /// The second value represents the position of the emphasis and it can be above or below. If the
        /// first value is none, the second value can be omitted.
        /// The defined values for the style:text-emphasize attribute are:
        /// * accent: calligraphic accent strokes.
        /// * circle: hollow circles.
        /// * disc: filled circles.
        /// * dot: calligraphic dot.
        /// * none: no emphasis marks.
        pub fn set_text_emphasize(&mut self, emphasize: TextEmphasize) {
            self.$acc()
                .set_attr("style:text-emphasize", emphasize.to_string());
        }

        /// The style:text-line-through-color attribute specifies the color that is used for linethrough text.
        /// The defined values for the style:text-line-through-color attribute are:
        /// * font-color: current text color is used for underlining.
        /// * a value of type color 18.3.9
        pub fn set_font_line_through_color(&mut self, color: Rgb<u8>) {
            self.$acc()
                .set_attr("style:text-line-through-color", color_string(color));
        }

        /// The style:text-line-through-mode attribute specifies whether lining through is applied to
        /// words only or to portions of text.
        /// The defined values for the style:text-line-through-mode attribute are:
        /// * continuous: lining is applied to words and separating spaces.
        /// * skip-white-space: lining is not applied to spaces between words.
        pub fn set_font_line_through_mode(&mut self, lmode: LineMode) {
            self.$acc()
                .set_attr("style:text-line-through-mode", lmode.to_string());
        }

        /// The style:text-line-through-style attribute specifies a style for rendering a line-through
        /// text.
        /// The defined values for the style:text-line-through-style attribute are:
        /// * none: text has no line through it.
        /// * dash: text has a dashed line through it.
        /// * dot-dash: text has a line whose repeating pattern is a dot followed by a dash through it.
        /// * dot-dot-dash: text has a line whose repeating pattern is two dots followed by a dash
        /// through it.
        /// * dotted: text has a dotted line through it.
        /// * long-dash: text has a dashed line whose dashes are longer than the ones from the dashed
        /// line for value dash through it.
        /// * solid: text has a solid line through it.
        /// * wave: text has a wavy line through it.
        /// Note: The definitions of the values of the style:text-line-through-style attribute are
        /// based on the text decoration style 'text-line-through-style' from [CSS3Text], §9.2.
        pub fn set_font_line_through_style(&mut self, lstyle: LineStyle) {
            self.$acc()
                .set_attr("style:text-line-through-style", lstyle.to_string());
        }

        /// The style:text-line-through-text attribute specifies a text that is used for line-through.
        /// The attribute will be evaluated only if the value of style:text-line-through-style 20.373
        /// attribute is different than none.
        /// If the attribute value is not empty, the attribute value string is used for line-through instead of the
        /// line style that has been specified by the style:text-line-through-style attribute.
        /// Consumers that do not support line-through with text should ignore the attribute, and should use
        /// the line style specified by the style:text-line-through-style attribute.
        /// Consumers that support line-through with single characters only, should use the first character of
        /// the value for line-through, if the style:text-line-through-text attribute value has more
        /// than one character. Consumers that support line-through with specific characters only (like ”x” or
        /// ”/” (U+002F, SOLIDUS) should use one of these characters if the attribute specifies characters
        /// that are not supported.
        pub fn set_font_line_through_text<S: Into<String>>(&mut self, text: S) {
            self.$acc()
                .set_attr("style:text-line-through-text", text.into());
        }

        /// The style:text-line-through-text-style specifies a text style that is applied to
        /// text-linethrough characters. It is not applied to line-through lines. If the attribute
        /// appears in an automatic style, it may reference either an automatic text style or a
        /// common style. If the attribute appears in a common style, it may reference a common
        /// style only.
        pub fn set_font_line_through_text_style(&mut self, style_ref: TextStyleRef) {
            self.$acc()
                .set_attr("style:text-line-through-text-style", style_ref.to_string());
        }

        /// The style:text-line-through-type attribute specifies whether text is lined through, and if
        /// so, whether a single or double line will be used.
        /// The defined values for the style:text-line-through-type attribute are:
        /// * double: a double line should be used for a line-through text.
        /// * none: deprecated.
        /// * single: a single line should be used for a line-through text.
        /// Every occurrence of the style:text-line-through-type attribute should be accompanied
        /// by an occurrence of the style:text-line-through-style 20.373 attribute on the same
        /// element. There should not be an occurrence of the style:text-line-through-type attribute
        /// if the value of the style:text-line-through-sty
        pub fn set_font_line_through_type(&mut self, ltype: LineType) {
            self.$acc()
                .set_attr("style:text-line-through-type", ltype.to_string());
        }

        /// The style:text-line-through-width attribute specifies the width of a line-through line. The
        /// value bold specifies a line width that is calculated from the font sizes like an auto width, but is
        /// wider than an auto width.
        /// The defined values for the style:text-line-through-width attribute are:
        /// * auto: the width of a line-through should be calculated from the font size of the text where the
        /// line-through will appear.
        /// * bold: the width of a line-through should be calculated from the font size of the text where the
        /// line-through will appear but is wider than for the value of auto.
        /// * a value of type percent 18.3.23
        /// * a value of type positiveInteger 18.2
        /// * a value of type positiveLength 18.3.26
        /// The line-through text styles referenced by the values dash, medium, thick and thin, are
        /// implementation-defined. Thin shall be smaller width than medium and medium shall be a smaller
        /// width than thick.
        pub fn set_font_line_through_width(&mut self, lwidth: LineWidth) {
            self.$acc()
                .set_attr("style:text-line-through-width", lwidth.to_string());
        }

        /// The style:text-outline attribute specifies whether to display an
        /// outline of text or the text itself.
        pub fn set_font_text_outline(&mut self, outline: bool) {
            self.$acc()
                .set_attr("style:text-outline", outline.to_string());
        }

        /// The style:text-overline-color attribute specifies a color that is
        /// used to overline text.
        ///
        /// The defined values for the style:text-overline-color attribute are:
        /// * font-color: the current text color is used for overlining.
        /// * a value of type color
        pub fn set_font_overline_color(&mut self, color: Rgb<u8>) {
            self.$acc()
                .set_attr("style:text-overline-color", color_string(color));
        }

        /// The style:text-overline-mode attribute specifies whether overlining is applied to words
        /// only or to portions of text.
        pub fn set_font_overline_mode(&mut self, lmode: LineMode) {
            self.$acc()
                .set_attr("style:text-overline-mode", lmode.to_string());
        }

        /// The style:text-overline-style attribute specifies a style for rendering a line over text.
        pub fn set_font_overline_style(&mut self, lstyle: LineStyle) {
            self.$acc()
                .set_attr("style:text-overline-style", lstyle.to_string());
        }

        /// The style:text-overline-type attribute specifies the type of overlining applied to a text.
        pub fn set_font_overline_type(&mut self, ltype: LineType) {
            self.$acc()
                .set_attr("style:text-overline-type", ltype.to_string());
        }

        /// The style:text-overline-width attribute specifies the width of an overline. The value bold
        /// specifies a line width that is calculated from the font sizes like an auto width, but is wider than an
        /// auto width.
        pub fn set_font_overline_width(&mut self, lwidth: LineWidth) {
            self.$acc()
                .set_attr("style:text-overline-width", lwidth.to_string());
        }

        /// The style:text-underline-color attribute specifies a color that is used to underline text.
        /// The defined values for the style:text-underline-color attribute are:
        /// * font-color: the current text color is used for underlining.
        /// * a value of type color: the color to be used for underlining.
        pub fn set_font_underline_color(&mut self, color: Rgb<u8>) {
            self.$acc()
                .set_attr("style:text-underline-color", color_string(color));
        }

        /// The style:text-underline-mode attribute specifies whether underlining is applied to words
        /// only or to portions of text. If underlining is applied to text portions, the spaces between words and
        /// the words are underlined.
        pub fn set_font_underline_mode(&mut self, lmode: LineMode) {
            self.$acc()
                .set_attr("style:text-underline-mode", lmode.to_string());
        }

        /// The style:text-underline-style attribute specifies a style for underlining text
        pub fn set_font_underline_style(&mut self, lstyle: LineStyle) {
            self.$acc()
                .set_attr("style:text-underline-style", lstyle.to_string());
        }

        /// The style:text-underline-type attribute specifies the type of underlining applied to a text
        pub fn set_font_underline_type(&mut self, ltype: LineType) {
            self.$acc()
                .set_attr("style:text-underline-type", ltype.to_string());
        }

        /// The style:text-underline-width attribute specifies the width of an underline. The value
        /// bold specifies a line width that is calculated from the font sizes like an auto width, but is wider
        /// than an auto width.
        pub fn set_font_underline_width(&mut self, lwidth: LineWidth) {
            self.$acc()
                .set_attr("style:text-underline-width", lwidth.to_string());
        }

        /// The style:use-window-font-color attribute specifies whether the window foreground color
        /// should be used as the foreground color for a light background color and white for a dark
        /// background color. The determination of light or dark color is implementation-defined.
        pub fn set_use_window_font_color(&mut self, window_color: bool) {
            self.$acc()
                .set_attr("style:use-window-font-color", window_color.to_string());
        }

        /// The text:condition attribute specifies the display of text.
        /// The defined value of the text:condition attribute is none, which means text is hidden.
        pub fn set_text_condition(&mut self, cond: TextCondition) {
            self.$acc().set_attr("text:condition", cond.to_string());
        }

        /// The text:display attribute specifies whether text is hidden.
        /// The defined values for the text:display attribute are:
        /// * condition: text is hidden under the condition specified in the text:condition 20.426
        /// attribute.
        /// * none: text is hidden unconditionally.
        /// * true: text is displayed. This is the default setting
        pub fn set_text_display(&mut self, cond: TextDisplay) {
            self.$acc().set_attr("text:display", cond.to_string());
        }
    };
}

macro_rules! fo_min_height {
    ($acc:ident) => {
        /// Minimum height.
        pub fn set_min_height(&mut self, height: Length) {
            self.$acc().set_attr("fo:min-height", height.to_string());
        }

        /// Minimum height as percentage.
        pub fn set_min_height_percent(&mut self, height: f64) {
            self.$acc()
                .set_attr("fo:min-height", percent_string(height));
        }
    };
}

macro_rules! style_dynamic_spacing {
    ($acc:ident) => {
        /// Dynamic spacing
        pub fn set_dynamic_spacing(&mut self, dynamic: bool) {
            self.$acc()
                .set_attr("style:dynamic-spacing", dynamic.to_string());
        }
    };
}
