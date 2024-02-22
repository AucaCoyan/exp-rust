fn main() {
    Demo::new().unwrap().run().unwrap();
}

slint::slint! {
import { AboutSlint, Button, VerticalBox } from "std-widgets.slint";
export component Demo inherits Window {
    height: 580px;
    width: 580px;
    title: "Espanso";
    VerticalBox {
        alignment: start;
        Text {
            text: "Hello World!";
            font-size: 24px;
            horizontal-alignment: center;
        }
        AboutSlint {
            preferred-height: 150px;
        }
        HorizontalLayout { alignment: center; Button { text: "OK!"; } }
    }
}

}
