pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum BodyCreatePodcastV1StudioPodcastsPostSource {
        PodcastTextSource(PodcastTextSource),

        PodcastUrlSource(PodcastUrlSource),

        BodyCreatePodcastV1StudioPodcastsPostSourceTwoItemList(Vec<BodyCreatePodcastV1StudioPodcastsPostSourceTwoItem>),
}

impl BodyCreatePodcastV1StudioPodcastsPostSource {
    pub fn is_podcast_text_source(&self) -> bool {
        matches!(self, Self::PodcastTextSource(_))
    }

    pub fn is_podcast_url_source(&self) -> bool {
        matches!(self, Self::PodcastUrlSource(_))
    }

    pub fn is_body_create_podcast_v1studio_podcasts_post_source_two_item_list(&self) -> bool {
        matches!(self, Self::BodyCreatePodcastV1StudioPodcastsPostSourceTwoItemList(_))
    }


    pub fn as_podcast_text_source(&self) -> Option<&PodcastTextSource> {
        match self {
                    Self::PodcastTextSource(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_podcast_text_source(self) -> Option<PodcastTextSource> {
        match self {
                    Self::PodcastTextSource(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_podcast_url_source(&self) -> Option<&PodcastUrlSource> {
        match self {
                    Self::PodcastUrlSource(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_podcast_url_source(self) -> Option<PodcastUrlSource> {
        match self {
                    Self::PodcastUrlSource(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_body_create_podcast_v1studio_podcasts_post_source_two_item_list(&self) -> Option<&Vec<BodyCreatePodcastV1StudioPodcastsPostSourceTwoItem>> {
        match self {
                    Self::BodyCreatePodcastV1StudioPodcastsPostSourceTwoItemList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_body_create_podcast_v1studio_podcasts_post_source_two_item_list(self) -> Option<Vec<BodyCreatePodcastV1StudioPodcastsPostSourceTwoItem>> {
        match self {
                    Self::BodyCreatePodcastV1StudioPodcastsPostSourceTwoItemList(value) => Some(value),
                    _ => None,
                }
    }
}
