use arboard::{Clipboard, ClipboardItem, ImageData};

fn main() {
	env_logger::init();

	let mut ctx = Clipboard::new().unwrap();

	let text = "hello world";
	let html = "<b>hello</b> <i>world</i>!";
	let bytes = [255, 100, 100, 255, 100, 255, 100, 100, 100, 100, 255, 100, 0, 0, 0, 255];
	let image = ImageData { width: 2, height: 2, bytes: bytes.as_ref().into() };

	ctx.set_text(text).unwrap();
	ctx.set_html(html, None).unwrap();
	//ctx.set().image(image.clone()).unwrap();

	let items = ctx.get().all().unwrap();

	for item in items {
		match item {
			ClipboardItem::Text(data) => println!("Text added to clipboard was {text}, got {data}"),
			ClipboardItem::Html(data) => println!("Html added to clipboard was {html}, got {data}"),
			ClipboardItem::ImagePng(data) => {
				println!(
					"Image added to clipboard was :\n{:?} got :\n{:?}",
					image.bytes, data.bytes
				);
			}
		}
	}
}
