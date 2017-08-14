use cursive::align::HAlign;
use cursive::view::{Identifiable, ViewWrapper};
use cursive::views::{BoxView, IdView, LinearLayout, TextArea, TextView};
use time;

use common::Action;
use cursive_views::messages_view::MessagesView;
use error;
use utils;


pub struct Dialog {
    layout: LinearLayout,
}

impl Dialog {
    pub fn new() -> error::Result<Dialog> {
        let layout = LinearLayout::vertical()
            .child(Dialog::create_dialog_title())
            .child(Dialog::create_messages_display_area())
            .child(Dialog::create_status_bar()?)
            .child(Dialog::create_message_edit_area());

        Ok(Dialog {
            layout: layout,
        })
    }

    fn create_dialog_title() -> IdView<TextView> {
        let dialog_title = TextView::new("dialog title")
            .h_align(HAlign::Center)
            .with_id("dialog_title");

        dialog_title
    }

    fn create_messages_display_area() -> BoxView<IdView<MessagesView>> {
        BoxView::with_full_screen(MessagesView::new()
            .action(Action::Online {
                time: time::now(),
                username: "foo".to_owned(),
            })
            .action(Action::Offline {
                time: time::now(),
                username: "bar".to_owned(),
            })
            .action(Action::Message {
                time: time::now(),
                username: "deadbeef".to_owned(),
                text: "hello tg-tui from deadbeef".to_owned(),
            })
            .delimiter()
            .with_id("messages_view"))
    }

    fn create_status_bar() -> error::Result<IdView<TextView>> {
        let status_bar = TextView::new(utils::now()?).with_id("status_bar");

        Ok(status_bar)
    }

    fn create_message_edit_area() -> LinearLayout {
        let prompt = "prompt";
        let initial_message_text = "message text";

        let message_edit_area = LinearLayout::horizontal()
            .child(TextView::new(prompt))
            .child(BoxView::with_full_width(TextArea::new().content(initial_message_text)));

        message_edit_area
    }
}

impl ViewWrapper for Dialog {
    wrap_impl!(self.layout: LinearLayout);
}
