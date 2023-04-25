## 초기설정
시작전에 GTK버전을 확인한다
pkg-config --modversion gtk4

버전에 맞는 크레이트를 추가한다
cargo add gtk4 --rename gtk --features v4_8



## GTK 위젯
[위젯리스트](https://docs.gtk.org/gtk4/visual_index.html)
위젯문서에 impl문 속성들을 구현해야한다
<pre><code>class Gtk.Button : Gtk.Widget
  implements Gtk.Accessible, Gtk.Actionable, Gtk.Buildable, Gtk.ConstraintTarget {
  /* No available fields */
}
</code></pre>
[구현하는 특성들은 rs문서(메서드)](https://gtk-rs.org/gtk4-rs/git/docs/gtk4/struct.Button.html#implements)


   클로저가 같은 값을 참조하는 경우 Rc cell을 쓴다
<pre><code>let number = Rc::new(Cell::new(0));
</code></pre>
#### 위젯을 모아주는 BOX쓰는법
<pre><code>let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);</code></pre>
BOX는 강한참조로 UI를 고정시켜주고 아래는 약한 참조(glib::clone)로 UI바꿔줌 다른반환값일 경우 @default-return
<pre><code>button_increase.connect_clicked(clone!(@weak number, @weak button_decrease =>
        move |_| {
            number.set(number.get() + 1);
            button_decrease.set_label(&number.get().to_string());
    }));
    button_decrease.connect_clicked(clone!(@weak button_increase =>
        move |_| {
            number.set(number.get() - 1);
            button_increase.set_label(&number.get().to_string());
    }));</code></pre>
#### 모듈화 및 컴포넌트 속성재정의시
   mod는 공개인터페이스, 아래는 사이트의 class 구조와 impl명시
<pre><code>glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}</code></pre>

imp.rs 에서 구성하고 메소드 및 위젯을 구성하고 <br/><br/>
   main.rs에서 아래와 같이 호출
<pre><code>let button = CustomButton::new();</code></pre><br/>
[GTK 위젯 매소드들](https://docs.gtk.org/gtk4/class.Window.html#methods)<br/><br/>
## 데이터 다루는 방법<br/>
아래형식으로 치환해서 사용
<pre><code>enum Value <T> {
    bool(bool),
    i8(i8),
    i32(i32),
    u32(u32),
    i64(i64),
    u64(u64),
    f32(f32),
    f64(f64),
    // boxed types
    String(Option<String>),
    Object(Option<dyn IsA<glib::Object>>),
}</code></pre><br/>

```rust
use gtk::prelude::*;
//일반 숫자타입
let integer_value = 10.to_value(); //gtk::prelude::to_value
let integer = integer_value
	.get::<i32>()<br/>
	.expect("The value needs to be of type `i32`.");
//일반 문자열타입, option타입 Some, None 정의
let string_some_value = "Hello!".to_value(); //gtk::prelude::to_value::to_value
let string_none_value = None::<String>.to_value();
let string_some = string_some_value
	.get::<Option<String>>()
	.expect("The value needs to be of type `Option<String>`.");
let string_none = string_none_value
	.get::<Option<String>>()
	.expect("The value needs to be of type `Option<String>`.");
```
        
[타입별 값다루기](https://gtk-rs.org/gtk4-rs/git/book/g_object_values.html)
<br/>
또는 Variant :데이터를 직렬화해야 할 때마다 사용(예: 데이터를 다른 프로세스로 보내거나 네트워크를 통해 전송하거나 디스크에 저장하기 위해) 설정을 저장 gio::Settings하거나 작업을 활성화할 때 사용 gio::Action
```rust
use gtk::prelude::*;
    //숫자값
    let integer_variant = 10.to_variant();
    let integer = integer_variant
        .get::<i32>()
        .expect("The variant needs to be of type `i32`.");

    // 문자열값
    let string_some_value = "Hello!".to_value();
    let string_none_value = None::<String>.to_value();
    let string_some = string_some_value
        .get::<Option<String>>()
        .expect("The value needs to be of type `Option<String>`.");
    let string_none = string_none_value
        .get::<Option<String>>()
        .expect("The value needs to be of type `Option<String>`.");

    // 백터값
    let variant = vec!["Hello", "there!"].to_variant();
    let vec = &variant
        .get::<Vec<String>>()
        .expect("The variant needs to be of type `String`.");
```
[VariantDict](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/struct.VariantDict.html)
<br/>
<br/>
### 위젯 상태에 대한 속성공유등 set, get, binding, custom<br/>
state와 property를 통해서 설정한다, custom 시 외부 라이브러리 필요할 수 있음 cargo add once_cell의 sysnc lazy
[사이트](https://gtk-rs.org/gtk4-rs/git/book/g_object_properties.html)<br/><br/>

## 신호, 콜백, 클로저생성을 연결하는 방법<br/>

버튼 같은 경우 connect_clicked("clicked"신호발생)<br/>
connect_closure(사용자 지정 신호 지정가능)  <br/>
```rust
    // 기본적인 Clicked 신호발생
    button.connect_clicked(|button| {
        button.set_label("Hello World!");
    });
    // 사용자지정타입으로 "clicked"를 명시한 타입
    button.connect_closure(
        "clicked",
        false,
        closure_local!(move |button: Button| {
            button.set_label("Hello World!");
        }),
    );

```
사용자정의 imp재정의방법 (once_cell::sync::Lazy;)<br/>
```rust
//아래와 같이 신호이름에 대해 명시
impl ObjectImpl for CustomButton {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("max-number-reached")
                .param_types([i32::static_type()])
                .build()]
        });
        SIGNALS.as_ref()
    }
//클로저 등록방식
    button.connect_closure(
        "max-number-reached",
        false,
        closure_local!(move |_button: CustomButton, number: i32| {
            println!("The maximum number {} has been reached", number);
        }),
    );
```
glib::closure_local! : 강한/약한 참조를 생성하기 위한 동일한 구문과 관찰된 객체가 드롭되면 클로저를 자동으로 연결 해제하는 감시 기능을 허용
<br/><br/>
## 이벤트 루프, 응답제어 방식<br/>
일반 클로저생성/작업이 오래걸릴 경우 GUI가 동작을 멈추는데 두가지 방식을 써야함 클로저안에 클로저생성(새로운 스레드), 채널생성 (동기식으로 구현 )해서 추가적인 스레드생성 방지<br/><br/>
### 병행처리 - 멀티스레드<br/>
비동기적반응(새로운 스레드)<br/>
```rust
use std::thread;
use glib::{clone, Continue, MainContext, PRIORITY_DEFAULT};

    button.connect_clicked(move |_| {
        // The long running operation runs now in a separate thread
        thread::spawn(move || {
            let five_seconds = Duration::from_secs(5);
            thread::sleep(five_seconds);
        });
    });
```
동기적반응등록(채널생성)<br/>
```rust
// 채널생성
    let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
    button.connect_clicked(move |_| {
// sender
        let sender = sender.clone();
        thread::spawn(move || {
            sender.send(false).expect("Could not send through channel");
// 작업내용
            let ten_seconds = Duration::from_secs(10);
            thread::sleep(ten_seconds);
            // Activate the button again
            sender.send(true).expect("Could not send through channel");
        });
    });

// receiver 대기중인 상태의 설정
    receiver.attach(
        None,
        clone!(@weak button => @default-return Continue(false),
                    move |enable_button| {
                        button.set_sensitive(enable_button);
                        Continue(true)
                    }
        ),
    );
```
### 단일처리 단일스레드<br/>
async await 로  작성
```rust
    button.connect_clicked(move |button| {
        let main_context = MainContext::default();
        main_context.spawn_local(clone!(@weak button => async move {
            button.set_sensitive(false);
            timeout_future_seconds(5).await;
            button.set_sensitive(true);
        }));
    });
```
<br/><br/>
## 시스템에 컴포넌트 상태를 기록하고 호출<br/>
### 기본셋팅<br/>
Filename : org.gtk_rs.DBManager.gschema.xml<br/>
Path : Proejct/src/<br/>
```xml
<?xml version="1.0" encoding="utf-8"?>
<schemalist>
  <schema id="org.gtk_rs.DBManager" path="/org/gtk_rs/DBManager/">
    <key name="is-switch-enabled" type="b">
      <default>false</default>
      <summary>Default switch state</summary>
    </key>
  </schema>
</schemalist>
```
build한 Application 의 프로세스명과 맞춰준다<br/>
type="b"는 boolean 타입을 명시 [기타타입의 명세](https://docs.gtk.org/glib/gvariant-format-strings.html), [false 외의 값(Gvariant)명세](https://docs.gtk.org/glib/gvariant-text.html)<br/>

시스템에서 쓰기 위한 스키마 컴파일필요
```bash
mkdir -p $HOME/.local/share/glib-2.0/schemas
cp org.gtk_rs.Settings1.gschema.xml $HOME/.local/share/glib-2.0/schemas/
glib-compile-schemas $HOME/.local/share/glib-2.0/schemas/

```

### 구현부(gtk::gio를 불러오고  SettingsBindFlags, Settings사용)<br/>
#### 클로저를 이용한 방법 Settings
```rust
use gio::Settings;
use gtk::gio;
use gtk::glib::signal::Inhibit;

const APP_ID: &str = "org.gtk_rs.DBManager";
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

fn build_ui(app: &Application) {
    let settings = Settings::new(APP_ID);
    // 스키마의 key
    let is_switch_enabled = settings.boolean("is-switch-enabled");

    //컴포넌트에서 값호출
    let switch = Switch::builder()
        .state(is_switch_enabled)
```
다음 클로저를 통해 상태컨트롤(Inhibit로 기본핸들러를 꺼준다)
```rust
    switch.connect_state_set(move |_, is_enabled| {
        settings
            .set_boolean("is-switch-enabled", is_enabled)
            .expect("Could not set setting.");
        Inhibit(false)
    });
```
#### Settings속성을 통해 바인딩하는 방법
```rust
use gio::{Settings, SettingsBindFlags};
use gtk::gio;

fn build_ui(app: &Application) {
    // Initialize settings
    let settings = Settings::new(APP_ID);
    let switch = Switch::builder()
    ....
        .build();

    settings
        .bind("is-switch-enabled", &switch, "state")
        .flags(SettingsBindFlags::DEFAULT)
        .build();
```
## 윈도우창 상태를 시스템에 저장 및 유지
#### 위와 같은 방식, Application 구조체를 재정의, 모듈화하고 (impl) 메인에서 모듈호출 및 .present();<br/>
#### 구조체를 정의하면서 crate : once_cell를 사용해서 카피하는 방식
Filename : org.gtk_rs.SavingWindowState.gschema.xml
```xml
<?xml version="1.0" encoding="utf-8"?>
<schemalist>
  <schema id="org.gtk_rs.SavingWindowState" path="/org/gtk_rs/SavingWindowState/">
    <key name="window-width" type="i">
      <default>-1</default>
      <summary>Default window width</summary>
    </key>
    <key name="window-height" type="i">
      <default>-1</default>
      <summary>Default window height</summary>
    </key>
    <key name="is-maximized" type="b">
      <default>false</default>
      <summary>Default window maximized behaviour</summary>
    </key>
  </schema>
</schemalist>
```
마찬가지로 스키마 컴파일<br/>
imp.rs 와 mod.rs는 [여기서](https://gtk-rs.org/gtk4-rs/git/book/saving_window_state.html)<br/><br/>
main.rs
```rust
mod custom_window;
use custom_window::Window;

const APP_ID: &str = "org.gtk_rs.SavingWindowState";
...
fn build_ui(app: &Application) {
    let window = Window::new(app);
    ....
    window.set_child(Some(&gtk_box));
    window.present();
```
마찬가지 APP_ID를 일치시켜준다<br/> 
다른점은 main에서 새로운 ApplicationWindow를 build하지 않고 모듈로 가져온 window를 build가 아닌 new로 생성한다는 것과 set_child()메서드가 기존 레퍼런스타입이 아닌 Option type을 받는다(Option<&impl IsA<Widget>>)
## 목록 위젯 및 스크롤윈도우
#### 일반 목록리스트(간단한 데이터시)
ListBox :  세로 일반리스트
FlowBox : 달력같은 버튼식 나열리스트
```rust
use gtk::{ListBox, Label, ScrolledWindow}

fn build_ui(app: &Application) {
    let list_box = ListBox::new();

    // 데이터 label입히기
    for number in 0..=100 {
        let label = Label::new(Some(&number.to_string()));
        list_box.append(&label);
    }
   // Scroll 윈도우빌드
    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) 
        .min_content_width(360)
        .child(&list_box)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_width(600)
        .default_height(300)
        .child(&scrolled_window)
        .build();
```
위같은 경우 데이터하나당 위젯을 생성하기에 리소스를 잡아먹는다. 데이터가 많은 경우  인터페이스에러가 생길 수 있는데 GTK에선 모델, 목록, 보기 방식으로 나눠서 처리하는 방식이있다.<br/>
기타 레이블클릭시 반응하는 방식, 데이터정렬, 리스트레이블정렬시 참고 [사이트](https://gtk-rs.org/gtk4-rs/git/book/list_widgets.html)

<br/><br/>[기타 rust doc 사이트](https://docs.rs/gtk/latest/gtk/)<br/>