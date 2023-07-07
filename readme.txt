+ extension:
    + rust-analyzer: WSL only
    + error lens: local - global

+ cargo:
    + cargo install cargo-watch    

+ create new rust project
    + cargo new project-name
    + cd project-name
    + cargo check: chạy đầu tiên để ktr lỗi
    + cargo run: run project: not optimized
    + cargo build: build project: not optimized
    + cargo build --release: build project: optimized
    + thay tất cả = cargo watch -x check -x test -x run = nodemon
    + thay tất cả = cargo watch -x check -x run = nodemon

+ các biến trong rust là immutable
+ các biến trong rust là siêu cụ thể
    + integer loại gì, cấp phát bộ nhớ bao nhiêu
    + float cấp phát bao nhiêu, giá trị gán có phải là float
+ array trong rust thì lại cực kì keo
    + phải tạo khởi tạo đúng định dạng element + số lượng
    + nhưng array ko bị immutable
+ tuple thì chúng ta khai báo nhiêu thì dữ liệu lần lượt của nó phải như vậy
    + chúng ta có thể có nhiều kiểu dữ liệu trong 1 tuple
    + tương tự array thì tuple cx ko bị immutable
    + tuple thoải mái hơn array

+ naming convention:
    + https://doc.rust-lang.org/1.0.0/style/style/naming/README.html

+ string is immutable also
    + string phân biệt với character => str = str + str = ok nhưng str = str + chr = no ok

+ rust docs, some default fn
    + https://doc.rust-lang.org/std/string/struct.String.html
    + https://doc.rust-lang.org/std/vec/struct.Vec.html
    + https://doc.rust-lang.org/reference/expressions/loop-expr.html#iterator-loops
    

