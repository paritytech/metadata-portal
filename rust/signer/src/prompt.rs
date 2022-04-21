use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Select};
use qr_lib::path::{QrFileName, QrPath};

pub fn select_file(files_to_sign: &[QrPath]) -> usize {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose file to sign")
        .default(0)
        .items(
            &files_to_sign
                .iter()
                .map(|p| &p.file_name)
                .collect::<Vec<&QrFileName>>(),
        )
        .interact()
        .unwrap()
}

pub fn want_to_continue() -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Ready to scan signature QR?")
        .default(true)
        .interact()
        .unwrap()
}
