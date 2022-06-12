# frozen_string_literal: true

require "ffi"

module MotorCity
  # Foreign Function Interface
  #
  # Calls native Rust functions to avoid as much
  # Ruby runtime penalty as possible
  #
  module FFI
    extend ::FFI::Library
    ffi_lib File.expand_path("libmotor_city.#{::FFI::Platform::LIBSUFFIX}", __dir__)
    attach_function :add, %i[uint uint], :uint
    attach_function :db_url, [], :string
  end
end
