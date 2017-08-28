use cursive::align::HAlign;
use cursive::direction::Direction;
use cursive::event::{Event, EventResult, Key};
use cursive::view::{Finder, Identifiable, View, ViewWrapper};
use cursive::views::{BoxView, IdView, LinearLayout, OnEventView, TextArea, TextView};

use action::Action;
use commands::{Command, CommandImpl, parse_command};
use cursive_views::messages_view::MessagesView;
use error::{self, ErrorKind};
use mode::Mode;
use utils;


pub struct Dialog {
    layout: LinearLayout,
    mode: Mode,
}

impl Dialog {
    pub fn new() -> error::Result<IdView<OnEventView<Dialog>>> {
        let layout = LinearLayout::vertical()
            .child(Dialog::create_dialog_title())
            .child(Dialog::create_messages_display_area())
            .child(Dialog::create_status_bar()?)
            .child(Dialog::create_message_edit_area())
            .child(Dialog::create_command_field());

        let dialog = Dialog {
            layout: layout,
            mode: Mode::Normal,
        };

        let evented_dialog = OnEventView::new(dialog)
            .on_pre_event_inner(Event::Char(':'), |dialog| {
                match dialog.mode {
                    Mode::Normal => {
                        dialog.find_id("command-field", |field: &mut IdView<OnEventView<TextArea>>| {
                            field.get_mut().with_view_mut(|f| {
                                f.on_event(Event::Char(':'));
                                f.take_focus(Direction::none());
                            })
                        });
                        dialog.mode = Mode::CommandLine;

                        Some(EventResult::Consumed(None))
                    },
                    _ => None,
                }
            })
            .on_pre_event_inner(Event::Char('i'), |dialog| {
                match dialog.mode {
                    Mode::Normal => {
                        dialog.mode = Mode::Insert;
                        Some(EventResult::Consumed(None))
                    },
                    Mode::Insert => {
                        dialog.find_id("message-edit", |edit: &mut IdView<OnEventView<TextArea>>| {
                            edit.on_event(Event::Char('i'));
                        });
                        Some(EventResult::Consumed(None))
                    },
                    Mode::CommandLine => {
                        dialog.find_id("command-field", |edit: &mut IdView<OnEventView<TextArea>>| {
                            edit.on_event(Event::Char('i'));
                        });
                        Some(EventResult::Consumed(None))
                    },
                }
            })
            .on_pre_event_inner(Event::Key(Key::Esc), |dialog| {
                match dialog.mode {
                    Mode::Normal => None,
                    Mode::Insert => {
                        dialog.mode = Mode::Normal;
                        Some(EventResult::Consumed(None))
                    },
                    Mode::CommandLine => {
                        dialog.mode = Mode::Normal;
                        Some(EventResult::Consumed(None))
                    },
                }
            });

        Ok(evented_dialog.with_id("dialog"))
    }

    fn create_dialog_title() -> IdView<TextView> {
        let dialog_title = TextView::new("dialog title")
            .h_align(HAlign::Center)
            .with_id("dialog_title");

        dialog_title
    }

    fn create_messages_display_area() -> BoxView<IdView<MessagesView>> {
        // TODO: fetch real messages
        BoxView::with_full_screen(MessagesView::new()
            .action(Action::online(
                "foo",
            ))
            .action(Action::offline(
                "bar",
            ))
            .action(Action::message(
                "deadbeef",
                "hello tg-tui from deadbeef",
            ))
            .action(Action::message(
                "",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, \
                 sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
                 Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris \
                 nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in \
                 reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla \
                 pariatur. Excepteur sint occaecat cupidatat non proident, sunt in \
                 culpa qui officia deserunt mollit anim id est laborum.",
            ))
            .delimiter()
            .with_id("messages_view"))
    }

    fn create_status_bar() -> error::Result<IdView<TextView>> {
        let status_bar = TextView::new(utils::local_strnow()).with_id("status-bar");

        Ok(status_bar)
    }

    fn create_message_edit_area() -> LinearLayout {
        let prompt = "prompt";
        let initial_message_text = "message text";

        let message_edit_area = OnEventView::new(TextArea::new().content(initial_message_text))
            .with_id("message-edit");

        let message_edit_area_wrapper = LinearLayout::horizontal()
            .child(TextView::new(prompt))
            .child(BoxView::with_full_width(message_edit_area));

        message_edit_area_wrapper
    }

    fn create_command_field() -> IdView<OnEventView<TextArea>> {
        let command_field = OnEventView::new(TextArea::new())
            .on_pre_event_inner(Event::Key(Key::Backspace), |v| {
                if v.get_content() == ":" {
                    v.find_id("dialog", |d: &mut OnEventView<Dialog>| {
                        d.with_view_mut(|d| {
                            d.mode = Mode::Normal;
                        });
                    });
                }

                v.on_event(Event::Key(Key::Backspace));

                Some(EventResult::Consumed(None))
            })
            .on_pre_event_inner(Event::Key(Key::Enter), |v| {
                match parse_command::<CommandImpl>(v.get_content()) {
                    Ok(command) => {
                        command.execute().unwrap();

                        // Clearing
                        while v.get_content() != "" {
                            v.on_event(Event::Key(Key::Backspace));
                        }
                    },
                    Err(error::Error(ErrorKind::UndefinedCommand(cmd), _)) => {
                        v.set_content(format!("Not a command: {}", cmd));
                        v.find_id("dialog", |d: &mut OnEventView<Dialog>| {
                            d.with_view_mut(|d| {
                                d.mode = Mode::Normal;
                            });
                        });
                    },
                    Err(_) => panic!("cannot handle this error in callback"),
                }

                Some(EventResult::Consumed(None))
            })
            .with_id("command-field");

        command_field
    }
}

impl ViewWrapper for Dialog {
    wrap_impl!(self.layout: LinearLayout);
}
