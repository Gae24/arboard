use std::{borrow::Cow, path::PathBuf};

use jni::{
	objects::{JObject, JString},
	AttachGuard, JavaVM,
};

use crate::{Error, ImageData};

impl From<jni::errors::Error> for Error {
	fn from(error: jni::errors::Error) -> Self {
		Error::Unknown { description: error.to_string() }
	}
}

pub(crate) struct Clipboard(());

impl Clipboard {
	pub(crate) fn new() -> Result<Self, Error> {
		Ok(Self(()))
	}

	fn with_android_context(
		callback: impl for<'attachment> FnOnce(&mut AttachGuard<'attachment>) -> Result<String, Error>,
	) -> Result<String, Error> {
		let ctx = ndk_context::android_context();
		let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }?;
		let mut env = vm.attach_current_thread()?;

		callback(&mut env)
	}

	fn clipboard_manager<'attachment>(
		&self,
		env: &mut AttachGuard<'attachment>,
	) -> Result<JObject<'attachment>, Error> {
		let ctx = ndk_context::android_context();
		let context = unsafe { JObject::from_raw(ctx.context().cast()) };
		let clipboard = env.new_string("clipboard")?;

		Ok(env
			.call_method(
				context,
				"getSystemService",
				"(Ljava/lang/String;)Ljava/lang/Object;",
				&[(&clipboard).into()],
			)?
			.l()?)
	}
}

pub(crate) struct Get<'clipboard> {
	clipboard: &'clipboard Clipboard,
}

impl<'clipboard> Get<'clipboard> {
	pub(crate) fn new(clipboard: &'clipboard mut Clipboard) -> Self {
		Self { clipboard }
	}

	pub(crate) fn text(self) -> Result<String, Error> {
		Clipboard::with_android_context(|env| {
			let clipboard_manager = self.clipboard.clipboard_manager(env)?;

			if !env.call_method(&clipboard_manager, "hasPrimaryClip", "()Z", &[])?.z()? {
				return Err(Error::ContentNotAvailable);
			}

			let clip = env
				.call_method(
					clipboard_manager,
					"getPrimaryClip",
					"()Landroid/content/ClipData;",
					&[],
				)?
				.l()?;

			if env.call_method(&clip, "getItemCount", "()I", &[])?.i()? == 0 {
				return Err(Error::ContentNotAvailable);
			}

			let item = env
				.call_method(&clip, "getItemAt", "(I)Landroid/content/ClipData$Item;", &[0.into()])?
				.l()?;

			let text = env.call_method(item, "getText", "()Ljava/lang/CharSequence;", &[])?.l()?;
			let text = JString::from(text);
			let text = env.get_string(&text)?;
			Ok(text.into())
		})
	}

	pub(crate) fn html(self) -> Result<String, Error> {
		Err(Error::ClipboardNotSupported)
	}

	pub(crate) fn image(self) -> Result<ImageData<'static>, Error> {
		Err(Error::ClipboardNotSupported)
	}

	pub(crate) fn file_list(self) -> Result<Vec<PathBuf>, Error> {
		Err(Error::ClipboardNotSupported)
	}
}

pub(crate) struct Set<'clipboard> {
	clipboard: &'clipboard mut Clipboard,
}

impl<'clipboard> Set<'clipboard> {
	pub(crate) fn new(clipboard: &'clipboard mut Clipboard) -> Self {
		Self { clipboard }
	}

	pub(crate) fn text(self, text: Cow<'_, str>) -> Result<(), Error> {
		Err(Error::ClipboardNotSupported)
	}

	pub(crate) fn html(self, _: Cow<'_, str>, _: Option<Cow<'_, str>>) -> Result<(), Error> {
		Err(Error::ClipboardNotSupported)
	}

	#[cfg(feature = "image-data")]
	pub(crate) fn image(self, _: ImageData) -> Result<(), Error> {
		Err(Error::ClipboardNotSupported)
	}
}

pub(crate) struct Clear<'clipboard> {
	clipboard: &'clipboard mut Clipboard,
}

impl<'clipboard> Clear<'clipboard> {
	pub(crate) fn new(clipboard: &'clipboard mut Clipboard) -> Self {
		Self { clipboard }
	}

	pub(crate) fn clear(self) -> Result<(), Error> {
		/*let vm = self.clipboard.vm()?;
		let mut env = vm.attach_current_thread()?;
		let clipboard_manager = self.clipboard.clipboard_manager(&mut env)?;

		env.call_method(clipboard_manager, "clearPrimaryClip", "()V", &[])?;

		Ok(())*/
		Err(Error::ClipboardNotSupported)
	}
}
