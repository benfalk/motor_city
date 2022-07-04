# frozen_string_literal: true

module MotorCity
  module FFI
    # Result Pattern Object
    #
    # This represents either a succcess or failure from the Rust
    # code.  If the status is zero then the result is a success
    # and the value is the pointer to the value you want.  When
    # the status is anything but zero it is considered a failure
    # and the value is a string pointer detailing what went wrong.
    #
    class Result < ::FFI::Struct
      layout :status, :uint32,
             :value,  :pointer

      # Unwrap Result Pointer
      #
      # @param result_ptr [::FFI::Pointer]
      # @return [::FFI::Pointer]
      def self.unwrap_pointer!(result_ptr)
        result = new(result_ptr)
        raise Error, result[:value].read_string unless result[:status].zero?

        result[:value]
      ensure
        FFI.free_result(result_ptr)
      end

      # Unwrap Result to Type
      #
      # @param result_ptr [::FFI::Pointer]
      # @param type [Type,Symbol]
      # @return [Object]
      def self.unwrap!(result_ptr, type)
        ptr = unwrap_pointer!(result_ptr)
        type.new(ptr)
      end
    end
  end
end
