use druid::widget::{Button, Checkbox, Flex, Label, List, TextBox, ViewSwitcher};
use druid::{
    im, AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, Handled, Lens, PlatformError,
    Selector, Target, Widget, WidgetExt, WindowDesc,
};
use uuid::Uuid;

#[derive(Debug, Clone, Data, Lens)]
struct TodoItem {
    done: bool,
    currently_edited: bool,
    text: String,
    #[data(same_fn = "PartialEq::eq")]
    id: Uuid,
}

fn build_todo_item() -> impl Widget<TodoItem> {
    Flex::row()
        .with_child(Checkbox::new("").lens(TodoItem::done))
        .with_child(ViewSwitcher::new(
            |data: &TodoItem, _| data.currently_edited,
            |edited, _, _| {
                if *edited {
                    Box::new(TextBox::new().lens(TodoItem::text))
                } else {
                    Box::new(Label::dynamic(|data: &String, _| data.clone()).lens(TodoItem::text))
                }
            },
        ))
        .with_child(ViewSwitcher::new(
            |data: &TodoItem, _| data.currently_edited,
            |edited, _, _| {
                let button = if *edited {
                    Button::new("Save")
                } else {
                    Button::new("Edit")
                };
                let button = button.on_click(|_, data: &mut TodoItem, _| {
                    data.currently_edited = !data.currently_edited
                });
                Box::new(button)
            },
        ))
        .with_child(
            Button::new("Delete")
                .on_click(|ctx, data: &mut TodoItem, _| ctx.submit_command(DELETE.with(data.id))),
        )
}

#[derive(Debug, Clone, Data, Lens)]
struct AppData {
    list: im::Vector<TodoItem>,
    added_text: String,
}

impl AppData {
    fn add_todo(&mut self) {
        self.list.push_back(TodoItem {
            currently_edited: false,
            done: false,
            text: self.added_text.clone(),
            id: uuid::Uuid::new_v4(),
        });
        self.added_text = "".into();
    }

    fn delete_todo(&mut self, uuid: &Uuid) {
        let mut id: usize = 0;
        for (idx, i) in self.list.iter().enumerate() {
            if i.id == *uuid {
                id = idx;
                break;
            }
        }
        self.list.remove(id);
    }
}

fn build_ui() -> impl Widget<AppData> {
    Flex::column()
        .with_child(List::new(build_todo_item).lens(AppData::list))
        .with_child(
            Flex::row()
                .with_child(
                    TextBox::new()
                        .with_placeholder("Todo Text")
                        .lens(AppData::added_text),
                )
                .with_child(
                    Button::new("Add Todo").on_click(|_, data: &mut AppData, _| data.add_todo()),
                ),
        )
}

struct Delegate;

const DELETE: Selector<Uuid> = Selector::new("todo.delete");

impl AppDelegate<AppData> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppData,
        _env: &Env,
    ) -> Handled {
        if let Some(id) = cmd.get(DELETE) {
            data.delete_todo(id);
            return Handled::Yes;
        }
        Handled::No
    }
}

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(WindowDesc::new(build_ui))
        .delegate(Delegate)
        .launch(AppData {
            added_text: "".into(),
            list: im::Vector::new(),
        })?;
    Ok(())
}
