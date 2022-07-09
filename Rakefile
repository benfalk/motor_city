# frozen_string_literal: true

require "ffi"
require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "rubocop/rake_task"

RuboCop::RakeTask.new

task :rust_build do
  `rustup run nightly cargo build --release`
  `mv -f ./target/release/libmotor_city.#{::FFI::Platform::LIBSUFFIX} ./lib/motor_city/`
end

task :rust_clean do
  `cargo clean`
  `rm -f ./lib/motor_city/libmotor_city.#{::FFI::Platform::LIBSUFFIX}`
end

task build: :rust_build
task spec: :rust_build

task clean: :rust_clean

task default: %i[spec rubocop]
