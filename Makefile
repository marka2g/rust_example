default:
	cd rust; cargo build --release
	cp rust/target/release/librust_example-*.a ruby/ext/rust_example/librust_example.a
	cd ruby; bundle
	cd ruby/ext/rust_example; ruby extconf.rb; make clean; make
	cd ruby; bundle exec rake test_fibers
