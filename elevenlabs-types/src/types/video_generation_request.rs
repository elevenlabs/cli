pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "model_id")]
#[non_exhaustive]
pub enum VideoGenerationRequest {
        #[serde(rename = "bytedance-seedance-v2")]
        #[non_exhaustive]
        BytedanceSeedanceV2 {
            #[serde(skip_serializing_if = "Option::is_none")]
            webhook: Option<WebhookTarget>,
            #[serde(default)]
            prompt: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            aspect_ratio: Option<BytedanceSeedance2RequestAspectRatio>,
            #[serde(skip_serializing_if = "Option::is_none")]
            duration_secs: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            seed: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            generate_audio: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            start_frame: Option<ImageReference>,
            #[serde(skip_serializing_if = "Option::is_none")]
            end_frame: Option<ImageReference>,
            #[serde(skip_serializing_if = "Option::is_none")]
            images: Option<Vec<ImageReference>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            videos: Option<Vec<VideoReference>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            audios: Option<Vec<AudioReference>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            resolution: Option<BytedanceSeedance2RequestResolution>,
        },

        #[serde(rename = "bytedance-seedance-v2-fast")]
        #[non_exhaustive]
        BytedanceSeedanceV2Fast {
            #[serde(skip_serializing_if = "Option::is_none")]
            webhook: Option<WebhookTarget>,
            #[serde(default)]
            prompt: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            aspect_ratio: Option<BytedanceSeedance2FastRequestAspectRatio>,
            #[serde(skip_serializing_if = "Option::is_none")]
            duration_secs: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            seed: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            generate_audio: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            start_frame: Option<ImageReference>,
            #[serde(skip_serializing_if = "Option::is_none")]
            end_frame: Option<ImageReference>,
            #[serde(skip_serializing_if = "Option::is_none")]
            images: Option<Vec<ImageReference>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            videos: Option<Vec<VideoReference>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            audios: Option<Vec<AudioReference>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            resolution: Option<BytedanceSeedance2FastRequestResolution>,
        },

        #[serde(rename = "bytedance-seedance-v2-mini")]
        #[non_exhaustive]
        BytedanceSeedanceV2Mini {
            #[serde(skip_serializing_if = "Option::is_none")]
            webhook: Option<WebhookTarget>,
            #[serde(default)]
            prompt: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            aspect_ratio: Option<BytedanceSeedance2MiniRequestAspectRatio>,
            #[serde(skip_serializing_if = "Option::is_none")]
            duration_secs: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            seed: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            generate_audio: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            start_frame: Option<ImageReference>,
            #[serde(skip_serializing_if = "Option::is_none")]
            end_frame: Option<ImageReference>,
            #[serde(skip_serializing_if = "Option::is_none")]
            images: Option<Vec<ImageReference>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            videos: Option<Vec<VideoReference>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            audios: Option<Vec<AudioReference>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            resolution: Option<BytedanceSeedance2MiniRequestResolution>,
        },

        #[serde(rename = "bytedance-seedance-v2.5")]
        #[non_exhaustive]
        BytedanceSeedanceV25 {
            #[serde(skip_serializing_if = "Option::is_none")]
            webhook: Option<WebhookTarget>,
            #[serde(default)]
            prompt: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            aspect_ratio: Option<BytedanceSeedance25RequestAspectRatio>,
            #[serde(skip_serializing_if = "Option::is_none")]
            resolution: Option<BytedanceSeedance25RequestResolution>,
            #[serde(skip_serializing_if = "Option::is_none")]
            duration_secs: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            generate_audio: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            start_frame: Option<ImageReference>,
            #[serde(skip_serializing_if = "Option::is_none")]
            end_frame: Option<ImageReference>,
            #[serde(skip_serializing_if = "Option::is_none")]
            images: Option<Vec<ImageReference>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            videos: Option<Vec<VideoReference>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            audios: Option<Vec<AudioReference>>,
        },

        #[serde(rename = "creatify-aurora")]
        #[non_exhaustive]
        CreatifyAurora {
            #[serde(skip_serializing_if = "Option::is_none")]
            webhook: Option<WebhookTarget>,
            image: ImageReference,
            audio: AudioReference,
            #[serde(skip_serializing_if = "Option::is_none")]
            resolution: Option<CreatifyAuroraRequestResolution>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            guidance_scale: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            audio_guidance_scale: Option<f64>,
        },

        #[serde(rename = "veo-3.1-fast-generate-001")]
        #[non_exhaustive]
        Veo31FastGenerate001 {
            #[serde(skip_serializing_if = "Option::is_none")]
            webhook: Option<WebhookTarget>,
            #[serde(default)]
            prompt: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            negative_prompt: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            seed: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            enhance_prompt: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            duration_secs: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            aspect_ratio: Option<Veo31FastRequestAspectRatio>,
            #[serde(skip_serializing_if = "Option::is_none")]
            resolution: Option<Veo31FastRequestResolution>,
            #[serde(skip_serializing_if = "Option::is_none")]
            generate_audio: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            start_frame: Option<ImageReference>,
            #[serde(skip_serializing_if = "Option::is_none")]
            end_frame: Option<ImageReference>,
            #[serde(skip_serializing_if = "Option::is_none")]
            images: Option<Vec<VeoImageReference>>,
        },

        #[serde(rename = "veo-3.1-generate-001")]
        #[non_exhaustive]
        Veo31Generate001 {
            #[serde(skip_serializing_if = "Option::is_none")]
            webhook: Option<WebhookTarget>,
            #[serde(default)]
            prompt: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            negative_prompt: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            seed: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            enhance_prompt: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            duration_secs: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            aspect_ratio: Option<Veo31RequestAspectRatio>,
            #[serde(skip_serializing_if = "Option::is_none")]
            resolution: Option<Veo31RequestResolution>,
            #[serde(skip_serializing_if = "Option::is_none")]
            generate_audio: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            start_frame: Option<ImageReference>,
            #[serde(skip_serializing_if = "Option::is_none")]
            end_frame: Option<ImageReference>,
            #[serde(skip_serializing_if = "Option::is_none")]
            images: Option<Vec<VeoImageReference>>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl VideoGenerationRequest {
    pub fn bytedance_seedance_v2(prompt: String) -> Self {
        Self::BytedanceSeedanceV2 { webhook: None, prompt, aspect_ratio: None, duration_secs: None, seed: None, generate_audio: None, start_frame: None, end_frame: None, images: None, videos: None, audios: None, resolution: None }
    }

    pub fn bytedance_seedance_v2fast(prompt: String) -> Self {
        Self::BytedanceSeedanceV2Fast { webhook: None, prompt, aspect_ratio: None, duration_secs: None, seed: None, generate_audio: None, start_frame: None, end_frame: None, images: None, videos: None, audios: None, resolution: None }
    }

    pub fn bytedance_seedance_v2mini(prompt: String) -> Self {
        Self::BytedanceSeedanceV2Mini { webhook: None, prompt, aspect_ratio: None, duration_secs: None, seed: None, generate_audio: None, start_frame: None, end_frame: None, images: None, videos: None, audios: None, resolution: None }
    }

    pub fn bytedance_seedance_v25(prompt: String) -> Self {
        Self::BytedanceSeedanceV25 { webhook: None, prompt, aspect_ratio: None, resolution: None, duration_secs: None, generate_audio: None, start_frame: None, end_frame: None, images: None, videos: None, audios: None }
    }

    pub fn creatify_aurora(image: ImageReference, audio: AudioReference) -> Self {
        Self::CreatifyAurora { webhook: None, image, audio, resolution: None, guidance_scale: None, audio_guidance_scale: None }
    }

    pub fn veo31fast_generate001(prompt: String) -> Self {
        Self::Veo31FastGenerate001 { webhook: None, prompt, negative_prompt: None, seed: None, enhance_prompt: None, duration_secs: None, aspect_ratio: None, resolution: None, generate_audio: None, start_frame: None, end_frame: None, images: None }
    }

    pub fn veo31generate001(prompt: String) -> Self {
        Self::Veo31Generate001 { webhook: None, prompt, negative_prompt: None, seed: None, enhance_prompt: None, duration_secs: None, aspect_ratio: None, resolution: None, generate_audio: None, start_frame: None, end_frame: None, images: None }
    }

    pub fn bytedance_seedance_v2_with_webhook(webhook: WebhookTarget, prompt: String, aspect_ratio: Option<BytedanceSeedance2RequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2RequestResolution>) -> Self {
        Self::BytedanceSeedanceV2 { webhook: Some(webhook), prompt, aspect_ratio, duration_secs, seed, generate_audio, start_frame, end_frame, images, videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2_with_aspect_ratio(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: BytedanceSeedance2RequestAspectRatio, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2RequestResolution>) -> Self {
        Self::BytedanceSeedanceV2 { webhook, prompt, aspect_ratio: Some(aspect_ratio), duration_secs, seed, generate_audio, start_frame, end_frame, images, videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2_with_duration_secs(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2RequestAspectRatio>, duration_secs: i64, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2RequestResolution>) -> Self {
        Self::BytedanceSeedanceV2 { webhook, prompt, aspect_ratio, duration_secs: Some(duration_secs), seed, generate_audio, start_frame, end_frame, images, videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2_with_seed(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2RequestAspectRatio>, duration_secs: Option<i64>, seed: i64, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2RequestResolution>) -> Self {
        Self::BytedanceSeedanceV2 { webhook, prompt, aspect_ratio, duration_secs, seed: Some(seed), generate_audio, start_frame, end_frame, images, videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2_with_generate_audio(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2RequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: bool, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2RequestResolution>) -> Self {
        Self::BytedanceSeedanceV2 { webhook, prompt, aspect_ratio, duration_secs, seed, generate_audio: Some(generate_audio), start_frame, end_frame, images, videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2_with_start_frame(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2RequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: ImageReference, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2RequestResolution>) -> Self {
        Self::BytedanceSeedanceV2 { webhook, prompt, aspect_ratio, duration_secs, seed, generate_audio, start_frame: Some(start_frame), end_frame, images, videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2_with_end_frame(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2RequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: ImageReference, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2RequestResolution>) -> Self {
        Self::BytedanceSeedanceV2 { webhook, prompt, aspect_ratio, duration_secs, seed, generate_audio, start_frame, end_frame: Some(end_frame), images, videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2_with_images(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2RequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Vec<ImageReference>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2RequestResolution>) -> Self {
        Self::BytedanceSeedanceV2 { webhook, prompt, aspect_ratio, duration_secs, seed, generate_audio, start_frame, end_frame, images: Some(images), videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2_with_videos(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2RequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Vec<VideoReference>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2RequestResolution>) -> Self {
        Self::BytedanceSeedanceV2 { webhook, prompt, aspect_ratio, duration_secs, seed, generate_audio, start_frame, end_frame, images, videos: Some(videos), audios, resolution }
    }

    pub fn bytedance_seedance_v2_with_audios(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2RequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Vec<AudioReference>, resolution: Option<BytedanceSeedance2RequestResolution>) -> Self {
        Self::BytedanceSeedanceV2 { webhook, prompt, aspect_ratio, duration_secs, seed, generate_audio, start_frame, end_frame, images, videos, audios: Some(audios), resolution }
    }

    pub fn bytedance_seedance_v2_with_resolution(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2RequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: BytedanceSeedance2RequestResolution) -> Self {
        Self::BytedanceSeedanceV2 { webhook, prompt, aspect_ratio, duration_secs, seed, generate_audio, start_frame, end_frame, images, videos, audios, resolution: Some(resolution) }
    }

    pub fn bytedance_seedance_v2fast_with_webhook(webhook: WebhookTarget, prompt: String, aspect_ratio: Option<BytedanceSeedance2FastRequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2FastRequestResolution>) -> Self {
        Self::BytedanceSeedanceV2Fast { webhook: Some(webhook), prompt, aspect_ratio, duration_secs, seed, generate_audio, start_frame, end_frame, images, videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2fast_with_aspect_ratio(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: BytedanceSeedance2FastRequestAspectRatio, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2FastRequestResolution>) -> Self {
        Self::BytedanceSeedanceV2Fast { webhook, prompt, aspect_ratio: Some(aspect_ratio), duration_secs, seed, generate_audio, start_frame, end_frame, images, videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2fast_with_duration_secs(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2FastRequestAspectRatio>, duration_secs: i64, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2FastRequestResolution>) -> Self {
        Self::BytedanceSeedanceV2Fast { webhook, prompt, aspect_ratio, duration_secs: Some(duration_secs), seed, generate_audio, start_frame, end_frame, images, videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2fast_with_seed(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2FastRequestAspectRatio>, duration_secs: Option<i64>, seed: i64, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2FastRequestResolution>) -> Self {
        Self::BytedanceSeedanceV2Fast { webhook, prompt, aspect_ratio, duration_secs, seed: Some(seed), generate_audio, start_frame, end_frame, images, videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2fast_with_generate_audio(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2FastRequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: bool, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2FastRequestResolution>) -> Self {
        Self::BytedanceSeedanceV2Fast { webhook, prompt, aspect_ratio, duration_secs, seed, generate_audio: Some(generate_audio), start_frame, end_frame, images, videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2fast_with_start_frame(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2FastRequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: ImageReference, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2FastRequestResolution>) -> Self {
        Self::BytedanceSeedanceV2Fast { webhook, prompt, aspect_ratio, duration_secs, seed, generate_audio, start_frame: Some(start_frame), end_frame, images, videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2fast_with_end_frame(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2FastRequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: ImageReference, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2FastRequestResolution>) -> Self {
        Self::BytedanceSeedanceV2Fast { webhook, prompt, aspect_ratio, duration_secs, seed, generate_audio, start_frame, end_frame: Some(end_frame), images, videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2fast_with_images(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2FastRequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Vec<ImageReference>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2FastRequestResolution>) -> Self {
        Self::BytedanceSeedanceV2Fast { webhook, prompt, aspect_ratio, duration_secs, seed, generate_audio, start_frame, end_frame, images: Some(images), videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2fast_with_videos(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2FastRequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Vec<VideoReference>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2FastRequestResolution>) -> Self {
        Self::BytedanceSeedanceV2Fast { webhook, prompt, aspect_ratio, duration_secs, seed, generate_audio, start_frame, end_frame, images, videos: Some(videos), audios, resolution }
    }

    pub fn bytedance_seedance_v2fast_with_audios(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2FastRequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Vec<AudioReference>, resolution: Option<BytedanceSeedance2FastRequestResolution>) -> Self {
        Self::BytedanceSeedanceV2Fast { webhook, prompt, aspect_ratio, duration_secs, seed, generate_audio, start_frame, end_frame, images, videos, audios: Some(audios), resolution }
    }

    pub fn bytedance_seedance_v2fast_with_resolution(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2FastRequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: BytedanceSeedance2FastRequestResolution) -> Self {
        Self::BytedanceSeedanceV2Fast { webhook, prompt, aspect_ratio, duration_secs, seed, generate_audio, start_frame, end_frame, images, videos, audios, resolution: Some(resolution) }
    }

    pub fn bytedance_seedance_v2mini_with_webhook(webhook: WebhookTarget, prompt: String, aspect_ratio: Option<BytedanceSeedance2MiniRequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2MiniRequestResolution>) -> Self {
        Self::BytedanceSeedanceV2Mini { webhook: Some(webhook), prompt, aspect_ratio, duration_secs, seed, generate_audio, start_frame, end_frame, images, videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2mini_with_aspect_ratio(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: BytedanceSeedance2MiniRequestAspectRatio, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2MiniRequestResolution>) -> Self {
        Self::BytedanceSeedanceV2Mini { webhook, prompt, aspect_ratio: Some(aspect_ratio), duration_secs, seed, generate_audio, start_frame, end_frame, images, videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2mini_with_duration_secs(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2MiniRequestAspectRatio>, duration_secs: i64, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2MiniRequestResolution>) -> Self {
        Self::BytedanceSeedanceV2Mini { webhook, prompt, aspect_ratio, duration_secs: Some(duration_secs), seed, generate_audio, start_frame, end_frame, images, videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2mini_with_seed(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2MiniRequestAspectRatio>, duration_secs: Option<i64>, seed: i64, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2MiniRequestResolution>) -> Self {
        Self::BytedanceSeedanceV2Mini { webhook, prompt, aspect_ratio, duration_secs, seed: Some(seed), generate_audio, start_frame, end_frame, images, videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2mini_with_generate_audio(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2MiniRequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: bool, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2MiniRequestResolution>) -> Self {
        Self::BytedanceSeedanceV2Mini { webhook, prompt, aspect_ratio, duration_secs, seed, generate_audio: Some(generate_audio), start_frame, end_frame, images, videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2mini_with_start_frame(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2MiniRequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: ImageReference, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2MiniRequestResolution>) -> Self {
        Self::BytedanceSeedanceV2Mini { webhook, prompt, aspect_ratio, duration_secs, seed, generate_audio, start_frame: Some(start_frame), end_frame, images, videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2mini_with_end_frame(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2MiniRequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: ImageReference, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2MiniRequestResolution>) -> Self {
        Self::BytedanceSeedanceV2Mini { webhook, prompt, aspect_ratio, duration_secs, seed, generate_audio, start_frame, end_frame: Some(end_frame), images, videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2mini_with_images(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2MiniRequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Vec<ImageReference>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2MiniRequestResolution>) -> Self {
        Self::BytedanceSeedanceV2Mini { webhook, prompt, aspect_ratio, duration_secs, seed, generate_audio, start_frame, end_frame, images: Some(images), videos, audios, resolution }
    }

    pub fn bytedance_seedance_v2mini_with_videos(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2MiniRequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Vec<VideoReference>, audios: Option<Vec<AudioReference>>, resolution: Option<BytedanceSeedance2MiniRequestResolution>) -> Self {
        Self::BytedanceSeedanceV2Mini { webhook, prompt, aspect_ratio, duration_secs, seed, generate_audio, start_frame, end_frame, images, videos: Some(videos), audios, resolution }
    }

    pub fn bytedance_seedance_v2mini_with_audios(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2MiniRequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Vec<AudioReference>, resolution: Option<BytedanceSeedance2MiniRequestResolution>) -> Self {
        Self::BytedanceSeedanceV2Mini { webhook, prompt, aspect_ratio, duration_secs, seed, generate_audio, start_frame, end_frame, images, videos, audios: Some(audios), resolution }
    }

    pub fn bytedance_seedance_v2mini_with_resolution(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance2MiniRequestAspectRatio>, duration_secs: Option<i64>, seed: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>, resolution: BytedanceSeedance2MiniRequestResolution) -> Self {
        Self::BytedanceSeedanceV2Mini { webhook, prompt, aspect_ratio, duration_secs, seed, generate_audio, start_frame, end_frame, images, videos, audios, resolution: Some(resolution) }
    }

    pub fn bytedance_seedance_v25_with_webhook(webhook: WebhookTarget, prompt: String, aspect_ratio: Option<BytedanceSeedance25RequestAspectRatio>, resolution: Option<BytedanceSeedance25RequestResolution>, duration_secs: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>) -> Self {
        Self::BytedanceSeedanceV25 { webhook: Some(webhook), prompt, aspect_ratio, resolution, duration_secs, generate_audio, start_frame, end_frame, images, videos, audios }
    }

    pub fn bytedance_seedance_v25_with_aspect_ratio(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: BytedanceSeedance25RequestAspectRatio, resolution: Option<BytedanceSeedance25RequestResolution>, duration_secs: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>) -> Self {
        Self::BytedanceSeedanceV25 { webhook, prompt, aspect_ratio: Some(aspect_ratio), resolution, duration_secs, generate_audio, start_frame, end_frame, images, videos, audios }
    }

    pub fn bytedance_seedance_v25_with_resolution(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance25RequestAspectRatio>, resolution: BytedanceSeedance25RequestResolution, duration_secs: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>) -> Self {
        Self::BytedanceSeedanceV25 { webhook, prompt, aspect_ratio, resolution: Some(resolution), duration_secs, generate_audio, start_frame, end_frame, images, videos, audios }
    }

    pub fn bytedance_seedance_v25_with_duration_secs(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance25RequestAspectRatio>, resolution: Option<BytedanceSeedance25RequestResolution>, duration_secs: i64, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>) -> Self {
        Self::BytedanceSeedanceV25 { webhook, prompt, aspect_ratio, resolution, duration_secs: Some(duration_secs), generate_audio, start_frame, end_frame, images, videos, audios }
    }

    pub fn bytedance_seedance_v25_with_generate_audio(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance25RequestAspectRatio>, resolution: Option<BytedanceSeedance25RequestResolution>, duration_secs: Option<i64>, generate_audio: bool, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>) -> Self {
        Self::BytedanceSeedanceV25 { webhook, prompt, aspect_ratio, resolution, duration_secs, generate_audio: Some(generate_audio), start_frame, end_frame, images, videos, audios }
    }

    pub fn bytedance_seedance_v25_with_start_frame(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance25RequestAspectRatio>, resolution: Option<BytedanceSeedance25RequestResolution>, duration_secs: Option<i64>, generate_audio: Option<bool>, start_frame: ImageReference, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>) -> Self {
        Self::BytedanceSeedanceV25 { webhook, prompt, aspect_ratio, resolution, duration_secs, generate_audio, start_frame: Some(start_frame), end_frame, images, videos, audios }
    }

    pub fn bytedance_seedance_v25_with_end_frame(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance25RequestAspectRatio>, resolution: Option<BytedanceSeedance25RequestResolution>, duration_secs: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: ImageReference, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>) -> Self {
        Self::BytedanceSeedanceV25 { webhook, prompt, aspect_ratio, resolution, duration_secs, generate_audio, start_frame, end_frame: Some(end_frame), images, videos, audios }
    }

    pub fn bytedance_seedance_v25_with_images(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance25RequestAspectRatio>, resolution: Option<BytedanceSeedance25RequestResolution>, duration_secs: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Vec<ImageReference>, videos: Option<Vec<VideoReference>>, audios: Option<Vec<AudioReference>>) -> Self {
        Self::BytedanceSeedanceV25 { webhook, prompt, aspect_ratio, resolution, duration_secs, generate_audio, start_frame, end_frame, images: Some(images), videos, audios }
    }

    pub fn bytedance_seedance_v25_with_videos(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance25RequestAspectRatio>, resolution: Option<BytedanceSeedance25RequestResolution>, duration_secs: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Vec<VideoReference>, audios: Option<Vec<AudioReference>>) -> Self {
        Self::BytedanceSeedanceV25 { webhook, prompt, aspect_ratio, resolution, duration_secs, generate_audio, start_frame, end_frame, images, videos: Some(videos), audios }
    }

    pub fn bytedance_seedance_v25_with_audios(webhook: Option<WebhookTarget>, prompt: String, aspect_ratio: Option<BytedanceSeedance25RequestAspectRatio>, resolution: Option<BytedanceSeedance25RequestResolution>, duration_secs: Option<i64>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<ImageReference>>, videos: Option<Vec<VideoReference>>, audios: Vec<AudioReference>) -> Self {
        Self::BytedanceSeedanceV25 { webhook, prompt, aspect_ratio, resolution, duration_secs, generate_audio, start_frame, end_frame, images, videos, audios: Some(audios) }
    }

    pub fn creatify_aurora_with_webhook(webhook: WebhookTarget, image: ImageReference, audio: AudioReference, resolution: Option<CreatifyAuroraRequestResolution>, guidance_scale: Option<f64>, audio_guidance_scale: Option<f64>) -> Self {
        Self::CreatifyAurora { webhook: Some(webhook), image, audio, resolution, guidance_scale, audio_guidance_scale }
    }

    pub fn creatify_aurora_with_resolution(webhook: Option<WebhookTarget>, image: ImageReference, audio: AudioReference, resolution: CreatifyAuroraRequestResolution, guidance_scale: Option<f64>, audio_guidance_scale: Option<f64>) -> Self {
        Self::CreatifyAurora { webhook, image, audio, resolution: Some(resolution), guidance_scale, audio_guidance_scale }
    }

    pub fn creatify_aurora_with_guidance_scale(webhook: Option<WebhookTarget>, image: ImageReference, audio: AudioReference, resolution: Option<CreatifyAuroraRequestResolution>, guidance_scale: f64, audio_guidance_scale: Option<f64>) -> Self {
        Self::CreatifyAurora { webhook, image, audio, resolution, guidance_scale: Some(guidance_scale), audio_guidance_scale }
    }

    pub fn creatify_aurora_with_audio_guidance_scale(webhook: Option<WebhookTarget>, image: ImageReference, audio: AudioReference, resolution: Option<CreatifyAuroraRequestResolution>, guidance_scale: Option<f64>, audio_guidance_scale: f64) -> Self {
        Self::CreatifyAurora { webhook, image, audio, resolution, guidance_scale, audio_guidance_scale: Some(audio_guidance_scale) }
    }

    pub fn veo31fast_generate001_with_webhook(webhook: WebhookTarget, prompt: String, negative_prompt: Option<String>, seed: Option<i64>, enhance_prompt: Option<bool>, duration_secs: Option<i64>, aspect_ratio: Option<Veo31FastRequestAspectRatio>, resolution: Option<Veo31FastRequestResolution>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<VeoImageReference>>) -> Self {
        Self::Veo31FastGenerate001 { webhook: Some(webhook), prompt, negative_prompt, seed, enhance_prompt, duration_secs, aspect_ratio, resolution, generate_audio, start_frame, end_frame, images }
    }

    pub fn veo31fast_generate001_with_negative_prompt(webhook: Option<WebhookTarget>, prompt: String, negative_prompt: String, seed: Option<i64>, enhance_prompt: Option<bool>, duration_secs: Option<i64>, aspect_ratio: Option<Veo31FastRequestAspectRatio>, resolution: Option<Veo31FastRequestResolution>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<VeoImageReference>>) -> Self {
        Self::Veo31FastGenerate001 { webhook, prompt, negative_prompt: Some(negative_prompt), seed, enhance_prompt, duration_secs, aspect_ratio, resolution, generate_audio, start_frame, end_frame, images }
    }

    pub fn veo31fast_generate001_with_seed(webhook: Option<WebhookTarget>, prompt: String, negative_prompt: Option<String>, seed: i64, enhance_prompt: Option<bool>, duration_secs: Option<i64>, aspect_ratio: Option<Veo31FastRequestAspectRatio>, resolution: Option<Veo31FastRequestResolution>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<VeoImageReference>>) -> Self {
        Self::Veo31FastGenerate001 { webhook, prompt, negative_prompt, seed: Some(seed), enhance_prompt, duration_secs, aspect_ratio, resolution, generate_audio, start_frame, end_frame, images }
    }

    pub fn veo31fast_generate001_with_enhance_prompt(webhook: Option<WebhookTarget>, prompt: String, negative_prompt: Option<String>, seed: Option<i64>, enhance_prompt: bool, duration_secs: Option<i64>, aspect_ratio: Option<Veo31FastRequestAspectRatio>, resolution: Option<Veo31FastRequestResolution>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<VeoImageReference>>) -> Self {
        Self::Veo31FastGenerate001 { webhook, prompt, negative_prompt, seed, enhance_prompt: Some(enhance_prompt), duration_secs, aspect_ratio, resolution, generate_audio, start_frame, end_frame, images }
    }

    pub fn veo31fast_generate001_with_duration_secs(webhook: Option<WebhookTarget>, prompt: String, negative_prompt: Option<String>, seed: Option<i64>, enhance_prompt: Option<bool>, duration_secs: i64, aspect_ratio: Option<Veo31FastRequestAspectRatio>, resolution: Option<Veo31FastRequestResolution>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<VeoImageReference>>) -> Self {
        Self::Veo31FastGenerate001 { webhook, prompt, negative_prompt, seed, enhance_prompt, duration_secs: Some(duration_secs), aspect_ratio, resolution, generate_audio, start_frame, end_frame, images }
    }

    pub fn veo31fast_generate001_with_aspect_ratio(webhook: Option<WebhookTarget>, prompt: String, negative_prompt: Option<String>, seed: Option<i64>, enhance_prompt: Option<bool>, duration_secs: Option<i64>, aspect_ratio: Veo31FastRequestAspectRatio, resolution: Option<Veo31FastRequestResolution>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<VeoImageReference>>) -> Self {
        Self::Veo31FastGenerate001 { webhook, prompt, negative_prompt, seed, enhance_prompt, duration_secs, aspect_ratio: Some(aspect_ratio), resolution, generate_audio, start_frame, end_frame, images }
    }

    pub fn veo31fast_generate001_with_resolution(webhook: Option<WebhookTarget>, prompt: String, negative_prompt: Option<String>, seed: Option<i64>, enhance_prompt: Option<bool>, duration_secs: Option<i64>, aspect_ratio: Option<Veo31FastRequestAspectRatio>, resolution: Veo31FastRequestResolution, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<VeoImageReference>>) -> Self {
        Self::Veo31FastGenerate001 { webhook, prompt, negative_prompt, seed, enhance_prompt, duration_secs, aspect_ratio, resolution: Some(resolution), generate_audio, start_frame, end_frame, images }
    }

    pub fn veo31fast_generate001_with_generate_audio(webhook: Option<WebhookTarget>, prompt: String, negative_prompt: Option<String>, seed: Option<i64>, enhance_prompt: Option<bool>, duration_secs: Option<i64>, aspect_ratio: Option<Veo31FastRequestAspectRatio>, resolution: Option<Veo31FastRequestResolution>, generate_audio: bool, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<VeoImageReference>>) -> Self {
        Self::Veo31FastGenerate001 { webhook, prompt, negative_prompt, seed, enhance_prompt, duration_secs, aspect_ratio, resolution, generate_audio: Some(generate_audio), start_frame, end_frame, images }
    }

    pub fn veo31fast_generate001_with_start_frame(webhook: Option<WebhookTarget>, prompt: String, negative_prompt: Option<String>, seed: Option<i64>, enhance_prompt: Option<bool>, duration_secs: Option<i64>, aspect_ratio: Option<Veo31FastRequestAspectRatio>, resolution: Option<Veo31FastRequestResolution>, generate_audio: Option<bool>, start_frame: ImageReference, end_frame: Option<ImageReference>, images: Option<Vec<VeoImageReference>>) -> Self {
        Self::Veo31FastGenerate001 { webhook, prompt, negative_prompt, seed, enhance_prompt, duration_secs, aspect_ratio, resolution, generate_audio, start_frame: Some(start_frame), end_frame, images }
    }

    pub fn veo31fast_generate001_with_end_frame(webhook: Option<WebhookTarget>, prompt: String, negative_prompt: Option<String>, seed: Option<i64>, enhance_prompt: Option<bool>, duration_secs: Option<i64>, aspect_ratio: Option<Veo31FastRequestAspectRatio>, resolution: Option<Veo31FastRequestResolution>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: ImageReference, images: Option<Vec<VeoImageReference>>) -> Self {
        Self::Veo31FastGenerate001 { webhook, prompt, negative_prompt, seed, enhance_prompt, duration_secs, aspect_ratio, resolution, generate_audio, start_frame, end_frame: Some(end_frame), images }
    }

    pub fn veo31fast_generate001_with_images(webhook: Option<WebhookTarget>, prompt: String, negative_prompt: Option<String>, seed: Option<i64>, enhance_prompt: Option<bool>, duration_secs: Option<i64>, aspect_ratio: Option<Veo31FastRequestAspectRatio>, resolution: Option<Veo31FastRequestResolution>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Vec<VeoImageReference>) -> Self {
        Self::Veo31FastGenerate001 { webhook, prompt, negative_prompt, seed, enhance_prompt, duration_secs, aspect_ratio, resolution, generate_audio, start_frame, end_frame, images: Some(images) }
    }

    pub fn veo31generate001_with_webhook(webhook: WebhookTarget, prompt: String, negative_prompt: Option<String>, seed: Option<i64>, enhance_prompt: Option<bool>, duration_secs: Option<i64>, aspect_ratio: Option<Veo31RequestAspectRatio>, resolution: Option<Veo31RequestResolution>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<VeoImageReference>>) -> Self {
        Self::Veo31Generate001 { webhook: Some(webhook), prompt, negative_prompt, seed, enhance_prompt, duration_secs, aspect_ratio, resolution, generate_audio, start_frame, end_frame, images }
    }

    pub fn veo31generate001_with_negative_prompt(webhook: Option<WebhookTarget>, prompt: String, negative_prompt: String, seed: Option<i64>, enhance_prompt: Option<bool>, duration_secs: Option<i64>, aspect_ratio: Option<Veo31RequestAspectRatio>, resolution: Option<Veo31RequestResolution>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<VeoImageReference>>) -> Self {
        Self::Veo31Generate001 { webhook, prompt, negative_prompt: Some(negative_prompt), seed, enhance_prompt, duration_secs, aspect_ratio, resolution, generate_audio, start_frame, end_frame, images }
    }

    pub fn veo31generate001_with_seed(webhook: Option<WebhookTarget>, prompt: String, negative_prompt: Option<String>, seed: i64, enhance_prompt: Option<bool>, duration_secs: Option<i64>, aspect_ratio: Option<Veo31RequestAspectRatio>, resolution: Option<Veo31RequestResolution>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<VeoImageReference>>) -> Self {
        Self::Veo31Generate001 { webhook, prompt, negative_prompt, seed: Some(seed), enhance_prompt, duration_secs, aspect_ratio, resolution, generate_audio, start_frame, end_frame, images }
    }

    pub fn veo31generate001_with_enhance_prompt(webhook: Option<WebhookTarget>, prompt: String, negative_prompt: Option<String>, seed: Option<i64>, enhance_prompt: bool, duration_secs: Option<i64>, aspect_ratio: Option<Veo31RequestAspectRatio>, resolution: Option<Veo31RequestResolution>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<VeoImageReference>>) -> Self {
        Self::Veo31Generate001 { webhook, prompt, negative_prompt, seed, enhance_prompt: Some(enhance_prompt), duration_secs, aspect_ratio, resolution, generate_audio, start_frame, end_frame, images }
    }

    pub fn veo31generate001_with_duration_secs(webhook: Option<WebhookTarget>, prompt: String, negative_prompt: Option<String>, seed: Option<i64>, enhance_prompt: Option<bool>, duration_secs: i64, aspect_ratio: Option<Veo31RequestAspectRatio>, resolution: Option<Veo31RequestResolution>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<VeoImageReference>>) -> Self {
        Self::Veo31Generate001 { webhook, prompt, negative_prompt, seed, enhance_prompt, duration_secs: Some(duration_secs), aspect_ratio, resolution, generate_audio, start_frame, end_frame, images }
    }

    pub fn veo31generate001_with_aspect_ratio(webhook: Option<WebhookTarget>, prompt: String, negative_prompt: Option<String>, seed: Option<i64>, enhance_prompt: Option<bool>, duration_secs: Option<i64>, aspect_ratio: Veo31RequestAspectRatio, resolution: Option<Veo31RequestResolution>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<VeoImageReference>>) -> Self {
        Self::Veo31Generate001 { webhook, prompt, negative_prompt, seed, enhance_prompt, duration_secs, aspect_ratio: Some(aspect_ratio), resolution, generate_audio, start_frame, end_frame, images }
    }

    pub fn veo31generate001_with_resolution(webhook: Option<WebhookTarget>, prompt: String, negative_prompt: Option<String>, seed: Option<i64>, enhance_prompt: Option<bool>, duration_secs: Option<i64>, aspect_ratio: Option<Veo31RequestAspectRatio>, resolution: Veo31RequestResolution, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<VeoImageReference>>) -> Self {
        Self::Veo31Generate001 { webhook, prompt, negative_prompt, seed, enhance_prompt, duration_secs, aspect_ratio, resolution: Some(resolution), generate_audio, start_frame, end_frame, images }
    }

    pub fn veo31generate001_with_generate_audio(webhook: Option<WebhookTarget>, prompt: String, negative_prompt: Option<String>, seed: Option<i64>, enhance_prompt: Option<bool>, duration_secs: Option<i64>, aspect_ratio: Option<Veo31RequestAspectRatio>, resolution: Option<Veo31RequestResolution>, generate_audio: bool, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Option<Vec<VeoImageReference>>) -> Self {
        Self::Veo31Generate001 { webhook, prompt, negative_prompt, seed, enhance_prompt, duration_secs, aspect_ratio, resolution, generate_audio: Some(generate_audio), start_frame, end_frame, images }
    }

    pub fn veo31generate001_with_start_frame(webhook: Option<WebhookTarget>, prompt: String, negative_prompt: Option<String>, seed: Option<i64>, enhance_prompt: Option<bool>, duration_secs: Option<i64>, aspect_ratio: Option<Veo31RequestAspectRatio>, resolution: Option<Veo31RequestResolution>, generate_audio: Option<bool>, start_frame: ImageReference, end_frame: Option<ImageReference>, images: Option<Vec<VeoImageReference>>) -> Self {
        Self::Veo31Generate001 { webhook, prompt, negative_prompt, seed, enhance_prompt, duration_secs, aspect_ratio, resolution, generate_audio, start_frame: Some(start_frame), end_frame, images }
    }

    pub fn veo31generate001_with_end_frame(webhook: Option<WebhookTarget>, prompt: String, negative_prompt: Option<String>, seed: Option<i64>, enhance_prompt: Option<bool>, duration_secs: Option<i64>, aspect_ratio: Option<Veo31RequestAspectRatio>, resolution: Option<Veo31RequestResolution>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: ImageReference, images: Option<Vec<VeoImageReference>>) -> Self {
        Self::Veo31Generate001 { webhook, prompt, negative_prompt, seed, enhance_prompt, duration_secs, aspect_ratio, resolution, generate_audio, start_frame, end_frame: Some(end_frame), images }
    }

    pub fn veo31generate001_with_images(webhook: Option<WebhookTarget>, prompt: String, negative_prompt: Option<String>, seed: Option<i64>, enhance_prompt: Option<bool>, duration_secs: Option<i64>, aspect_ratio: Option<Veo31RequestAspectRatio>, resolution: Option<Veo31RequestResolution>, generate_audio: Option<bool>, start_frame: Option<ImageReference>, end_frame: Option<ImageReference>, images: Vec<VeoImageReference>) -> Self {
        Self::Veo31Generate001 { webhook, prompt, negative_prompt, seed, enhance_prompt, duration_secs, aspect_ratio, resolution, generate_audio, start_frame, end_frame, images: Some(images) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
