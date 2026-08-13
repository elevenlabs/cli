pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "model_id")]
#[non_exhaustive]
pub enum ImageGenerationRequest {
        #[serde(rename = "bytedance-seedream-5-lite")]
        #[non_exhaustive]
        BytedanceSeedream5Lite {
            #[serde(skip_serializing_if = "Option::is_none")]
            webhook: Option<WebhookTarget>,
            #[serde(default)]
            prompt: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            images: Option<Vec<ImageReference>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            aspect_ratio: Option<BytedanceSeedream5LiteRequestAspectRatio>,
            #[serde(skip_serializing_if = "Option::is_none")]
            seed: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            resolution: Option<BytedanceSeedream5LiteRequestResolution>,
        },

        #[serde(rename = "bytedance-seedream-5-pro")]
        #[non_exhaustive]
        BytedanceSeedream5Pro {
            #[serde(skip_serializing_if = "Option::is_none")]
            webhook: Option<WebhookTarget>,
            #[serde(default)]
            prompt: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            images: Option<Vec<ImageReference>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            aspect_ratio: Option<BytedanceSeedream5ProRequestAspectRatio>,
            #[serde(skip_serializing_if = "Option::is_none")]
            seed: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            resolution: Option<BytedanceSeedream5ProRequestResolution>,
        },

        #[serde(rename = "gemini-2.5-flash-image")]
        #[non_exhaustive]
        Gemini25FlashImage {
            #[serde(skip_serializing_if = "Option::is_none")]
            webhook: Option<WebhookTarget>,
            #[serde(default)]
            prompt: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            images: Option<Vec<ImageReference>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            aspect_ratio: Option<Gemini25FlashImageRequestAspectRatio>,
        },

        #[serde(rename = "gemini-3-pro-image")]
        #[non_exhaustive]
        Gemini3ProImage {
            #[serde(skip_serializing_if = "Option::is_none")]
            webhook: Option<WebhookTarget>,
            #[serde(default)]
            prompt: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            images: Option<Vec<ImageReference>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            aspect_ratio: Option<Gemini3ProImageRequestAspectRatio>,
            #[serde(skip_serializing_if = "Option::is_none")]
            resolution: Option<Gemini3ProImageRequestResolution>,
        },

        #[serde(rename = "gemini-3.1-flash-image")]
        #[non_exhaustive]
        Gemini31FlashImage {
            #[serde(skip_serializing_if = "Option::is_none")]
            webhook: Option<WebhookTarget>,
            #[serde(default)]
            prompt: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            images: Option<Vec<ImageReference>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            aspect_ratio: Option<Gemini31FlashImageRequestAspectRatio>,
            #[serde(skip_serializing_if = "Option::is_none")]
            resolution: Option<Gemini31FlashImageRequestResolution>,
        },

        #[serde(rename = "gemini-3.1-flash-lite-image")]
        #[non_exhaustive]
        Gemini31FlashLiteImage {
            #[serde(skip_serializing_if = "Option::is_none")]
            webhook: Option<WebhookTarget>,
            #[serde(default)]
            prompt: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            images: Option<Vec<ImageReference>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            aspect_ratio: Option<Gemini31FlashLiteImageRequestAspectRatio>,
            #[serde(skip_serializing_if = "Option::is_none")]
            resolution: Option<String>,
        },

        #[serde(rename = "gpt-image-1")]
        #[non_exhaustive]
        GptImage1 {
            #[serde(skip_serializing_if = "Option::is_none")]
            webhook: Option<WebhookTarget>,
            #[serde(default)]
            prompt: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            images: Option<Vec<ImageReference>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            mask: Option<ImageReference>,
            #[serde(skip_serializing_if = "Option::is_none")]
            quality: Option<GptImage1RequestQuality>,
            #[serde(skip_serializing_if = "Option::is_none")]
            background: Option<GptImage1RequestBackground>,
            #[serde(skip_serializing_if = "Option::is_none")]
            aspect_ratio: Option<GptImage1RequestAspectRatio>,
        },

        #[serde(rename = "gpt-image-1.5")]
        #[non_exhaustive]
        GptImage15 {
            #[serde(skip_serializing_if = "Option::is_none")]
            webhook: Option<WebhookTarget>,
            #[serde(default)]
            prompt: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            images: Option<Vec<ImageReference>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            mask: Option<ImageReference>,
            #[serde(skip_serializing_if = "Option::is_none")]
            quality: Option<GptImage15RequestQuality>,
            #[serde(skip_serializing_if = "Option::is_none")]
            background: Option<GptImage15RequestBackground>,
            #[serde(skip_serializing_if = "Option::is_none")]
            aspect_ratio: Option<GptImage15RequestAspectRatio>,
        },

        #[serde(rename = "gpt-image-2")]
        #[non_exhaustive]
        GptImage2 {
            #[serde(skip_serializing_if = "Option::is_none")]
            webhook: Option<WebhookTarget>,
            #[serde(default)]
            prompt: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            images: Option<Vec<ImageReference>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            mask: Option<ImageReference>,
            #[serde(skip_serializing_if = "Option::is_none")]
            quality: Option<GptImage2RequestQuality>,
            #[serde(skip_serializing_if = "Option::is_none")]
            aspect_ratio: Option<GptImage2RequestAspectRatio>,
            #[serde(skip_serializing_if = "Option::is_none")]
            resolution: Option<GptImage2RequestResolution>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ImageGenerationRequest {
    pub fn bytedance_seedream5lite(prompt: String) -> Self {
        Self::BytedanceSeedream5Lite { webhook: None, prompt, images: None, aspect_ratio: None, seed: None, resolution: None }
    }

    pub fn bytedance_seedream5pro(prompt: String) -> Self {
        Self::BytedanceSeedream5Pro { webhook: None, prompt, images: None, aspect_ratio: None, seed: None, resolution: None }
    }

    pub fn gemini25flash_image(prompt: String) -> Self {
        Self::Gemini25FlashImage { webhook: None, prompt, images: None, aspect_ratio: None }
    }

    pub fn gemini3pro_image(prompt: String) -> Self {
        Self::Gemini3ProImage { webhook: None, prompt, images: None, aspect_ratio: None, resolution: None }
    }

    pub fn gemini31flash_image(prompt: String) -> Self {
        Self::Gemini31FlashImage { webhook: None, prompt, images: None, aspect_ratio: None, resolution: None }
    }

    pub fn gemini31flash_lite_image(prompt: String) -> Self {
        Self::Gemini31FlashLiteImage { webhook: None, prompt, images: None, aspect_ratio: None, resolution: None }
    }

    pub fn gpt_image1(prompt: String) -> Self {
        Self::GptImage1 { webhook: None, prompt, images: None, mask: None, quality: None, background: None, aspect_ratio: None }
    }

    pub fn gpt_image15(prompt: String) -> Self {
        Self::GptImage15 { webhook: None, prompt, images: None, mask: None, quality: None, background: None, aspect_ratio: None }
    }

    pub fn gpt_image2(prompt: String) -> Self {
        Self::GptImage2 { webhook: None, prompt, images: None, mask: None, quality: None, aspect_ratio: None, resolution: None }
    }

    pub fn bytedance_seedream5lite_with_webhook(webhook: WebhookTarget, prompt: String, images: Option<Vec<ImageReference>>, aspect_ratio: Option<BytedanceSeedream5LiteRequestAspectRatio>, seed: Option<i64>, resolution: Option<BytedanceSeedream5LiteRequestResolution>) -> Self {
        Self::BytedanceSeedream5Lite { webhook: Some(webhook), prompt, images, aspect_ratio, seed, resolution }
    }

    pub fn bytedance_seedream5lite_with_images(webhook: Option<WebhookTarget>, prompt: String, images: Vec<ImageReference>, aspect_ratio: Option<BytedanceSeedream5LiteRequestAspectRatio>, seed: Option<i64>, resolution: Option<BytedanceSeedream5LiteRequestResolution>) -> Self {
        Self::BytedanceSeedream5Lite { webhook, prompt, images: Some(images), aspect_ratio, seed, resolution }
    }

    pub fn bytedance_seedream5lite_with_aspect_ratio(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, aspect_ratio: BytedanceSeedream5LiteRequestAspectRatio, seed: Option<i64>, resolution: Option<BytedanceSeedream5LiteRequestResolution>) -> Self {
        Self::BytedanceSeedream5Lite { webhook, prompt, images, aspect_ratio: Some(aspect_ratio), seed, resolution }
    }

    pub fn bytedance_seedream5lite_with_seed(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, aspect_ratio: Option<BytedanceSeedream5LiteRequestAspectRatio>, seed: i64, resolution: Option<BytedanceSeedream5LiteRequestResolution>) -> Self {
        Self::BytedanceSeedream5Lite { webhook, prompt, images, aspect_ratio, seed: Some(seed), resolution }
    }

    pub fn bytedance_seedream5lite_with_resolution(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, aspect_ratio: Option<BytedanceSeedream5LiteRequestAspectRatio>, seed: Option<i64>, resolution: BytedanceSeedream5LiteRequestResolution) -> Self {
        Self::BytedanceSeedream5Lite { webhook, prompt, images, aspect_ratio, seed, resolution: Some(resolution) }
    }

    pub fn bytedance_seedream5pro_with_webhook(webhook: WebhookTarget, prompt: String, images: Option<Vec<ImageReference>>, aspect_ratio: Option<BytedanceSeedream5ProRequestAspectRatio>, seed: Option<i64>, resolution: Option<BytedanceSeedream5ProRequestResolution>) -> Self {
        Self::BytedanceSeedream5Pro { webhook: Some(webhook), prompt, images, aspect_ratio, seed, resolution }
    }

    pub fn bytedance_seedream5pro_with_images(webhook: Option<WebhookTarget>, prompt: String, images: Vec<ImageReference>, aspect_ratio: Option<BytedanceSeedream5ProRequestAspectRatio>, seed: Option<i64>, resolution: Option<BytedanceSeedream5ProRequestResolution>) -> Self {
        Self::BytedanceSeedream5Pro { webhook, prompt, images: Some(images), aspect_ratio, seed, resolution }
    }

    pub fn bytedance_seedream5pro_with_aspect_ratio(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, aspect_ratio: BytedanceSeedream5ProRequestAspectRatio, seed: Option<i64>, resolution: Option<BytedanceSeedream5ProRequestResolution>) -> Self {
        Self::BytedanceSeedream5Pro { webhook, prompt, images, aspect_ratio: Some(aspect_ratio), seed, resolution }
    }

    pub fn bytedance_seedream5pro_with_seed(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, aspect_ratio: Option<BytedanceSeedream5ProRequestAspectRatio>, seed: i64, resolution: Option<BytedanceSeedream5ProRequestResolution>) -> Self {
        Self::BytedanceSeedream5Pro { webhook, prompt, images, aspect_ratio, seed: Some(seed), resolution }
    }

    pub fn bytedance_seedream5pro_with_resolution(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, aspect_ratio: Option<BytedanceSeedream5ProRequestAspectRatio>, seed: Option<i64>, resolution: BytedanceSeedream5ProRequestResolution) -> Self {
        Self::BytedanceSeedream5Pro { webhook, prompt, images, aspect_ratio, seed, resolution: Some(resolution) }
    }

    pub fn gemini25flash_image_with_webhook(webhook: WebhookTarget, prompt: String, images: Option<Vec<ImageReference>>, aspect_ratio: Option<Gemini25FlashImageRequestAspectRatio>) -> Self {
        Self::Gemini25FlashImage { webhook: Some(webhook), prompt, images, aspect_ratio }
    }

    pub fn gemini25flash_image_with_images(webhook: Option<WebhookTarget>, prompt: String, images: Vec<ImageReference>, aspect_ratio: Option<Gemini25FlashImageRequestAspectRatio>) -> Self {
        Self::Gemini25FlashImage { webhook, prompt, images: Some(images), aspect_ratio }
    }

    pub fn gemini25flash_image_with_aspect_ratio(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, aspect_ratio: Gemini25FlashImageRequestAspectRatio) -> Self {
        Self::Gemini25FlashImage { webhook, prompt, images, aspect_ratio: Some(aspect_ratio) }
    }

    pub fn gemini3pro_image_with_webhook(webhook: WebhookTarget, prompt: String, images: Option<Vec<ImageReference>>, aspect_ratio: Option<Gemini3ProImageRequestAspectRatio>, resolution: Option<Gemini3ProImageRequestResolution>) -> Self {
        Self::Gemini3ProImage { webhook: Some(webhook), prompt, images, aspect_ratio, resolution }
    }

    pub fn gemini3pro_image_with_images(webhook: Option<WebhookTarget>, prompt: String, images: Vec<ImageReference>, aspect_ratio: Option<Gemini3ProImageRequestAspectRatio>, resolution: Option<Gemini3ProImageRequestResolution>) -> Self {
        Self::Gemini3ProImage { webhook, prompt, images: Some(images), aspect_ratio, resolution }
    }

    pub fn gemini3pro_image_with_aspect_ratio(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, aspect_ratio: Gemini3ProImageRequestAspectRatio, resolution: Option<Gemini3ProImageRequestResolution>) -> Self {
        Self::Gemini3ProImage { webhook, prompt, images, aspect_ratio: Some(aspect_ratio), resolution }
    }

    pub fn gemini3pro_image_with_resolution(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, aspect_ratio: Option<Gemini3ProImageRequestAspectRatio>, resolution: Gemini3ProImageRequestResolution) -> Self {
        Self::Gemini3ProImage { webhook, prompt, images, aspect_ratio, resolution: Some(resolution) }
    }

    pub fn gemini31flash_image_with_webhook(webhook: WebhookTarget, prompt: String, images: Option<Vec<ImageReference>>, aspect_ratio: Option<Gemini31FlashImageRequestAspectRatio>, resolution: Option<Gemini31FlashImageRequestResolution>) -> Self {
        Self::Gemini31FlashImage { webhook: Some(webhook), prompt, images, aspect_ratio, resolution }
    }

    pub fn gemini31flash_image_with_images(webhook: Option<WebhookTarget>, prompt: String, images: Vec<ImageReference>, aspect_ratio: Option<Gemini31FlashImageRequestAspectRatio>, resolution: Option<Gemini31FlashImageRequestResolution>) -> Self {
        Self::Gemini31FlashImage { webhook, prompt, images: Some(images), aspect_ratio, resolution }
    }

    pub fn gemini31flash_image_with_aspect_ratio(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, aspect_ratio: Gemini31FlashImageRequestAspectRatio, resolution: Option<Gemini31FlashImageRequestResolution>) -> Self {
        Self::Gemini31FlashImage { webhook, prompt, images, aspect_ratio: Some(aspect_ratio), resolution }
    }

    pub fn gemini31flash_image_with_resolution(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, aspect_ratio: Option<Gemini31FlashImageRequestAspectRatio>, resolution: Gemini31FlashImageRequestResolution) -> Self {
        Self::Gemini31FlashImage { webhook, prompt, images, aspect_ratio, resolution: Some(resolution) }
    }

    pub fn gemini31flash_lite_image_with_webhook(webhook: WebhookTarget, prompt: String, images: Option<Vec<ImageReference>>, aspect_ratio: Option<Gemini31FlashLiteImageRequestAspectRatio>, resolution: Option<String>) -> Self {
        Self::Gemini31FlashLiteImage { webhook: Some(webhook), prompt, images, aspect_ratio, resolution }
    }

    pub fn gemini31flash_lite_image_with_images(webhook: Option<WebhookTarget>, prompt: String, images: Vec<ImageReference>, aspect_ratio: Option<Gemini31FlashLiteImageRequestAspectRatio>, resolution: Option<String>) -> Self {
        Self::Gemini31FlashLiteImage { webhook, prompt, images: Some(images), aspect_ratio, resolution }
    }

    pub fn gemini31flash_lite_image_with_aspect_ratio(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, aspect_ratio: Gemini31FlashLiteImageRequestAspectRatio, resolution: Option<String>) -> Self {
        Self::Gemini31FlashLiteImage { webhook, prompt, images, aspect_ratio: Some(aspect_ratio), resolution }
    }

    pub fn gemini31flash_lite_image_with_resolution(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, aspect_ratio: Option<Gemini31FlashLiteImageRequestAspectRatio>, resolution: String) -> Self {
        Self::Gemini31FlashLiteImage { webhook, prompt, images, aspect_ratio, resolution: Some(resolution) }
    }

    pub fn gpt_image1_with_webhook(webhook: WebhookTarget, prompt: String, images: Option<Vec<ImageReference>>, mask: Option<ImageReference>, quality: Option<GptImage1RequestQuality>, background: Option<GptImage1RequestBackground>, aspect_ratio: Option<GptImage1RequestAspectRatio>) -> Self {
        Self::GptImage1 { webhook: Some(webhook), prompt, images, mask, quality, background, aspect_ratio }
    }

    pub fn gpt_image1_with_images(webhook: Option<WebhookTarget>, prompt: String, images: Vec<ImageReference>, mask: Option<ImageReference>, quality: Option<GptImage1RequestQuality>, background: Option<GptImage1RequestBackground>, aspect_ratio: Option<GptImage1RequestAspectRatio>) -> Self {
        Self::GptImage1 { webhook, prompt, images: Some(images), mask, quality, background, aspect_ratio }
    }

    pub fn gpt_image1_with_mask(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, mask: ImageReference, quality: Option<GptImage1RequestQuality>, background: Option<GptImage1RequestBackground>, aspect_ratio: Option<GptImage1RequestAspectRatio>) -> Self {
        Self::GptImage1 { webhook, prompt, images, mask: Some(mask), quality, background, aspect_ratio }
    }

    pub fn gpt_image1_with_quality(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, mask: Option<ImageReference>, quality: GptImage1RequestQuality, background: Option<GptImage1RequestBackground>, aspect_ratio: Option<GptImage1RequestAspectRatio>) -> Self {
        Self::GptImage1 { webhook, prompt, images, mask, quality: Some(quality), background, aspect_ratio }
    }

    pub fn gpt_image1_with_background(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, mask: Option<ImageReference>, quality: Option<GptImage1RequestQuality>, background: GptImage1RequestBackground, aspect_ratio: Option<GptImage1RequestAspectRatio>) -> Self {
        Self::GptImage1 { webhook, prompt, images, mask, quality, background: Some(background), aspect_ratio }
    }

    pub fn gpt_image1_with_aspect_ratio(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, mask: Option<ImageReference>, quality: Option<GptImage1RequestQuality>, background: Option<GptImage1RequestBackground>, aspect_ratio: GptImage1RequestAspectRatio) -> Self {
        Self::GptImage1 { webhook, prompt, images, mask, quality, background, aspect_ratio: Some(aspect_ratio) }
    }

    pub fn gpt_image15_with_webhook(webhook: WebhookTarget, prompt: String, images: Option<Vec<ImageReference>>, mask: Option<ImageReference>, quality: Option<GptImage15RequestQuality>, background: Option<GptImage15RequestBackground>, aspect_ratio: Option<GptImage15RequestAspectRatio>) -> Self {
        Self::GptImage15 { webhook: Some(webhook), prompt, images, mask, quality, background, aspect_ratio }
    }

    pub fn gpt_image15_with_images(webhook: Option<WebhookTarget>, prompt: String, images: Vec<ImageReference>, mask: Option<ImageReference>, quality: Option<GptImage15RequestQuality>, background: Option<GptImage15RequestBackground>, aspect_ratio: Option<GptImage15RequestAspectRatio>) -> Self {
        Self::GptImage15 { webhook, prompt, images: Some(images), mask, quality, background, aspect_ratio }
    }

    pub fn gpt_image15_with_mask(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, mask: ImageReference, quality: Option<GptImage15RequestQuality>, background: Option<GptImage15RequestBackground>, aspect_ratio: Option<GptImage15RequestAspectRatio>) -> Self {
        Self::GptImage15 { webhook, prompt, images, mask: Some(mask), quality, background, aspect_ratio }
    }

    pub fn gpt_image15_with_quality(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, mask: Option<ImageReference>, quality: GptImage15RequestQuality, background: Option<GptImage15RequestBackground>, aspect_ratio: Option<GptImage15RequestAspectRatio>) -> Self {
        Self::GptImage15 { webhook, prompt, images, mask, quality: Some(quality), background, aspect_ratio }
    }

    pub fn gpt_image15_with_background(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, mask: Option<ImageReference>, quality: Option<GptImage15RequestQuality>, background: GptImage15RequestBackground, aspect_ratio: Option<GptImage15RequestAspectRatio>) -> Self {
        Self::GptImage15 { webhook, prompt, images, mask, quality, background: Some(background), aspect_ratio }
    }

    pub fn gpt_image15_with_aspect_ratio(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, mask: Option<ImageReference>, quality: Option<GptImage15RequestQuality>, background: Option<GptImage15RequestBackground>, aspect_ratio: GptImage15RequestAspectRatio) -> Self {
        Self::GptImage15 { webhook, prompt, images, mask, quality, background, aspect_ratio: Some(aspect_ratio) }
    }

    pub fn gpt_image2_with_webhook(webhook: WebhookTarget, prompt: String, images: Option<Vec<ImageReference>>, mask: Option<ImageReference>, quality: Option<GptImage2RequestQuality>, aspect_ratio: Option<GptImage2RequestAspectRatio>, resolution: Option<GptImage2RequestResolution>) -> Self {
        Self::GptImage2 { webhook: Some(webhook), prompt, images, mask, quality, aspect_ratio, resolution }
    }

    pub fn gpt_image2_with_images(webhook: Option<WebhookTarget>, prompt: String, images: Vec<ImageReference>, mask: Option<ImageReference>, quality: Option<GptImage2RequestQuality>, aspect_ratio: Option<GptImage2RequestAspectRatio>, resolution: Option<GptImage2RequestResolution>) -> Self {
        Self::GptImage2 { webhook, prompt, images: Some(images), mask, quality, aspect_ratio, resolution }
    }

    pub fn gpt_image2_with_mask(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, mask: ImageReference, quality: Option<GptImage2RequestQuality>, aspect_ratio: Option<GptImage2RequestAspectRatio>, resolution: Option<GptImage2RequestResolution>) -> Self {
        Self::GptImage2 { webhook, prompt, images, mask: Some(mask), quality, aspect_ratio, resolution }
    }

    pub fn gpt_image2_with_quality(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, mask: Option<ImageReference>, quality: GptImage2RequestQuality, aspect_ratio: Option<GptImage2RequestAspectRatio>, resolution: Option<GptImage2RequestResolution>) -> Self {
        Self::GptImage2 { webhook, prompt, images, mask, quality: Some(quality), aspect_ratio, resolution }
    }

    pub fn gpt_image2_with_aspect_ratio(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, mask: Option<ImageReference>, quality: Option<GptImage2RequestQuality>, aspect_ratio: GptImage2RequestAspectRatio, resolution: Option<GptImage2RequestResolution>) -> Self {
        Self::GptImage2 { webhook, prompt, images, mask, quality, aspect_ratio: Some(aspect_ratio), resolution }
    }

    pub fn gpt_image2_with_resolution(webhook: Option<WebhookTarget>, prompt: String, images: Option<Vec<ImageReference>>, mask: Option<ImageReference>, quality: Option<GptImage2RequestQuality>, aspect_ratio: Option<GptImage2RequestAspectRatio>, resolution: GptImage2RequestResolution) -> Self {
        Self::GptImage2 { webhook, prompt, images, mask, quality, aspect_ratio, resolution: Some(resolution) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
