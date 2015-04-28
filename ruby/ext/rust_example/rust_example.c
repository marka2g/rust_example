#include<stdio.h>
#include<ruby.h>

extern char *hello_rust(void);
extern void  fill_slice(char *);
extern void  rust_example_init();

VALUE hello(void) {
    char *hello = hello_rust();
    printf("%s\n", hello);

    return Qnil;
}

VALUE init() {
    rust_example_init();

    return Qnil;
}

// https://github.com/ruby/ruby/blob/trunk/README.EXT#L682
void Init_rust_example(void) {
    VALUE rust_example = rb_define_module("RustExample");

    rb_define_singleton_method(rust_example, "init", init, 0);
    rb_define_singleton_method(rust_example, "hello", hello, 0);
}
