# frozen_string_literal: true

module MotorCity
  module FFI
    # Simple collection from Rust
    class Array < ::FFI::Struct
      layout :size,     :uint64,
             :capacity, :uint64,
             :items,    :pointer

      def self.unwrap!(pointer, type)
        array = new(pointer)

        array[:items]
          .read_array_of_type(
            ::FFI::Type::POINTER,
            :read_pointer,
            array[:size]
          )
          .map!(&type.method(:new))
      ensure
        FFI.free_array(pointer)
      end
    end
  end
end
