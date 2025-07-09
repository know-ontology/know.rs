// This is free and unencumbered software released into the public domain.

#[derive(Clone, Debug, Default)]
pub struct DisplayConfig {
    pub color: bool,
    pub emojis: bool,
}

#[derive(Debug)]
pub struct DisplayInline<'a, T>(pub &'a T);

#[derive(Debug)]
pub struct DisplayOneliner<'a, T>(pub &'a T);

#[derive(Debug)]
pub struct DisplayConcise<'a, T>(pub &'a T);

#[derive(Debug)]
pub struct DisplayDetailed<'a, T>(pub &'a T);

#[derive(Debug)]
pub struct DisplayMarkdown<'a, T>(pub &'a T);

#[derive(Debug)]
pub struct DisplayMime<'a, T>(pub &'a T);
