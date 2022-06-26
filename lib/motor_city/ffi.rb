# frozen_string_literal: true

module MotorCity
  # Foreign Function Interface
  #
  # Calls native Rust functions to avoid as much
  # Ruby runtime penalty as possible
  #
  module FFI
    extend ::FFI::Library
    ffi_lib File.expand_path("libmotor_city.#{::FFI::Platform::LIBSUFFIX}", __dir__)
    attach_function :db_url, [], :string
    attach_function :connection_ok, [], :bool

    attach_function :find_post, [:int32], :pointer, blocking: false
    attach_function :free_post, [:pointer], :void
  end
end
