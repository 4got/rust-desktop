use druid::widget::{Button, Flex, Label, Padding};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, PlatformError, Widget, WindowDesc};
#[path = "lib/menu.rs"]
mod menu;
use menu::Menu;

struct Sizes {
    BASE: f64,
}
const SIZE: Sizes = Sizes { BASE: 10.0 };

#[derive(Clone, Data, Lens)]
struct RootState {
    menu: menu::State,
}
impl RootState {
    fn new() -> Self {
        Self {
            menu: menu::State::new(),
        }
    }
}
// #[derive(Clone, Data, Lens)]
// struct MenuState {
//     selectedItem: Option<MenuItem>,
// }
// #[derive(Clone, Data)]
// enum MenuItem {
//     New,
//     Load,
//     Save,
//     Quit,
// }
// impl PartialEq for MenuItem {
//     fn eq(&self, other: &Self) -> bool {
//         self.isbn == other.isbn
//     }
// }

fn build_menu() -> impl Widget<RootState> {
    let mut root = Flex::column();
    for menu_item in menu::Item::as_vec() {
        let link = Button::<RootState>::new(format!("{:?}", &menu_item)).on_click(
            move |_ctx, data, _env| {
                data.menu.selected_item = Some(menu_item);
            },
        );
        root.add_flex_child(link, SIZE.BASE);
    }
    root
}
fn build_root() -> impl Widget<RootState> {
    let mut root = Flex::column();
    let mut main = Flex::row();

    // menu
    main.add_flex_child(Padding::new(SIZE.BASE, build_menu()), SIZE.BASE);

    // content
    let label = Label::new(|data: &RootState, _env: &Env| {
        format!(
            "Selected menu item: {:?}!",
            data.menu.selected_item.as_ref().unwrap()
        )
    });
    let content = Flex::column().with_child(Padding::new(SIZE.BASE, label));
    main.add_flex_child(content, SIZE.BASE);

    root.add_flex_child(main, SIZE.BASE);
    root
}

fn main() -> Result<(), PlatformError> {
    let menu_title: LocalizedString<RootState> = LocalizedString::new("Menu");
    let menu_window = WindowDesc::new(build_root()).title(menu_title);
    AppLauncher::with_window(menu_window).launch(RootState::new())?;
    Ok(())
}

// fn main() -> Result<(), PlatformError> {
//     let main_window = WindowDesc::new(ui_builder());
//     let data = 0_u32;
//     AppLauncher::with_window(main_window)
//         .log_to_console()
//         .launch(data)
// }

// fn ui_builder() -> impl Widget<u32> {
//     // The label text will be computed dynamically based on the current locale and count
//     let text =
//         LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());
//     let label = Label::new(text).padding(5.0).center();
//     let button = Button::new("increment")
//         .on_click(|_ctx, data, _env| *data += 1)
//         .padding(5.0);

//     Flex::column().with_child(label).with_child(button)
// }
