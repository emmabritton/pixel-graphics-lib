use pixels_graphics_lib::dialogs::save_file_dialog::SaveFileDialog;
use pixels_graphics_lib::dialogs::FileDialogResults;
use pixels_graphics_lib::ui::styles::UiStyle;

// This example is used a test to make the dialogs with controllers have been correctly written

fn main() {
    let _ = SaveFileDialog::<SR, SN>::new(None, None, 10, 10, &UiStyle::default().dialog);
}

#[derive(Clone, Debug, PartialEq)]
enum SR {}

#[derive(Clone, Debug, PartialEq)]
enum SN {}

impl FileDialogResults<SR> for SR {
    fn save_file_result(_: String) -> SR {
        todo!()
    }

    fn load_file_result(_: String) -> SR {
        todo!()
    }
}
