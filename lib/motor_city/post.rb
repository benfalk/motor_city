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

    # @return [MotorCity::Post]
    def self.find(id)
      new(FFI.find_post(id))
    end
  end
end
