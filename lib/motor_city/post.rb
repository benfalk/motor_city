# frozen_string_literal: true

module MotorCity
  # Database Post
  class Post < ::FFI::ManagedStruct
    layout :id,        :int32,
           :title,     :string,
           :body,      :string,
           :published, :bool

    # @return [nil]
    def self.release(result_ptr)
      FFI.free_post(result_ptr)
    end

    # @return [MotorCity::Post, nil]
    def self.find(id)
      pointer = FFI.find_post_with_pool(id, MotorCity.connection)
      return if pointer.null?

      FFI::Result.unwrap!(pointer, self)
    end

    def self.all
      result_ptr = FFI.all_post_with_pool(MotorCity.connection)
      array_ptr = FFI::Result.unwrap_pointer!(result_ptr)
      FFI::Array.unwrap!(array_ptr, self)
    end
  end
end
