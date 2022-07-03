# frozen_string_literal: true

module MotorCity
  # Database Post
  class Post < ::FFI::ManagedStruct
    layout :id,        :int32,
           :title,     :string,
           :body,      :string,
           :published, :bool

    # @return [nil]
    def self.release(ptr)
      FFI.free_post(ptr)
    end

    # @return [MotorCity::Post, nil]
    def self.find(id)
      pointer = FFI.find_post_with_pool(id, MotorCity.connection)
      return if pointer.null?

      new(pointer)
    end
  end
end
