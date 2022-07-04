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
  end
end
