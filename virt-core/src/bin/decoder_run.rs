use virt_core::decoder::decode;
use virt_core::widget::Widget;

fn main() {
    let widget_config = decode("C:/Users/dillb/Documents/Rust_Projects/virt/virt-core/src/bin/widget_files/all_widgets.toml").unwrap();

    println!("widget config: {:#?}", widget_config);

    let widget = Widget::new(widget_config).unwrap();

    println!("widget: {:#?}", widget)
}