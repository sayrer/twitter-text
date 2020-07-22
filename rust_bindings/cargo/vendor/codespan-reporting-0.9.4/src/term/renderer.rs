use std::io::{self, Write};
use std::ops::{Range, RangeTo};
use termcolor::{ColorSpec, WriteColor};

use crate::diagnostic::{LabelStyle, Severity};
use crate::files::Location;
use crate::term::{Chars, Config, Styles};

/// The 'location focus' of a source code snippet.
pub struct Locus {
    /// The user-facing name of the file.
    pub name: String,
    /// The location.
    pub location: Location,
}

/// Single-line label, with an optional message.
///
/// ```text
/// ^^^^^^^^^ blah blah
/// ```
pub type SingleLabel<'diagnostic> = (LabelStyle, Range<usize>, &'diagnostic str);

/// A multi-line label to render.
///
/// Locations are relative to the start of where the source cord is rendered.
pub enum MultiLabel<'diagnostic> {
    /// Left top corner for multi-line labels.
    ///
    /// ```text
    /// ╭
    /// ```
    TopLeft(LabelStyle),
    /// Multi-line label top.
    ///
    /// ```text
    /// ╭────────────^
    /// ```
    Top(LabelStyle, RangeTo<usize>),
    /// Left vertical labels for multi-line labels.
    ///
    /// ```text
    /// │
    /// ```
    Left(LabelStyle),
    /// Multi-line label bottom, with an optional message.
    ///
    /// ```text
    /// ╰────────────^ blah blah
    /// ```
    Bottom(LabelStyle, RangeTo<usize>, &'diagnostic str),
}

#[derive(Copy, Clone)]
enum VerticalBound {
    Top,
    Bottom,
}

type Underline = (LabelStyle, VerticalBound);

/// A renderer of display list entries.
///
/// The following diagram gives an overview of each of the parts of the renderer's output:
///
/// ```text
///                     ┌ outer gutter
///                     │ ┌ left border
///                     │ │ ┌ inner gutter
///                     │ │ │   ┌─────────────────────────── source ─────────────────────────────┐
///                     │ │ │   │                                                                │
///                  ┌────────────────────────────────────────────────────────────────────────────
///        header ── │ error[0001]: oh noes, a cupcake has occurred!
///         empty ── │
/// snippet start ── │    ┌─ test:9:0
/// snippet empty ── │    │
///  snippet line ── │  9 │   ╭ Cupcake ipsum dolor. Sit amet marshmallow topping cheesecake
///  snippet line ── │ 10 │   │ muffin. Halvah croissant candy canes bonbon candy. Apple pie jelly
///                  │    │ ╭─│─────────^
/// snippet break ── │    · │ │
///  snippet line ── │ 33 │ │ │ Muffin danish chocolate soufflé pastry icing bonbon oat cake.
///  snippet line ── │ 34 │ │ │ Powder cake jujubes oat cake. Lemon drops tootsie roll marshmallow
///                  │    │ │ ╰─────────────────────────────^ blah blah
/// snippet break ── │    · │
///  snippet line ── │ 38 │ │   Brownie lemon drops chocolate jelly-o candy canes. Danish marzipan
///  snippet line ── │ 39 │ │   jujubes soufflé carrot cake marshmallow tiramisu caramels candy canes.
///                  │    │ │           ^^^^^^^^^^^^^^^^^^^ -------------------- blah blah
///                  │    │ │           │
///                  │    │ │           blah blah
///                  │    │ │           note: this is a note
///  snippet line ── │ 40 │ │   Fruitcake jelly-o danish toffee. Tootsie roll pastry cheesecake
///  snippet line ── │ 41 │ │   soufflé marzipan. Chocolate bar oat cake jujubes lollipop pastry
///  snippet line ── │ 42 │ │   cupcake. Candy canes cupcake toffee gingerbread candy canes muffin
///                  │    │ │                                ^^^^^^^^^^^^^^^^^^ blah blah
///                  │    │ ╰──────────^ blah blah
/// snippet break ── │    ·
///  snippet line ── │ 82 │     gingerbread toffee chupa chups chupa chups jelly-o cotton candy.
///                  │    │                 ^^^^^^                         ------- blah blah
/// snippet empty ── │    │
///  snippet note ── │    = blah blah
///  snippet note ── │    = blah blah blah
///                  │      blah blah
///  snippet note ── │    = blah blah blah
///                  │      blah blah
///         empty ── │
/// ```
///
/// Filler text from http://www.cupcakeipsum.com
pub struct Renderer<'writer, 'config> {
    writer: &'writer mut dyn WriteColor,
    config: &'config Config,
}

impl<'writer, 'config> Renderer<'writer, 'config> {
    /// Construct a renderer from the given writer and config.
    pub fn new(
        writer: &'writer mut dyn WriteColor,
        config: &'config Config,
    ) -> Renderer<'writer, 'config> {
        Renderer { writer, config }
    }

    fn chars(&self) -> &'config Chars {
        &self.config.chars
    }

    fn styles(&self) -> &'config Styles {
        &self.config.styles
    }

    /// Diagnostic header, with severity, code, and message.
    ///
    /// ```text
    /// error[E0001]: unexpected type in `+` application
    /// ```
    pub fn render_header(
        &mut self,
        locus: Option<&Locus>,
        severity: Severity,
        code: Option<&str>,
        message: &str,
    ) -> io::Result<()> {
        // Write locus
        //
        // ```text
        // test:2:9:
        // ```
        if let Some(locus) = locus {
            self.snippet_locus(locus)?;
            write!(self, ": ")?;
        }

        // Write severity name
        //
        // ```text
        // error
        // ```
        self.set_color(self.styles().header(severity))?;
        match severity {
            Severity::Bug => write!(self, "bug")?,
            Severity::Error => write!(self, "error")?,
            Severity::Warning => write!(self, "warning")?,
            Severity::Help => write!(self, "help")?,
            Severity::Note => write!(self, "note")?,
        }

        // Write error code
        //
        // ```text
        // [E0001]
        // ```
        if let Some(code) = &code {
            write!(self, "[{}]", code)?;
        }

        // Write diagnostic message
        //
        // ```text
        // : unexpected type in `+` application
        // ```
        self.set_color(&self.styles().header_message)?;
        write!(self, ": {}", message)?;
        self.reset()?;

        write!(self, "\n")?;

        Ok(())
    }

    /// Empty line.
    pub fn render_empty(&mut self) -> io::Result<()> {
        write!(self, "\n")?;

        Ok(())
    }

    /// Top left border and locus.
    ///
    /// ```text
    /// ┌─ test:2:9
    /// ```
    pub fn render_snippet_start(&mut self, outer_padding: usize, locus: &Locus) -> io::Result<()> {
        self.outer_gutter(outer_padding)?;

        self.set_color(&self.styles().source_border)?;
        write!(self, "{}", self.chars().source_border_top_left)?;
        write!(self, "{0}", self.chars().source_border_top)?;
        self.reset()?;

        write!(self, " ")?;
        self.snippet_locus(&locus)?;

        write!(self, "\n")?;

        Ok(())
    }

    /// A line of source code.
    ///
    /// ```text
    /// 10 │   │ muffin. Halvah croissant candy canes bonbon candy. Apple pie jelly
    ///    │ ╭─│─────────^
    /// ```
    pub fn render_snippet_source(
        &mut self,
        outer_padding: usize,
        line_number: usize,
        source: &str,
        severity: Severity,
        single_labels: &[SingleLabel<'_>],
        num_multi_labels: usize,
        multi_labels: &[(usize, MultiLabel<'_>)],
    ) -> io::Result<()> {
        // Trim trailing newlines, linefeeds, and null chars from source, if they exist.
        // FIXME: Use the number of trimmed placeholders when rendering single line carets
        let source = source.trim_end_matches(['\n', '\r', '\0'].as_ref());

        // Write source line
        //
        // ```text
        // 10 │   │ muffin. Halvah croissant candy canes bonbon candy. Apple pie jelly
        // ```
        {
            // Write outer gutter (with line number) and border
            self.outer_gutter_number(line_number, outer_padding)?;
            self.border_left()?;

            // Write inner gutter (with multi-line continuations on the left if necessary)
            let mut multi_labels_iter = multi_labels.iter().peekable();
            for label_column in 0..num_multi_labels {
                match multi_labels_iter.peek() {
                    Some((label_index, label)) if *label_index == label_column => {
                        match label {
                            MultiLabel::TopLeft(label_style) => {
                                self.label_multi_top_left(severity, *label_style)?;
                            }
                            MultiLabel::Top(..) => self.inner_gutter_space()?,
                            MultiLabel::Left(label_style) | MultiLabel::Bottom(label_style, ..) => {
                                self.label_multi_left(severity, *label_style, None)?;
                            }
                        }
                        multi_labels_iter.next();
                    }
                    Some((_, _)) | None => self.inner_gutter_space()?,
                }
            }

            // Write source
            write!(self.config.source(self.writer), " {}", source)?;
            write!(self, "\n")?;
        }

        // Write single labels underneath source
        //
        // ```text
        //   │     - ---- ^^^ second mutable borrow occurs here
        //   │     │ │
        //   │     │ first mutable borrow occurs here
        //   │     first borrow later used by call
        //   │     help: some help here
        // ```
        if !single_labels.is_empty() {
            // Our plan is as follows:
            //
            // 1. Do an initial scan to find:
            //    - The number of non-empty messages.
            //    - The right-most start and end positions of labels.
            //    - A candidate for a trailing label (where the label's message
            //      is printed to the left of the caret).
            // 2. Check if the trailing label candidate overlaps another label -
            //    if so we print it underneath the carets with the other labels.
            // 3. Print a line of carets, and (possibly) the trailing message
            //    to the left.
            // 4. Print vertical lines pointing to the carets, and the messages
            //    for those carets.
            //
            // We try our best avoid introducing new dynamic allocations,
            // instead preferring to iterate over the labels multiple times. It
            // is unclear what the performance tradeoffs are however, so further
            // investigation may be required.

            // The number of non-empty messages to print.
            let mut num_messages = 0;
            // The right-most start position, eg:
            //
            // ```text
            // -^^^^---- ^^^^^^^
            //           │
            //           right-most start position
            // ```
            let mut max_label_start = 0;
            // The right-most end position, eg:
            //
            // ```text
            // -^^^^---- ^^^^^^^
            //                 │
            //                 right-most end position
            // ```
            let mut max_label_end = 0;
            // A trailing message, eg:
            //
            // ```text
            // ^^^ second mutable borrow occurs here
            // ```
            let mut trailing_label = None;

            for (label_index, label) in single_labels.iter().enumerate() {
                let (_, range, message) = label;
                if !message.is_empty() {
                    num_messages += 1;
                }
                max_label_start = std::cmp::max(max_label_start, range.start);
                max_label_end = std::cmp::max(max_label_end, range.end);
                // This is a candidate for the trailing label, so let's record it.
                if range.end == max_label_end {
                    if message.is_empty() {
                        trailing_label = None;
                    } else {
                        trailing_label = Some((label_index, label));
                    }
                }
            }
            if let Some((trailing_label_index, (_, trailing_range, _))) = trailing_label {
                // Check to see if the trailing label candidate overlaps any of
                // the other labels on the current line.
                if single_labels
                    .iter()
                    .enumerate()
                    .filter(|(label_index, _)| *label_index != trailing_label_index)
                    .any(|(_, (_, range, _))| is_overlapping(trailing_range, range))
                {
                    // If it does, we'll instead want to render it below the
                    // carets along with the other hanging labels.
                    trailing_label = None;
                }
            }

            // Write a line of carets
            //
            // ```text
            //   │ ^^^^^^  -------^^^^^^^^^-------^^^^^----- ^^^^ trailing label message
            // ```
            self.outer_gutter(outer_padding)?;
            self.border_left()?;
            self.inner_gutter(severity, num_multi_labels, multi_labels)?;
            write!(self, " ")?;

            let mut previous_label_style = None;
            for (byte_index, ch) in source
                .char_indices()
                // Add a placeholder source column at the end to allow for
                // printing carets at the end of lines, eg:
                //
                // ```text
                // 1 │ Hello world!
                //   │             ^
                // ```
                //
                // ' ' is used because its unicode width is equal to 1 according
                // to `unicode_width::UnicodeWidthChar`.
                .chain(std::iter::once((source.len(), ' ')))
            {
                // Find the current label style at this column
                let column_range = byte_index..(byte_index + ch.len_utf8());
                let current_label_style = single_labels
                    .iter()
                    .filter(|(_, range, _)| is_overlapping(range, &column_range))
                    .map(|(label_style, _, _)| *label_style)
                    .max_by_key(label_priority_key);

                // Update writer style if necessary
                if previous_label_style != current_label_style {
                    match current_label_style {
                        None => self.reset()?,
                        Some(label_style) => {
                            self.set_color(self.styles().label(severity, label_style))?;
                        }
                    }
                }

                let caret_ch = match current_label_style {
                    Some(LabelStyle::Primary) => Some(self.chars().single_primary_caret),
                    Some(LabelStyle::Secondary) => Some(self.chars().single_secondary_caret),
                    // Only print padding if we are before the end of the last single line caret
                    None if byte_index < max_label_end => Some(' '),
                    None => None,
                };
                if let Some(caret_ch) = caret_ch {
                    // FIXME: improve rendering for carets that occurring within character boundaries
                    for _ in 0..self.config.width(ch) {
                        write!(self, "{}", caret_ch)?;
                    }
                }

                previous_label_style = current_label_style;
            }
            // Reset style if it was previously set
            if previous_label_style.is_some() {
                self.reset()?;
            }
            // Write first trailing label message
            if let Some((_, (label_style, _, message))) = trailing_label {
                write!(self, " ")?;
                self.set_color(self.styles().label(severity, *label_style))?;
                write!(self, "{}", message)?;
                self.reset()?;
            }
            write!(self, "\n")?;

            // Write hanging labels pointing to carets
            //
            // ```text
            //   │     │ │
            //   │     │ first mutable borrow occurs here
            //   │     first borrow later used by call
            //   │     help: some help here
            // ```
            if num_messages > trailing_label.iter().count() {
                // Write first set of vertical lines before hanging labels
                //
                // ```text
                //   │     │ │
                // ```
                self.outer_gutter(outer_padding)?;
                self.border_left()?;
                self.inner_gutter(severity, num_multi_labels, multi_labels)?;
                write!(self, " ")?;
                self.caret_pointers(
                    severity,
                    max_label_start,
                    single_labels,
                    trailing_label,
                    source.char_indices(),
                )?;
                write!(self, "\n")?;

                // Write hanging labels pointing to carets
                //
                // ```text
                //   │     │ first mutable borrow occurs here
                //   │     first borrow later used by call
                //   │     help: some help here
                // ```
                for (label_style, range, message) in
                    hanging_labels(single_labels, trailing_label).rev()
                {
                    self.outer_gutter(outer_padding)?;
                    self.border_left()?;
                    self.inner_gutter(severity, num_multi_labels, multi_labels)?;
                    write!(self, " ")?;
                    self.caret_pointers(
                        severity,
                        max_label_start,
                        single_labels,
                        trailing_label,
                        source
                            .char_indices()
                            .take_while(|(byte_index, _)| *byte_index < range.start),
                    )?;
                    self.set_color(self.styles().label(severity, *label_style))?;
                    write!(self, "{}", message)?;
                    self.reset()?;
                    write!(self, "\n")?;
                }
            }
        }

        // Write top or bottom label carets underneath source
        //
        // ```text
        //     │ ╰───│──────────────────^ woops
        //     │   ╭─│─────────^
        // ```
        for (multi_label_index, (_, label)) in multi_labels.iter().enumerate() {
            let (label_style, range, bottom_message) = match label {
                MultiLabel::TopLeft(_) | MultiLabel::Left(_) => continue, // no label caret needed
                MultiLabel::Top(ls, range) => (*ls, range, None),
                MultiLabel::Bottom(ls, range, message) => (*ls, range, Some(message)),
            };

            self.outer_gutter(outer_padding)?;
            self.border_left()?;

            // Write inner gutter.
            //
            // ```text
            //  │ ╭─│───│
            // ```
            let mut underline = None;
            let mut multi_labels_iter = multi_labels.iter().enumerate().peekable();
            for label_column in 0..num_multi_labels {
                match multi_labels_iter.peek() {
                    Some((i, (label_index, label))) if *label_index == label_column => {
                        match label {
                            MultiLabel::TopLeft(ls) | MultiLabel::Left(ls) => {
                                self.label_multi_left(severity, *ls, underline.map(|(s, _)| s))?;
                            }
                            MultiLabel::Top(ls, ..) if multi_label_index > *i => {
                                self.label_multi_left(severity, *ls, underline.map(|(s, _)| s))?;
                            }
                            MultiLabel::Bottom(ls, ..) if multi_label_index < *i => {
                                self.label_multi_left(severity, *ls, underline.map(|(s, _)| s))?;
                            }
                            MultiLabel::Top(ls, ..) if multi_label_index == *i => {
                                underline = Some((*ls, VerticalBound::Top));
                                self.label_multi_top_left(severity, label_style)?
                            }
                            MultiLabel::Bottom(ls, ..) if multi_label_index == *i => {
                                underline = Some((*ls, VerticalBound::Bottom));
                                self.label_multi_bottom_left(severity, label_style)?;
                            }
                            MultiLabel::Top(..) | MultiLabel::Bottom(..) => {
                                self.inner_gutter_column(severity, underline)?;
                            }
                        }
                        multi_labels_iter.next();
                    }
                    Some((_, _)) | None => self.inner_gutter_column(severity, underline)?,
                }
            }

            // Finish the top or bottom caret
            let range = range.clone();
            match bottom_message {
                None => self.label_multi_top_caret(severity, label_style, source, range)?,
                Some(message) => {
                    self.label_multi_bottom_caret(severity, label_style, source, range, message)?
                }
            }
        }

        Ok(())
    }

    /// An empty source line, for providing additional whitespace to source snippets.
    ///
    /// ```text
    /// │ │ │
    /// ```
    pub fn render_snippet_empty(
        &mut self,
        outer_padding: usize,
        severity: Severity,
        num_multi_labels: usize,
        multi_labels: &[(usize, MultiLabel<'_>)],
    ) -> io::Result<()> {
        self.outer_gutter(outer_padding)?;
        self.border_left()?;
        self.inner_gutter(severity, num_multi_labels, multi_labels)?;
        write!(self, "\n")?;
        Ok(())
    }

    /// A broken source line, for labeling skipped sections of source.
    ///
    /// ```text
    /// · │ │
    /// ```
    pub fn render_snippet_break(
        &mut self,
        outer_padding: usize,
        severity: Severity,
        num_multi_labels: usize,
        multi_labels: &[(usize, MultiLabel<'_>)],
    ) -> io::Result<()> {
        self.outer_gutter(outer_padding)?;
        self.border_left_break()?;
        self.inner_gutter(severity, num_multi_labels, multi_labels)?;
        write!(self, "\n")?;
        Ok(())
    }

    /// Additional notes.
    ///
    /// ```text
    /// = expected type `Int`
    ///      found type `String`
    /// ```
    pub fn render_snippet_note(&mut self, outer_padding: usize, message: &str) -> io::Result<()> {
        for (note_line_index, line) in message.lines().enumerate() {
            self.outer_gutter(outer_padding)?;
            match note_line_index {
                0 => {
                    self.set_color(&self.styles().note_bullet)?;
                    write!(self, "{}", self.chars().note_bullet)?;
                    self.reset()?;
                }
                _ => write!(self, " ")?,
            }
            // Write line of message
            write!(self, " {}", line)?;
            write!(self, "\n")?;
        }

        Ok(())
    }

    /// Location focus.
    fn snippet_locus(&mut self, locus: &Locus) -> io::Result<()> {
        write!(
            self,
            "{origin}:{line_number}:{column_number}",
            origin = locus.name,
            line_number = locus.location.line_number,
            column_number = locus.location.column_number,
        )
    }

    /// The outer gutter of a source line.
    fn outer_gutter(&mut self, outer_padding: usize) -> io::Result<()> {
        write!(self, "{space: >width$}", space = "", width = outer_padding,)?;
        write!(self, " ")?;
        Ok(())
    }

    /// The outer gutter of a source line, with line number.
    fn outer_gutter_number(&mut self, line_number: usize, outer_padding: usize) -> io::Result<()> {
        self.set_color(&self.styles().line_number)?;
        write!(
            self,
            "{line_number: >width$}",
            line_number = line_number,
            width = outer_padding,
        )?;
        self.reset()?;
        write!(self, " ")?;
        Ok(())
    }

    /// The left-hand border of a source line.
    fn border_left(&mut self) -> io::Result<()> {
        self.set_color(&self.styles().source_border)?;
        write!(self, "{}", self.chars().source_border_left)?;
        self.reset()?;
        Ok(())
    }

    /// The broken left-hand border of a source line.
    fn border_left_break(&mut self) -> io::Result<()> {
        self.set_color(&self.styles().source_border)?;
        write!(self, "{}", self.chars().source_border_left_break)?;
        self.reset()?;
        Ok(())
    }

    /// Write vertical lines pointing to carets.
    fn caret_pointers(
        &mut self,
        severity: Severity,
        max_label_start: usize,
        single_labels: &[SingleLabel<'_>],
        trailing_label: Option<(usize, &SingleLabel<'_>)>,
        char_indices: impl Iterator<Item = (usize, char)>,
    ) -> io::Result<()> {
        for (byte_index, ch) in char_indices {
            let column_range = byte_index..(byte_index + ch.len_utf8());
            let label_style = hanging_labels(single_labels, trailing_label)
                .filter(|(_, range, _)| column_range.contains(&range.start))
                .map(|(label_style, _, _)| *label_style)
                .max_by_key(label_priority_key);

            let spaces = match label_style {
                None => 0..self.config.width(ch),
                Some(label_style) => {
                    self.set_color(self.styles().label(severity, label_style))?;
                    write!(self, "{}", self.chars().pointer_left)?;
                    self.reset()?;
                    1..self.config.width(ch)
                }
            };
            // Only print padding if we are before the end of the last single line caret
            if byte_index <= max_label_start {
                for _ in spaces {
                    write!(self, " ")?;
                }
            }
        }

        Ok(())
    }

    /// The left of a multi-line label.
    ///
    /// ```text
    ///  │
    /// ```
    fn label_multi_left(
        &mut self,
        severity: Severity,
        label_style: LabelStyle,
        underline: Option<LabelStyle>,
    ) -> io::Result<()> {
        match underline {
            None => write!(self, " ")?,
            // Continue an underline horizontally
            Some(label_style) => {
                self.set_color(self.styles().label(severity, label_style))?;
                write!(self, "{}", self.chars().multi_top)?;
                self.reset()?;
            }
        }
        self.set_color(self.styles().label(severity, label_style))?;
        write!(self, "{}", self.chars().multi_left)?;
        self.reset()?;
        Ok(())
    }

    /// The top-left of a multi-line label.
    ///
    /// ```text
    ///  ╭
    /// ```
    fn label_multi_top_left(
        &mut self,
        severity: Severity,
        label_style: LabelStyle,
    ) -> io::Result<()> {
        write!(self, " ")?;
        self.set_color(self.styles().label(severity, label_style))?;
        write!(self, "{}", self.chars().multi_top_left)?;
        self.reset()?;
        Ok(())
    }

    /// The bottom left of a multi-line label.
    ///
    /// ```text
    ///  ╰
    /// ```
    fn label_multi_bottom_left(
        &mut self,
        severity: Severity,
        label_style: LabelStyle,
    ) -> io::Result<()> {
        write!(self, " ")?;
        self.set_color(self.styles().label(severity, label_style))?;
        write!(self, "{}", self.chars().multi_bottom_left)?;
        self.reset()?;
        Ok(())
    }

    /// Multi-line label top.
    ///
    /// ```text
    /// ─────────────^
    /// ```
    fn label_multi_top_caret(
        &mut self,
        severity: Severity,
        label_style: LabelStyle,
        source: &str,
        range: RangeTo<usize>,
    ) -> io::Result<()> {
        self.set_color(self.styles().label(severity, label_style))?;

        for (_, ch) in source
            .char_indices()
            .take_while(|(byte_index, _)| *byte_index < range.end + 1)
        {
            // FIXME: improve rendering for carets that occurring within character boundaries
            for _ in 0..self.config.width(ch) {
                write!(self, "{}", self.chars().multi_top)?;
            }
        }

        let caret_start = match label_style {
            LabelStyle::Primary => self.config.chars.multi_primary_caret_start,
            LabelStyle::Secondary => self.config.chars.multi_secondary_caret_start,
        };
        write!(self, "{}", caret_start)?;
        self.reset()?;
        write!(self, "\n")?;
        Ok(())
    }

    /// Multi-line label bottom, with a message.
    ///
    /// ```text
    /// ─────────────^ expected `Int` but found `String`
    /// ```
    fn label_multi_bottom_caret(
        &mut self,
        severity: Severity,
        label_style: LabelStyle,
        source: &str,
        range: RangeTo<usize>,
        message: &str,
    ) -> io::Result<()> {
        self.set_color(self.styles().label(severity, label_style))?;

        for (_, ch) in source
            .char_indices()
            .take_while(|(byte_index, _)| *byte_index < range.end)
        {
            // FIXME: improve rendering for carets that occurring within character boundaries
            for _ in 0..self.config.width(ch) {
                write!(self, "{}", self.chars().multi_bottom)?;
            }
        }

        let caret_end = match label_style {
            LabelStyle::Primary => self.config.chars.multi_primary_caret_start,
            LabelStyle::Secondary => self.config.chars.multi_secondary_caret_start,
        };
        write!(self, "{}", caret_end)?;
        if !message.is_empty() {
            write!(self, " {}", message)?;
        }
        self.reset()?;
        write!(self, "\n")?;
        Ok(())
    }

    /// Writes an empty gutter space, or continues an underline horizontally.
    fn inner_gutter_column(
        &mut self,
        severity: Severity,
        underline: Option<Underline>,
    ) -> io::Result<()> {
        match underline {
            None => self.inner_gutter_space(),
            Some((label_style, vertical_bound)) => {
                self.set_color(self.styles().label(severity, label_style))?;
                let ch = match vertical_bound {
                    VerticalBound::Top => self.config.chars.multi_top,
                    VerticalBound::Bottom => self.config.chars.multi_bottom,
                };
                write!(self, "{0}{0}", ch)?;
                self.reset()?;
                Ok(())
            }
        }
    }

    /// Writes an empty gutter space.
    fn inner_gutter_space(&mut self) -> io::Result<()> {
        write!(self, "  ")
    }

    /// Writes an inner gutter, with the left lines if necessary.
    fn inner_gutter(
        &mut self,
        severity: Severity,
        num_multi_labels: usize,
        multi_labels: &[(usize, MultiLabel<'_>)],
    ) -> io::Result<()> {
        let mut multi_labels_iter = multi_labels.iter().peekable();
        for label_column in 0..num_multi_labels {
            match multi_labels_iter.peek() {
                Some((label_index, label)) if *label_index == label_column => match label {
                    MultiLabel::TopLeft(label_style)
                    | MultiLabel::Left(label_style)
                    | MultiLabel::Bottom(label_style, ..) => {
                        self.label_multi_left(severity, *label_style, None)?;
                        multi_labels_iter.next();
                    }
                    MultiLabel::Top(..) => {
                        self.inner_gutter_space()?;
                        multi_labels_iter.next();
                    }
                },
                Some((_, _)) | None => self.inner_gutter_space()?,
            }
        }

        Ok(())
    }
}

impl<'writer, 'config> Write for Renderer<'writer, 'config> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<'writer, 'config> WriteColor for Renderer<'writer, 'config> {
    fn supports_color(&self) -> bool {
        self.writer.supports_color()
    }

    fn set_color(&mut self, spec: &ColorSpec) -> io::Result<()> {
        self.writer.set_color(spec)
    }

    fn reset(&mut self) -> io::Result<()> {
        self.writer.reset()
    }

    fn is_synchronous(&self) -> bool {
        self.writer.is_synchronous()
    }
}

/// Check if two ranges overlap
fn is_overlapping(range0: &Range<usize>, range1: &Range<usize>) -> bool {
    let start = std::cmp::max(range0.start, range1.start);
    let end = std::cmp::min(range0.end, range1.end);
    start < end
}

/// For prioritizing primary labels over secondary labels when rendering carets.
fn label_priority_key(label_style: &LabelStyle) -> u8 {
    match label_style {
        LabelStyle::Secondary => 0,
        LabelStyle::Primary => 1,
    }
}

/// Return an iterator that yields the labels that require hanging messages
/// rendered underneath them.
fn hanging_labels<'labels, 'diagnostic>(
    single_labels: &'labels [SingleLabel<'diagnostic>],
    trailing_label: Option<(usize, &'labels SingleLabel<'diagnostic>)>,
) -> impl 'labels + DoubleEndedIterator<Item = &'labels SingleLabel<'diagnostic>> {
    single_labels
        .iter()
        .enumerate()
        .filter(|(_, (_, _, message))| !message.is_empty())
        .filter(move |(i, _)| trailing_label.map_or(true, |(j, _)| *i != j))
        .map(|(_, label)| label)
}