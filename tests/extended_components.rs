use lawn_ui::*;

#[test]
fn test_combobox() {
    let cb = Combobox::new("cb-1")
        .placeholder("Pick framework")
        .item(ComboboxItem::new("gpui", "GPUI"));
    let _ = cb;
}

#[test]
fn test_date_picker() {
    let dp = DatePicker::new("dp-1").selected_date(Some((2026, 10, 24)));
    let _ = dp;
}

#[test]
fn test_number_input() {
    let ni = NumberInput::new("ni-1", 10).min(0).max(100).step(5);
    let _ = ni;
}

#[test]
fn test_otp_input() {
    let otp = OtpInput::new("otp-1").length(4).value("1234");
    let _ = otp;
}

#[test]
fn test_toast() {
    let toast = Toast::new("t-1", "Saved successfully")
        .description("Your changes are live")
        .variant(ToastVariant::Success);
    let _ = toast;
}

#[test]
fn test_toggle() {
    let toggle = Toggle::new("tog-1").pressed(true).size(ToggleSize::Lg);
    let _ = toggle;
}

#[test]
fn test_tree() {
    let root = TreeNode::new("src", "src").child(TreeNode::new("lib.rs", "lib.rs"));
    let tree = Tree::new("tree-1").node(root);
    let _ = tree;
}
