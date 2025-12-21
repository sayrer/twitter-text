// Copyright 2025 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

//! Entity types for the nom parser.

/// The type of entity parsed by nom.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NomEntityType {
    Url,
    UrlWithoutProtocol,
    Hashtag,
    Cashtag,
    Username,
    List,
    FederatedMention,
    Emoji,
    InvalidChar,
}

/// An entity parsed by the nom parser.
///
/// Contains the entity type, the matched text slice, and byte positions.
/// For URLs, also tracks the host position for TLD validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NomEntity<'a> {
    /// The type of entity
    pub entity_type: NomEntityType,
    /// The matched text
    pub value: &'a str,
    /// Start byte position in the original input
    pub start: usize,
    /// End byte position in the original input
    pub end: usize,
    /// For List entities: byte position where list_slug starts (after the /)
    pub list_slug_start: Option<usize>,
    /// For URL entities: start of the host/domain portion
    pub host_start: Option<usize>,
    /// For URL entities: end of the host/domain portion
    pub host_end: Option<usize>,
}

impl<'a> NomEntity<'a> {
    /// Create a new entity with basic info.
    pub fn new(entity_type: NomEntityType, value: &'a str, start: usize, end: usize) -> Self {
        Self {
            entity_type,
            value,
            start,
            end,
            list_slug_start: None,
            host_start: None,
            host_end: None,
        }
    }

    /// Create a list entity with slug position.
    pub fn new_list(value: &'a str, start: usize, end: usize, list_slug_start: usize) -> Self {
        Self {
            entity_type: NomEntityType::List,
            value,
            start,
            end,
            list_slug_start: Some(list_slug_start),
            host_start: None,
            host_end: None,
        }
    }

    /// Create a URL entity with host position.
    pub fn new_url(
        entity_type: NomEntityType,
        value: &'a str,
        start: usize,
        end: usize,
        host_start: usize,
        host_end: usize,
    ) -> Self {
        Self {
            entity_type,
            value,
            start,
            end,
            list_slug_start: None,
            host_start: Some(host_start),
            host_end: Some(host_end),
        }
    }
}
