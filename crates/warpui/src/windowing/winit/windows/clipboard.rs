use std::borrow::Cow;
use std::io::Cursor;
use std::ops::Not;
use std::slice;

use arboard::{self, Clipboard as WindowsClipboardInner};
use windows::Win32::{
    Foundation::HGLOBAL,
    System::{
        DataExchange::{
            CloseClipboard, GetClipboardData, IsClipboardFormatAvailable, OpenClipboard,
        },
        Memory::{GlobalLock, GlobalSize, GlobalUnlock},
    },
};

use crate::Clipboard;
use crate::clipboard::ClipboardContent;

pub struct WindowsClipboard {
    inner: WindowsClipboardInner,
}

impl WindowsClipboard {
    pub fn new() -> Result<Self, arboard::Error> {
        Ok(Self {
            inner: WindowsClipboardInner::new()?,
        })
    }
}

impl Clipboard for WindowsClipboard {
    fn write(&mut self, contents: ClipboardContent) {
        if let Some(image) = contents.images.as_ref().and_then(|images| images.first()) {
            let decoded = match image::load_from_memory(&image.data) {
                Ok(image) => image.into_rgba8(),
                Err(error) => {
                    log::warn!("Unable to decode clipboard image: {error:?}");
                    return;
                }
            };
            let image = arboard::ImageData {
                width: decoded.width() as usize,
                height: decoded.height() as usize,
                bytes: Cow::Owned(decoded.into_raw()),
            };
            if let Err(error) = self.inner.set().image(image) {
                log::warn!("Unable to set clipboard image: {error:?}");
            }
            return;
        }

        let set_result = if let Some(html) = &contents.html {
            self.inner.set().html(html, Some(&contents.plain_text))
        } else {
            self.inner.set().text(&contents.plain_text)
        };

        if let Err(err) = set_result {
            if contents.html.is_some() {
                log::warn!("Unable to set clipboard HTML: {err:?}");
            } else {
                log::warn!("Unable to set clipboard text: {err:?}");
            }
        }
    }

    fn read(&mut self) -> ClipboardContent {
        let mut content = ClipboardContent {
            plain_text: self.inner.get().text().unwrap_or_default(),
            ..Default::default()
        };

        // Try to get HTML content
        if let Ok(html) = self.inner.get().html() {
            content.html = html.is_empty().not().then_some(html);
        }

        // Some environments provide HTML but do not provide a plaintext representation.
        // If that happens, derive a best-effort plaintext fallback from the HTML.
        if content.plain_text.trim().is_empty()
            && let Some(html) = content.html.as_ref()
        {
            let derived = crate::clipboard_utils::strip_html_to_plain_text(html);
            if !derived.trim().is_empty() {
                content.plain_text = derived;
            }
        }

        // Get file paths.
        content.paths = self.inner.get().file_list().ok().map(|list| {
            list.into_iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect()
        });

        // Try to get image content from clipboard
        content.images = crate::clipboard_utils::read_images_from_clipboard(
            &mut self.inner,
            &content.html,
            &content.plain_text,
        );
        if content.images.is_none() {
            content.images = read_legacy_dib_image();
        }

        content
    }
}

fn read_legacy_dib_image() -> Option<Vec<crate::clipboard::ImageData>> {
    unsafe {
        OpenClipboard(None).ok()?;
        let _clipboard_guard = ClipboardGuard;
        IsClipboardFormatAvailable(8).ok()?;

        let handle = GetClipboardData(8).ok()?;
        let global = HGLOBAL(handle.0);
        let size = GlobalSize(global);
        let data = GlobalLock(global).cast::<u8>();
        if data.is_null() {
            return None;
        }

        let image = (size >= 40).then(|| {
            let dib = slice::from_raw_parts(data, size);
            dib_to_png(dib)
        });
        let _ = GlobalUnlock(global);
        image.flatten().map(|data| {
            vec![crate::clipboard::ImageData {
                data,
                mime_type: "image/png".to_string(),
                filename: None,
            }]
        })
    }
}

struct ClipboardGuard;

impl Drop for ClipboardGuard {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseClipboard();
        }
    }
}

fn dib_to_png(dib: &[u8]) -> Option<Vec<u8>> {
    let header_size = u32::from_le_bytes(dib.get(..4)?.try_into().ok()?) as usize;
    if header_size < 40 || header_size > dib.len() {
        return None;
    }

    let bits_per_pixel = u16::from_le_bytes(dib.get(14..16)?.try_into().ok()?);
    let compression = u32::from_le_bytes(dib.get(16..20)?.try_into().ok()?);
    let colors_used = u32::from_le_bytes(dib.get(32..36)?.try_into().ok()?) as usize;
    let palette_entries = if colors_used > 0 {
        colors_used
    } else if bits_per_pixel <= 8 {
        1usize.checked_shl(bits_per_pixel.into())?
    } else {
        0
    };
    let external_masks = if header_size == 40 {
        match compression {
            3 => 12,
            6 => 16,
            _ => 0,
        }
    } else {
        0
    };
    let pixel_offset = 14usize
        .checked_add(header_size)?
        .checked_add(external_masks)?
        .checked_add(palette_entries.checked_mul(4)?)?;
    let file_size = dib.len().checked_add(14)?;
    if file_size > u32::MAX as usize || pixel_offset > file_size {
        return None;
    }

    let mut bitmap = Vec::with_capacity(file_size);
    bitmap.extend_from_slice(b"BM");
    bitmap.extend_from_slice(&(file_size as u32).to_le_bytes());
    bitmap.extend_from_slice(&[0; 4]);
    bitmap.extend_from_slice(&(pixel_offset as u32).to_le_bytes());
    bitmap.extend_from_slice(dib);

    let decoded = image::load_from_memory(&bitmap).ok()?;
    let mut png = Cursor::new(Vec::new());
    decoded.write_to(&mut png, image::ImageFormat::Png).ok()?;
    Some(png.into_inner())
}

#[cfg(test)]
#[path = "clipboard_tests.rs"]
mod tests;
